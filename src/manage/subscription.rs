//! This module contains Subscription view and its messages

use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{column, text},
};

use crate::{
    helpers::centered_row,
    widgets::event_catcher::{enter_catcher, event_catcher},
};

use super::AsClient;

mod delete_subscription;

#[derive(Debug)]
pub struct View<DB: AsClient> {
    view: ViewVariant<DB>,
    errmsg: Option<&'static str>,
}

#[derive(Debug)]
enum ViewVariant<DB: AsClient> {
    Main(DB),
    CreateSubscription(delete_subscription::View<DB>),
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    CreateSubscription,
    CreateSub(delete_subscription::Message),
    Error(&'static str),
}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        match &self.view {
            ViewVariant::Main(client) => client.clone(),
            ViewVariant::CreateSubscription(view) => view.get_db(),
        }
    }

    fn err(msg: &'static str) -> Task<Message> {
        Task::done(Message::Error(msg))
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
                let header = iced::widget::row![event_catcher(
                    iced::widget::button("Back").on_press(Message::Back),
                    enter_catcher(Message::Back)
                )];

                let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
                    .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
                    .into();

                let main_view: iced::Element<_> = column![
                    iced::widget::vertical_space().width(Length::Fill),
                    centered_row(errmsg),
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
                    delete_subscription::View::new(db.clone()),
                );
                Task::none()
            }
            Message::CreateSub(msg) => {
                let ViewVariant::CreateSubscription(view) = &mut self.view
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
