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

mod disable_sub;
mod enable_sub;
mod rename_sub;

#[derive(Debug)]
pub struct View<DB: AsClient> {
    view: ViewVariant<DB>,
    errmsg: Option<&'static str>,
}

#[derive(Debug)]
enum ViewVariant<DB: AsClient> {
    Main(DB),
    EnableSub(enable_sub::View<DB>),
    DisableSub(disable_sub::View<DB>),
    RenameSub(rename_sub::View<DB>),
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    EnableSubscription,
    EnableSub(enable_sub::Message),
    DisableSubscription,
    DisableSub(disable_sub::Message),
    RenameSubscription,
    RenameSub(rename_sub::Message),
    Error(&'static str),
}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        match &self.view {
            ViewVariant::Main(client) => client.clone(),
            ViewVariant::EnableSub(view) => view.get_db(),
            ViewVariant::DisableSub(view) => view.get_db(),
            ViewVariant::RenameSub(view) => view.get_db(),
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
                        iced::widget::button("Enable subscription")
                            .on_press(Message::EnableSubscription)
                            .width(Length::FillPortion(8)),
                    ),
                    centered_row(
                        iced::widget::button("Disable subscription")
                            .on_press(Message::DisableSubscription)
                            .width(Length::FillPortion(8)),
                    ),
                    centered_row(
                        iced::widget::button("Rename subscription")
                            .on_press(Message::RenameSubscription)
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
            ViewVariant::EnableSub(ref view) => {
                view.view().map(Message::EnableSub)
            }
            ViewVariant::DisableSub(ref view) => {
                view.view().map(Message::DisableSub)
            }
            ViewVariant::RenameSub(ref view) => {
                view.view().map(Message::RenameSub)
            }
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Back => {
                Self::err("This should have been propagated higher")
            }
            Message::EnableSubscription => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };
                let (view, task) = enable_sub::View::new(db.clone());
                self.view = ViewVariant::EnableSub(view);

                task.map(Message::EnableSub)
            }
            Message::EnableSub(enable_sub::Message::Back) => {
                let ViewVariant::EnableSub(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                self.view = ViewVariant::Main(view.get_db().clone());
                Task::none()
            }
            Message::EnableSub(msg) => {
                let ViewVariant::EnableSub(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                view.update(msg).map(Message::EnableSub)
            }
            Message::DisableSubscription => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };
                let (view, task) = disable_sub::View::new(db.clone());
                self.view = ViewVariant::DisableSub(view);

                task.map(Message::DisableSub)
            }
            Message::DisableSub(disable_sub::Message::Back) => {
                let ViewVariant::DisableSub(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                self.view = ViewVariant::Main(view.get_db().clone());
                Task::none()
            }
            Message::DisableSub(msg) => {
                let ViewVariant::DisableSub(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                view.update(msg).map(Message::DisableSub)
            }
            Message::RenameSubscription => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };
                let (view, task) = rename_sub::View::new(db.clone());
                self.view = ViewVariant::RenameSub(view);

                task.map(Message::RenameSub)
            }
            Message::RenameSub(rename_sub::Message::Back) => {
                let ViewVariant::RenameSub(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                self.view = ViewVariant::Main(view.get_db().clone());
                Task::none()
            }
            Message::RenameSub(msg) => {
                let ViewVariant::RenameSub(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Couldn't create subscription",
                    ));
                };

                view.update(msg).map(Message::RenameSub)
            }
            Message::Error(msg) => {
                self.errmsg = Some(msg);
                Task::none()
            }
        }
    }
}
