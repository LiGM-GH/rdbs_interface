//! This module contains Publication view and its messages

use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{button, column, text},
};

use crate::{helpers::centered_row, manage::AsClient};

mod add_table;
mod delete_table;
mod rename_publication;

#[derive(Debug)]
pub struct View<DB: AsClient> {
    view: ViewVariant<DB>,
    errmsg: Option<&'static str>,
}

#[derive(Debug)]
enum ViewVariant<DB: AsClient> {
    Main(DB),
    AddTable(add_table::View<DB>),
    DeleteTable(delete_table::View<DB>),
    RenameTable(rename_publication::View<DB>),
}

#[derive(Clone, Debug)]
pub enum Message {
    Back,
    AddTable,
    AddTableMsg(add_table::Message),
    DeleteTable,
    DeleteTableMsg(delete_table::Message),
    RenameTable,
    RenameTableMsg(rename_publication::Message),
    Error(&'static str),
}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        match &self.view {
            ViewVariant::Main(client) => client.clone(),
            ViewVariant::AddTable(view) => view.get_db(),
            ViewVariant::DeleteTable(view) => view.get_db(),
            ViewVariant::RenameTable(view) => view.get_db(),
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
            ViewVariant::Main(_) => self.main_view(),
            ViewVariant::AddTable(ref view) => {
                view.view().map(Message::AddTableMsg)
            }
            ViewVariant::DeleteTable(ref view) => {
                view.view().map(Message::DeleteTableMsg)
            }
            ViewVariant::RenameTable(ref view) => {
                view.view().map(Message::RenameTableMsg)
            }
        }
    }

    fn main_view(&self) -> iced::Element<'_, Message> {
        let header =
            iced::widget::row![button("Back").on_press(Message::Back),];

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        let main_view: iced::Element<_> = column![
            iced::widget::vertical_space().width(Length::Fill),
            centered_row(errmsg),
            centered_row(
                iced::widget::button("Add table")
                    .on_press(Message::AddTable)
                    .width(Length::FillPortion(8)),
            ),
            centered_row(
                iced::widget::button("Delete table")
                    .on_press(Message::DeleteTable)
                    .width(Length::FillPortion(8)),
            ),
            centered_row(
                iced::widget::button("Rename table")
                    .on_press(Message::RenameTable)
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

    fn err(msg: &'static str) -> Task<Message> {
        Task::done(Message::Error(msg))
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Back => {
                Self::err("This should have been propagated higher")
            }
            Message::AddTable => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };

                let (view, task) = add_table::View::new(db.clone());

                self.view = ViewVariant::AddTable(view);

                task.map(Message::AddTableMsg)
            }
            Message::AddTableMsg(add_table::Message::Back) => {
                let ViewVariant::AddTable(ref mut view) = self.view else {
                    return Task::done(Message::Error("Something happened"));
                };

                self.view = ViewVariant::Main(view.get_db().clone());

                Task::none()
            }
            Message::AddTableMsg(msg) => {
                let ViewVariant::AddTable(ref mut view) = self.view else {
                    return Task::done(Message::Error("Couldn't add table"));
                };

                view.update(msg).map(Message::AddTableMsg)
            }
            Message::Error(msg) => {
                self.errmsg = Some(msg);
                Task::none()
            }
            Message::DeleteTable => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };

                self.errmsg = None;
                let (view, task) = delete_table::View::new(db.clone());

                self.view = ViewVariant::DeleteTable(view);

                task.map(Message::DeleteTableMsg)
            }
            Message::DeleteTableMsg(delete_table::Message::Back) => {
                let ViewVariant::DeleteTable(ref mut view) = self.view else {
                    return Task::done(Message::Error("Something happened"));
                };

                self.view = ViewVariant::Main(view.get_db().clone());

                Task::none()
            }
            Message::DeleteTableMsg(msg) => {
                let ViewVariant::DeleteTable(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Not in the right view to do that!",
                    ));
                };

                view.update(msg).map(Message::DeleteTableMsg)
            }
            Message::RenameTable => {
                let ViewVariant::Main(db) = &self.view else {
                    return Task::done(Message::Error("DB is probably uninit"));
                };

                self.errmsg = None;
                let (view, task) = rename_publication::View::new(db.clone());

                self.view = ViewVariant::RenameTable(view);

                task.map(Message::RenameTableMsg)
            }
            Message::RenameTableMsg(rename_publication::Message::Back) => {
                let ViewVariant::RenameTable(ref mut view) = self.view else {
                    return Task::done(Message::Error("Something happened"));
                };

                self.view = ViewVariant::Main(view.get_db().clone());

                Task::none()
            }
            Message::RenameTableMsg(msg) => {
                let ViewVariant::RenameTable(view) = &mut self.view else {
                    return Task::done(Message::Error(
                        "Not in the right view to do that!",
                    ));
                };

                view.update(msg).map(Message::RenameTableMsg)
            }
        }
    }
}
