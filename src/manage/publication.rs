//! This module contains Publication view and its messages

use iced::{Alignment, Length, Task, widget::column};

use crate::{helpers::centered_row, manage::AsClient};

mod add_table;

pub struct View<DB: AsClient> {
    view: ViewVariant<DB>,
    errmsg: Option<&'static str>,
}

enum ViewVariant<DB: AsClient> {
    Main(DB),
    CreateSubscription(add_table::View<DB>),
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    CreateSubscription,
    CreateSub(add_table::Message),
    Error(&'static str),
}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        match &self.view {
            ViewVariant::Main(client) => client.clone(),
            ViewVariant::CreateSubscription(view) => view.get_db(),
        }
    }

    pub fn new(client: DB) -> Self {
        Self {
            view: ViewVariant::Main(client),
            errmsg: None,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        match self.view {
            ViewVariant::Main(_) => {
                let header = iced::widget::row![
                    iced::widget::button("Back").on_press(Message::Back),
                ];

                let main_view: iced::Element<_> = column![
                    iced::widget::vertical_space().width(Length::Fill),
                    centered_row(
                        iced::widget::button("Create subscription")
                            .on_press(Message::CreateSubscription)
                            .width(Length::FillPortion(8)),
                    ),
                    iced::widget::vertical_space().width(Length::Fill),
                ]
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .into();

                iced::widget::column![header, main_view].into()
            }
            ViewVariant::CreateSubscription(ref sub) => {
                sub.view().map(Message::CreateSub)
            }
        }
    }

    fn err(msg: &'static str) -> Task<Message> {
        Task::done(Message::Error(msg))
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Back => {
                Self::err("This should have been propagated higher")
            }
            Message::CreateSubscription => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };

                self.view = ViewVariant::CreateSubscription(
                    add_table::View::new(db.clone()),
                );
                Task::none()
            }
            Message::CreateSub(msg) => {
                let ViewVariant::CreateSubscription(ref mut view) = self.view
                else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                view.update(msg).map(Message::CreateSub)
            }
            Message::Error(msg) => {
                self.errmsg = Some(msg);
                Task::none()
            }
        }
    }
}
