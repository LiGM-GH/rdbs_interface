use std::sync::Arc;

use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{button, column, pick_list, text, vertical_space},
};

use crate::{
    helpers::centered_row,
    manage::AsClient,
    traits::{GetDb, Viewable},
};

#[derive(Debug)]
pub struct View<DB: AsClient> {
    client: DB,
    table_list: Arc<Vec<String>>,
    table_curr: Option<String>,
    pub_list: Arc<Vec<String>>,
    pub_curr: Option<String>,
    errmsg: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Tables(Arc<Vec<String>>),
    TableSelected(String),
    Pubs(Arc<Vec<String>>),
    PubSelected(String),
    Error(&'static str),
    AddTable,
    Back,
    Clear,
}

impl<DB: AsClient> Viewable for View<DB> {
    type Message = Message;

    fn back(&self) -> Message {
        Message::Back
    }

    fn view(&self) -> Element<'_, Message> {
        let options = self.pub_list.as_slice();

        let header = iced::widget::row![
            iced::widget::button("Back").on_press(Message::Back),
        ];

        log::trace!("pub_list: {:?}", self.pub_list);

        let pub_list =
            pick_list(options, self.pub_curr.clone(), Message::PubSelected);

        let options = self.table_list.as_slice();

        let table_list =
            pick_list(options, self.table_curr.clone(), Message::TableSelected);

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        let main_view: iced::Element<_> = column![
            vertical_space().height(Length::FillPortion(1)),
            centered_row(errmsg),
            pub_list,
            table_list,
            centered_row(button("Add table").on_press_maybe(
                if self.table_curr.is_some() {
                    Some(Message::AddTable)
                } else {
                    None
                }
            )),
            vertical_space().height(Length::FillPortion(1)),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into();

        column![header, main_view].into()
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Error(err) => {
                self.errmsg = Some(err);
                Task::none()
            }
            Message::Tables(tables) => {
                self.table_list = tables;
                Task::none()
            }
            Message::TableSelected(table) => {
                self.table_curr = Some(table);
                Task::none()
            }
            Message::Pubs(pubs) => {
                self.pub_list = pubs;
                Task::none()
            }
            Message::PubSelected(publication) => {
                self.pub_curr = Some(publication.clone());
                Task::perform(
                    Self::update_tables(self.client.clone(), publication),
                    |val| match val {
                        Ok(val) => Message::Tables(Arc::new(val)),
                        Err(err) => {
                            log::error!("{err}");
                            Message::Error("Couldn't update tables")
                        }
                    },
                )
            }
            Message::Back => Task::done(Message::Error(
                "This should have been propagated higher",
            )),
            Message::AddTable => {
                let Some(publication) = self.pub_curr.clone() else {
                    return Task::done(Message::Error(
                        "No publication provided",
                    ));
                };

                let Some(table) = self.table_curr.clone() else {
                    return Task::done(Message::Error("No table provided"));
                };

                let add_task = Task::perform(
                    Self::add_table(
                        self.client.clone(),
                        publication.clone(),
                        table,
                    ),
                    |val| match val {
                        Ok(()) => Message::Clear,
                        Err(err) => {
                            log::error!("{err}");

                            Message::Error("Couldn't add table")
                        }
                    },
                );

                let update_task = Task::perform(
                    Self::update_tables(self.client.clone(), publication),
                    |val| match val {
                        Ok(val) => Message::Tables(Arc::new(val)),
                        Err(err) => {
                            log::error!("{err}");
                            Message::Error("Couldn't update tables")
                        }
                    },
                );

                add_task.chain(update_task)
            }
            Message::Clear => {
                self.table_curr = None;
                self.pub_curr = None;
                self.table_list = Arc::new(Vec::new());
                self.errmsg = None;

                Task::none()
            }
        }
    }
}

impl<DB: AsClient> GetDb for View<DB> {
    type DB = DB;
    fn get_db(&self) -> DB {
        self.client.clone()
    }
}

impl<DB: AsClient> View<DB> {
    pub fn new(client: DB) -> (Self, Task<Message>) {
        let pubs_task =
            Task::perform(Self::update_pubs(client.clone()), |val| match val {
                Ok(val) => Message::Pubs(Arc::new(val)),
                Err(err) => {
                    log::error!("{err}");
                    Message::Error("Error occurred while getting publications")
                }
            });

        (
            Self {
                client,
                table_list: Arc::new(Vec::new()),
                table_curr: None,
                errmsg: None,
                pub_list: Arc::new(Vec::new()),
                pub_curr: None,
            },
            pubs_task,
        )
    }

    async fn add_table(
        client: DB,
        publication: String,
        table: String,
    ) -> Result<(), tokio_postgres::Error> {
        client
            .as_ref()
            .execute(
                &format!(
                    "ALTER PUBLICATION \"{}\" ADD TABLE public.\"{}\";",
                    publication, table
                ),
                &[],
            )
            .await
            .map(|_| ())
    }

    async fn update_tables(
        client: DB,
        publication: String,
    ) -> Result<Vec<String>, tokio_postgres::Error> {
        let val = client
            .as_ref()
            .query(
                "SELECT tablename FROM pg_tables WHERE schemaname = 'public' EXCEPT (SELECT tablename FROM pg_publication_tables WHERE schemaname = 'public' AND pubname = $1);",
                &[&publication],
            )
            .await?;

        let val = val
            .iter()
            .map(|val| val.get::<_, String>("tablename"))
            .collect::<Vec<_>>();

        log::trace!("These are table names: {val:?}");

        Ok(val)
    }

    async fn update_pubs(
        client: DB,
    ) -> Result<Vec<String>, tokio_postgres::Error> {
        let val = client
            .as_ref()
            .query("SELECT pubname FROM pg_publication;", &[])
            .await?;

        let val = val
            .iter()
            .map(|val| val.get::<_, String>("pubname"))
            .collect::<Vec<_>>();

        Ok(val)
    }
}
