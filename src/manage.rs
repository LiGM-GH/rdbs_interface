//! This module contains Manage view and its messages
//! Here a user can choose which exact view he wants: either  `ManagePub` or `ManageSub`

use iced::{Alignment, Color, Element, Length, Task, widget::text};

use crate::{
    helpers,
    traits::{AsClient, GetDb, Viewable},
};

mod publication;
mod subscription;

#[derive(Debug)]
pub struct View<DB: AsClient> {
    pub view: ViewVariant<DB>,
    errmsg: Option<&'static str>,
}

#[derive(Debug)]
pub enum ViewVariant<DB: AsClient> {
    Choose(DB),
    Pub(publication::View<DB>),
    Sub(subscription::View<DB>),
}

impl<DB: AsClient> ViewVariant<DB> {}

#[derive(Clone, Debug)]
pub enum Message {
    PubChosen,
    SubChosen,
    Back,
    PubMsg(publication::Message),
    SubMsg(subscription::Message),
    Error(&'static str),
}

impl<DB: AsClient> Viewable for View<DB> {
    type Message = Message;
    fn back(&self) -> Message {
        match &self.view {
            ViewVariant::Pub(view) => Message::PubMsg(view.back()),
            ViewVariant::Sub(view) => Message::SubMsg(view.back()),
            ViewVariant::Choose(_) => Message::Back,
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        match &self.view {
            ViewVariant::Choose(_) => self.choose_view(),
            ViewVariant::Pub(view) => view.view().map(Message::PubMsg),
            ViewVariant::Sub(view) => view.view().map(Message::SubMsg),
        }
    }

    fn update(&mut self, msg: Message) -> iced::Task<Message> {
        match msg {
            Message::PubMsg(publication::Message::Back)
            | Message::SubMsg(subscription::Message::Back) => {
                let client = self.get_db();
                self.view = ViewVariant::Choose(client);
                Task::none()
            }
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
            Message::PubMsg(message) => {
                let ViewVariant::Pub(value) = &mut self.view else {
                    return Self::err(
                        "Couldn't manage publication: in an unintended state!",
                    );
                };

                value.update(message).map(Message::PubMsg)
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
                self.errmsg = Some(val);
                Task::none()
            }
            Message::Back => {
                Self::err("We should go back before this happened")
            }
        }
    }
}

impl<DB: AsClient> GetDb for View<DB> {
    type DB = DB;
    fn get_db(&self) -> DB {
        match &self.view {
            ViewVariant::Choose(db) => db.clone(),
            ViewVariant::Sub(view) => view.get_db(),
            ViewVariant::Pub(view) => view.get_db(),
        }
    }
}

impl<DB: AsClient> View<DB> {
    pub const fn new(client: DB) -> Self {
        Self {
            view: ViewVariant::Choose(client),
            errmsg: None,
        }
    }

    fn choose_view(&self) -> iced::Element<'_, Message> {
        let header = iced::widget::row![
            iced::widget::button("Back").on_press(Message::Back),
        ];

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        let main_view: iced::Element<_> =
            iced::widget::container(iced::widget::column![
                helpers::centered_row(errmsg),
                helpers::centered_row(iced::widget::text(
                    "OK, WHAT DO YOU DO? WHICH MANAGER ARE YOU?"
                )),
                helpers::centered_row2(
                    iced::widget::button("The Pub manager")
                        .on_press(Message::PubChosen),
                    iced::widget::button("The Sub manager")
                        .on_press(Message::SubChosen),
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
}
