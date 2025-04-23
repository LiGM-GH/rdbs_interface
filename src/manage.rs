//! This module contains Manage view and its messages
//! Here a user can choose which exact view he wants: either  ManagePub or ManageSub

use iced::{Alignment, Length, Task};
use tokio_postgres::Client;

use crate::helpers;

mod publication;
mod subscription;

pub trait AsClient: AsRef<Client> + Clone {}

impl<T: AsRef<Client> + Clone> AsClient for T {}

pub struct View<DB: AsClient> {
    view: ViewVariant<DB>,
    error: Option<&'static str>,
}

enum ViewVariant<DB: AsClient> {
    Choose(DB),
    Pub(publication::View<DB>),
    Sub(subscription::View<DB>),
}

impl<DB: AsClient> ViewVariant<DB> {
    fn get_db(&self) -> DB {
        match self {
            Self::Choose(db) => db.clone(),
            Self::Sub(view) => view.get_db(),
            Self::Pub(view) => view.get_db(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PubChosen,
    SubChosen,
    Back,
    PubMsg(publication::Message),
    SubMsg(subscription::Message),
    Error(&'static str),
}

impl<DB: AsClient> View<DB> {
    pub fn new(client: DB) -> Self {
        Self {
            view: ViewVariant::Choose(client),
            error: None,
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        match &self.view {
            ViewVariant::Choose(_) => self.choose_view(),
            ViewVariant::Pub(view) => view.view().map(Message::PubMsg),
            ViewVariant::Sub(view) => view.view().map(Message::SubMsg),
        }
    }

    fn choose_view(&self) -> iced::Element<'_, Message> {
        let header = iced::widget::row![
            iced::widget::button("Back").on_press(Message::Back),
        ];

        let main_view: iced::Element<_> =
            iced::widget::container(iced::widget::column![
                helpers::centered_row(iced::widget::text(
                    "OK, WHAT DO YOU DO? WHICH MANAGER ARE YOU?"
                )),
                helpers::centered_row2(
                    iced::widget::button("The Pub manager")
                        .on_press(Message::PubChosen),
                    iced::widget::button("The Sub manager")
                        .on_press(Message::SubChosen)
                )
            ])
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        iced::widget::column![header, main_view].into()
    }

    fn err(val: &'static str) -> iced::Task<Message> {
        Task::done(Message::Error(val))
    }

    pub fn update(&mut self, msg: Message) -> iced::Task<Message> {
        match msg {
            Message::PubChosen => {
                let ViewVariant::Choose(client) = &self.view else {
                    return Self::err(
                        "Couldn't change to publication: in an unintended state!",
                    );
                };

                self.view =
                    ViewVariant::Pub(publication::View::new(client.clone()));

                Task::none()
            }
            Message::SubChosen => {
                let ViewVariant::Choose(client) = &self.view else {
                    return Self::err(
                        "Couldn't change to publication: in an unintended state!",
                    );
                };

                self.view =
                    ViewVariant::Sub(subscription::View::new(client.clone()));

                Task::none()
            }
            Message::PubMsg(publication::Message::Back) => {
                let client = self.view.get_db();
                self.view = ViewVariant::Choose(client);
                Task::none()
            }
            Message::PubMsg(message) => {
                let ViewVariant::Pub(value) = &mut self.view else {
                    return Self::err(
                        "Couldn't manage publication: in an unintended state!",
                    );
                };

                value.update(message).map(Message::PubMsg)
            }
            Message::SubMsg(subscription::Message::Back) => {
                let client = self.view.get_db();
                self.view = ViewVariant::Choose(client);
                Task::none()
            }
            Message::SubMsg(message) => {
                let ViewVariant::Sub(value) = &mut self.view else {
                    return Self::err(
                        "Couldn't manage subscription: in an unintended state!",
                    );
                };

                value.update(message).map(Message::SubMsg)
            }
            Message::Error(val) => {
                self.error = Some(val);
                Task::none()
            }
            Message::Back => {
                Self::err("We should go back before this happened")
            }
        }
    }
}
