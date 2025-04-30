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
    sub_list: Arc<Vec<String>>,
    sub_curr: Option<String>,
    errmsg: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Subs(Arc<Vec<String>>),
    SubSelected(String),
    Error(&'static str),
    DisableSub,
    Back,
    Clear,
}

impl<DB: AsClient> Viewable for View<DB> {
    type Message = Message;

    fn back(&self) -> Message {
        Message::Back
    }

    fn view(&self) -> Element<'_, Message> {
        let options = self.sub_list.as_slice();

        let header = iced::widget::row![
            iced::widget::button("Back").on_press(Message::Back),
        ];

        log::trace!("pub_list: {:?}", self.sub_list);

        let pub_list =
            pick_list(options, self.sub_curr.clone(), Message::SubSelected);

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        let main_view: iced::Element<_> = column![
            vertical_space().height(Length::FillPortion(1)),
            centered_row(errmsg),
            pub_list,
            centered_row(button("Disable subscription").on_press_maybe(
                if self.sub_curr.is_some() {
                    Some(Message::DisableSub)
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
            Message::Subs(pubs) => {
                self.sub_list = pubs;
                Task::none()
            }
            Message::SubSelected(sub) => {
                self.sub_curr = Some(sub);
                Task::perform(Self::update_subs(self.client.clone()), |val| {
                    match val {
                        Ok(val) => Message::Subs(Arc::new(val)),
                        Err(err) => {
                            log::error!("{err}");
                            Message::Error("Couldn't update tables")
                        }
                    }
                })
            }
            Message::Back => Task::done(Message::Error(
                "This should have been propagated higher",
            )),
            Message::DisableSub => {
                let Some(sub) = self.sub_curr.clone() else {
                    return Task::done(Message::Error(
                        "No publication provided",
                    ));
                };

                let disable_task = Task::perform(
                    Self::disable_sub(self.client.clone(), sub),
                    |val| match val {
                        Ok(()) => Message::Clear,
                        Err(err) => {
                            log::error!("{err}");

                            Message::Error("Couldn't add table")
                        }
                    },
                );

                let update_task = Task::perform(
                    Self::update_subs(self.client.clone()),
                    |val| match val {
                        Ok(val) => Message::Subs(Arc::new(val)),
                        Err(err) => {
                            log::error!("{err}");
                            Message::Error("Couldn't update tables")
                        }
                    },
                );

                disable_task.chain(update_task)
            }
            Message::Clear => {
                self.sub_curr = None;
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
            Task::perform(Self::update_subs(client.clone()), |val| match val {
                Ok(val) => Message::Subs(Arc::new(val)),
                Err(err) => {
                    log::error!("{err}");
                    Message::Error("Error occurred while getting publications")
                }
            });

        (
            Self {
                client,
                errmsg: None,
                sub_list: Arc::new(Vec::new()),
                sub_curr: None,
            },
            pubs_task,
        )
    }

    async fn disable_sub(
        client: DB,
        sub: String,
    ) -> Result<(), tokio_postgres::Error> {
        client
            .as_ref()
            .execute(&format!("ALTER SUBSCRIPTION \"{}\" DISABLE;", sub), &[])
            .await
            .map(|_| ())
    }

    async fn update_subs(
        client: DB,
    ) -> Result<Vec<String>, tokio_postgres::Error> {
        let val = client
            .as_ref()
            .query(
                "SELECT subname FROM pg_subscription WHERE subenabled = 't';",
                &[],
            )
            .await?;

        let val = val
            .iter()
            .map(|val| val.get::<_, String>("subname"))
            .collect::<Vec<_>>();

        Ok(val)
    }
}
