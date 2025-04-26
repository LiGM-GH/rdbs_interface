use std::sync::Arc;

use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{button, column, pick_list, text, text_input, vertical_space},
};

use crate::{helpers::centered_row, manage::AsClient};

#[derive(Debug)]
pub struct View<DB: AsClient> {
    client: DB,
    sub_list: Arc<Vec<String>>,
    sub_curr: Option<String>,
    new_name: String,
    errmsg: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub enum Message {
    NewName(String),
    Subs(Arc<Vec<String>>),
    SubSelected(String),
    Error(&'static str),
    RenameSub,
    Back,
    Clear,
}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        self.client.clone()
    }

    pub fn new(client: DB) -> (Self, Task<Message>) {
        let subs_task =
            Task::perform(Self::update_subs(client.clone()), |val| match val {
                Ok(val) => Message::Subs(Arc::new(val)),
                Err(err) => {
                    log::error!("{err}");
                    Message::Error("Error occurred while getting subscriptions")
                }
            });

        (
            Self {
                client,
                errmsg: None,
                sub_list: Arc::new(Vec::new()),
                sub_curr: None,
                new_name: String::new(),
            },
            subs_task,
        )
    }

    pub fn view(&self) -> Element<'_, Message> {
        let options = self.sub_list.as_slice();

        let header = iced::widget::row![
            iced::widget::button("Back").on_press(Message::Back),
        ];

        log::trace!("sub_list: {:?}", self.sub_list);

        let sub_list =
            pick_list(options, self.sub_curr.clone(), Message::SubSelected);

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        let main_view: iced::Element<_> = column![
            vertical_space().height(Length::FillPortion(1)),
            centered_row(errmsg),
            sub_list,
            centered_row(
                text_input("New name", &self.new_name)
                    .on_input(Message::NewName)
            ),
            centered_row(button("Rename subscription").on_press_maybe(
                if self.new_name.is_empty() {
                    None
                } else {
                    Some(Message::RenameSub)
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
            Message::Subs(subs) => {
                self.sub_list = subs;
                Task::none()
            }
            Message::SubSelected(subscription) => {
                self.sub_curr = Some(subscription.clone());
                Task::none()
            }
            Message::Back => Task::done(Message::Error(
                "This should have been propagated higher",
            )),
            Message::RenameSub => {
                let Some(subscription) = self.sub_curr.clone() else {
                    return Task::done(Message::Error(
                        "No subscription provided",
                    ));
                };

                Task::perform(
                    Self::rename_subscription(
                        self.client.clone(),
                        subscription,
                        self.new_name.clone(),
                    ),
                    |val| match val {
                        Ok(()) => Message::Clear,
                        Err(err) => {
                            log::error!("{err}");

                            Message::Error("Couldn't rename subscription")
                        }
                    },
                )
            }
            Message::Clear => {
                self.sub_curr = None;
                self.errmsg = None;
                self.new_name = String::new();

                Task::perform(Self::update_subs(self.client.clone()), |val| {
                    match val {
                        Ok(val) => Message::Subs(Arc::new(val)),
                        Err(err) => {
                            log::error!("{err}");
                            Message::Error("Couldn't update subscriptions")
                        }
                    }
                })
            }
        }
    }

    async fn rename_subscription(
        client: DB,
        subscription: String,
        new_name: String,
    ) -> Result<(), tokio_postgres::Error> {
        let new_name = pg_escape::quote_identifier(&new_name);

        let query = &format!(
            "ALTER SUBSCRIPTION \"{}\" RENAME TO \"{}\";",
            subscription, new_name
        );

        client.as_ref().execute(query, &[]).await.map(|_| ())
    }

    async fn update_subs(
        client: DB,
    ) -> Result<Vec<String>, tokio_postgres::Error> {
        let val = client
            .as_ref()
            .query("SELECT subname FROM pg_subscription;", &[])
            .await?;

        let val = val
            .iter()
            .map(|val| val.get::<_, String>("subname"))
            .collect::<Vec<_>>();

        Ok(val)
    }
}
