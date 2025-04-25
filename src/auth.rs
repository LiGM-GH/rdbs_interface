//! This module contains auth view and its messages

use std::sync::Arc;

use iced::{
    Alignment, Color, Element, Length, Task,
    widget::{button, column, text, text_input},
};
use tokio_postgres::NoTls;

use crate::{
    helpers::{centered_row, centered_row2},
    widgets::event_catcher::{enter_catcher, event_catcher},
};

#[derive(Default)]
pub struct View {
    dbname: Option<String>,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    errmsg: Option<&'static str>,
}

#[derive(Clone)]
pub enum Message {
    Error(&'static str),

    DbNameChange(String),
    UsernameInput(String),
    PasswordInput(String),
    HostInput(String),
    PortInput(u16),
    Start,

    Connected(Arc<tokio_postgres::Client>),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::DbNameChange(arg0) => {
                f.debug_tuple("DbNameChange").field(arg0).finish()
            }
            Self::UsernameInput(arg0) => {
                f.debug_tuple("UsernameInput").field(arg0).finish()
            }
            Self::PasswordInput(_arg0) => f
                .debug_tuple("PasswordInput")
                .field(&"***" as &dyn std::fmt::Debug)
                .finish(),
            Self::HostInput(arg0) => {
                f.debug_tuple("HostInput").field(arg0).finish()
            }
            Self::PortInput(arg0) => {
                f.debug_tuple("PortInput").field(arg0).finish()
            }
            Self::Start => write!(f, "Start"),
            Self::Connected(arg0) => {
                f.debug_tuple("Connected").field(arg0).finish()
            }
        }
    }
}

impl View {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::DbNameChange(name) => {
                self.dbname = Some(name);
                Task::none()
            }
            Message::UsernameInput(name) => {
                self.username = Some(name);
                Task::none()
            }
            Message::PasswordInput(password) => {
                self.password = Some(password);
                Task::none()
            }
            Message::HostInput(host) => {
                self.host = Some(host);
                Task::none()
            }
            Message::PortInput(port) => {
                self.port = Some(port);
                Task::none()
            }

            Message::Start => {
                let Some(username) = &self.username else {
                    return Task::done(Message::Error("No username provided"));
                };

                let Some(password) = &self.password else {
                    return Task::done(Message::Error("No password provided"));
                };

                let Some(host) = &self.host else {
                    return Task::done(Message::Error("No host provided"));
                };

                let Some(dbname) = &self.dbname else {
                    return Task::done(Message::Error("No dbname provided"));
                };

                let Some(port) = &self.port else {
                    return Task::done(Message::Error("No port provided"));
                };

                self.errmsg = None;

                Task::perform(
                    connect(
                        username.clone(),
                        password.clone(),
                        host.clone(),
                        *port,
                        dbname.clone(),
                    ),
                    |val| match val {
                        Ok(client) => Message::Connected(Arc::new(client)),
                        Err(_err) => Message::Error("Couldn't connect to DB"),
                    },
                )
            }
            Message::Error(err) => {
                self.errmsg = Some(err);
                Task::none()
            }
            Message::Connected(_) => Task::done(Message::Error(
                "Connected message should have been handled higher",
            )),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let username_input = text_input(
            "Username",
            self.username.as_ref().unwrap_or(&String::new()),
        )
        .width(Length::FillPortion(8))
        .on_input(Message::UsernameInput);

        let password_input = text_input(
            "Password",
            self.password.as_ref().unwrap_or(&String::new()),
        )
        .secure(true)
        .width(Length::FillPortion(8))
        .on_input(Message::PasswordInput);

        let dbname_input = text_input(
            "DB name",
            self.dbname.as_ref().unwrap_or(&String::new()),
        )
        .width(Length::FillPortion(8))
        .on_input(Message::DbNameChange);

        let host_input = iced::widget::text_input(
            "Host",
            self.host.as_ref().map_or("", |val| val as &str),
        )
        .on_input(Message::HostInput)
        .width(Length::FillPortion(6));

        let port_input = iced_aw::typed_input::<u16, _, _, _, _>(
            self.port.as_ref().unwrap_or(&0),
            Message::PortInput,
        )
        .width(Length::FillPortion(2));

        let errmsg: Element<Message> = text(self.errmsg.unwrap_or(""))
            .color(Color::from_rgba(1.0, 0.0, 0.0, 1.0))
            .into();

        column![
            iced::widget::vertical_space().width(Length::Fill),
            centered_row(errmsg),
            centered_row(username_input),
            centered_row(password_input),
            centered_row(dbname_input),
            centered_row2(host_input, port_input),
            centered_row(event_catcher(
                button("Start").on_press(Message::Start),
                enter_catcher(Message::Start)
            )),
            iced::widget::vertical_space().width(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}

#[derive(Debug, thiserror::Error)]
enum ConnErr {
    #[error("Tokio error {0:?}")]
    TokioError(#[from] tokio_postgres::Error),
}

async fn connect(
    username: String,
    password: String,
    host: String,
    port: u16,
    dbname: String,
) -> Result<tokio_postgres::Client, ConnErr> {
    let mut config = tokio_postgres::Config::new();

    config
        .user(username)
        .password(password)
        .host(host)
        .port(port)
        .dbname(dbname);

    let (client, conn) = config.connect(NoTls).await?;

    tokio::task::spawn(conn);

    Ok(client)
}
