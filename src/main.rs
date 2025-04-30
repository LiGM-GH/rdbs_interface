#![warn(
    clippy::all,
    clippy::perf,
    clippy::style,
    clippy::nursery,
    clippy::pedantic,
    clippy::complexity,
    clippy::suspicious
)]
#![allow(clippy::uninlined_format_args)]

use std::sync::Arc;

use iced::{
    Element, Task,
    keyboard::{Key, Modifiers},
    widget::{focus_next, focus_previous},
};
use log::info;
use log4rs::config::Deserializers;
use tokio_postgres::Client;
use traits::Viewable;

mod auth;
mod helpers;
mod manage;
mod theme;
mod widgets;
mod traits;

fn main() -> iced::Result {
    log4rs::init_file("log4rs.yaml", Deserializers::default()).unwrap();

    info!("Starting the app");

    iced::application("RDBS_Interface", View::update, View::view)
        .executor::<tokio::runtime::Runtime>()
        .theme(|_val| theme::get())
        .subscription(|_this| {
            iced::keyboard::on_key_press(|key, modif| match key {
                Key::Named(iced::keyboard::key::Named::Tab)
                    if modif == Modifiers::empty() =>
                {
                    Some(Message::FocusNext)
                }
                Key::Named(iced::keyboard::key::Named::Tab)
                    if modif == Modifiers::SHIFT =>
                {
                    Some(Message::FocusPrev)
                }
                Key::Named(iced::keyboard::key::Named::Backspace) => {
                    Some(Message::LittleBack)
                }
                _ => None,
            })
        })
        .run_with(|| (View::default(), Task::none()))
}

#[derive(Clone, Debug)]
enum Message {
    Auth(auth::Message),
    Manage(manage::Message),
    LittleBack,
    FocusNext,
    FocusPrev,
}

struct View {
    view: ViewVariant,
    auth: auth::View,
}

#[derive(Debug)]
enum ViewVariant {
    Auth,
    Manage(manage::View<Arc<Client>>),
}

impl Default for View {
    fn default() -> Self {
        Self {
            view: ViewVariant::Auth,
            auth: auth::View::default(),
        }
    }
}

impl View {
    fn view(&self) -> Element<'_, Message> {
        match &self.view {
            ViewVariant::Auth => self.auth.view().map(Message::Auth),
            ViewVariant::Manage(view) => view.view().map(Message::Manage),
        }
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        log::trace!("main: update: message: {:?}", msg);

        match msg {
            Message::LittleBack => match &self.view {
                ViewVariant::Auth => Task::none(),
                ViewVariant::Manage(view) => {
                    Task::done(Message::Manage(view.back()))
                }
            },
            Message::FocusNext => focus_next(),
            Message::FocusPrev => focus_previous(),
            Message::Auth(auth::Message::Connected(client)) => {
                let manage = manage::View::new(client);
                self.view = ViewVariant::Manage(manage);

                Task::none()
            }
            Message::Auth(message) => {
                self.auth.update(message).map(Message::Auth)
            }
            Message::Manage(manage::Message::Back) => {
                self.view = ViewVariant::Auth;
                Task::none()
            }
            Message::Manage(message) => {
                log::trace!(
                    "This is message: {:?}, and that is view: {:?}",
                    message,
                    self.view
                );

                let ViewVariant::Manage(manage) = &mut self.view else {
                    self.view = ViewVariant::Auth;

                    return Task::done(Message::Auth(auth::Message::Error(
                        "Weren't in a Manage state. Might be improper login?",
                    )));
                };

                manage.update(message).map(Message::Manage)
            }
        }
    }
}
