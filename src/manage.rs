//! This module contains Manage view and its messages

use iced::{Alignment, Length, Task, widget::column};
use tokio_postgres::Client;

use crate::helpers::centered_row;

mod create_subscription;

pub struct View<DB: AsRef<Client>> {
    view: ViewVariant<DB>,
    errmsg: Option<&'static str>,
}

#[derive(Default)]
enum ViewVariant<DB: AsRef<Client>> {
    #[default]
    Default,
    Main(DB),
    CreateSubscription(create_subscription::View<DB>),
}

#[derive(Clone, Debug)]
pub enum Message {
    CreateSubscription,
    CreateSub(create_subscription::Message),
    Error(&'static str),
}

impl<DB: AsRef<Client>> View<DB> {
    pub fn new(client: DB) -> Self {
        Self {
            view: ViewVariant::Main(client),
            errmsg: None,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        match self.view {
            ViewVariant::Main(_) => column![
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
            .into(),
            ViewVariant::CreateSubscription(ref sub) => {
                sub.view().map(Message::CreateSub)
            }
            ViewVariant::Default => unreachable!(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::CreateSubscription => {
                let ViewVariant::Main(db) = std::mem::take(&mut self.view) else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };
                self.view = ViewVariant::CreateSubscription(
                    create_subscription::View::new(db),
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
