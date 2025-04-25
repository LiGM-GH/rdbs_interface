use std::sync::Arc;

use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{button, column, pick_list, text, text_input, vertical_space},
};
use tokio_postgres::types::Type;

use crate::{helpers::centered_row, manage::AsClient};

#[derive(Debug)]
pub struct View<DB: AsClient> {
    client: DB,
    pub_list: Arc<Vec<String>>,
    pub_curr: Option<String>,
    new_name: String,
    errmsg: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Message {
    NewName(String),
    Pubs(Arc<Vec<String>>),
    PubSelected(String),
    Error(&'static str),
    RenamePub,
    Back,
    Clear,
}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        self.client.clone()
    }

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
                errmsg: None,
                pub_list: Arc::new(Vec::new()),
                pub_curr: None,
                new_name: String::new(),
            },
            pubs_task,
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let options = self.pub_list.as_slice();

        let header = iced::widget::row![
            iced::widget::button("Back").on_press(Message::Back),
        ];

        log::trace!("pub_list: {:?}", self.pub_list);

        let pub_list =
            pick_list(options, self.pub_curr.clone(), Message::PubSelected);

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        let main_view: iced::Element<_> = column![
            vertical_space().height(Length::FillPortion(1)),
            centered_row(errmsg),
            pub_list,
            centered_row(
                text_input("New name", &self.new_name)
                    .on_input(Message::NewName)
            ),
            centered_row(button("Rename publication").on_press_maybe(
                if self.new_name.is_empty() {
                    None
                } else {
                    Some(Message::RenamePub)
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

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Error(err) => {
                self.errmsg = Some(err);
                Task::none()
            }
            Message::NewName(name) => {
                self.new_name = name;
                Task::none()
            }
            Message::Pubs(pubs) => {
                self.pub_list = pubs;
                Task::none()
            }
            Message::PubSelected(publication) => {
                self.pub_curr = Some(publication.clone());
                Task::none()
            }
            Message::Back => Task::done(Message::Error(
                "This should have been propagated higher",
            )),
            Message::RenamePub => {
                let Some(publication) = self.pub_curr.clone() else {
                    return Task::done(Message::Error(
                        "No publication provided",
                    ));
                };

                let new_name = self.new_name.clone();

                Task::perform(
                    Self::rename_publication(
                        self.client.clone(),
                        publication,
                        new_name,
                    ),
                    |val| match val {
                        Ok(()) => Message::Clear,
                        Err(err) => {
                            log::error!("{err}");

                            Message::Error("Couldn't rename publication")
                        }
                    },
                )
            }
            Message::Clear => {
                self.pub_curr = None;
                self.errmsg = None;
                self.new_name = String::new();

                Task::perform(Self::update_pubs(self.client.clone()), |val| {
                    match val {
                        Ok(val) => Message::Pubs(Arc::new(val)),
                        Err(err) => {
                            log::error!("{err}");
                            Message::Error("Couldn't update publications")
                        }
                    }
                })
            }
        }
    }

    async fn rename_publication(
        client: DB,
        publication: String,
        new_name: String,
    ) -> Result<(), tokio_postgres::Error> {
        let new_name = pg_escape::quote_identifier(&new_name);

        let query = &format!(
            "ALTER PUBLICATION \"{}\" RENAME TO \"{}\";",
            publication, new_name
        );

        client.as_ref().execute(query, &[]).await.map(|_| ())
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
