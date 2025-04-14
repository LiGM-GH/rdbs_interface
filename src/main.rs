use gio::prelude::SettingsExtManual;
use iced::{
    Alignment, Element, Length, Task,
    widget::{column, row, text_input},
};
use tokio_postgres::NoTls;

#[derive(Default)]
struct Main {
    dbname: Option<String>,
    username: Option<String>,
    password: Option<String>,
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Debug, Clone)]
enum Message {
    DbNameChange(String),
    UsernameInput(String),
    PasswordInput(String),
    HostInput(String),
    PortInput(u16),
    Start,
}

fn get_theme() -> iced::Theme {
    let settings = gio::Settings::new("org.gnome.desktop.interface");
    let theme: String = settings.get("color-scheme");
    match &theme as &str {
        "prefer-light" => iced::Theme::GruvboxLight,
        "prefer-dark" => iced::Theme::GruvboxDark,
        _ => iced::Theme::default(),
    }
}

fn main() -> iced::Result {
    iced::application("RDBS_Interface", Main::update, Main::view)
        .executor::<tokio::runtime::Runtime>()
        .theme(|_val| get_theme())
        .run_with(|| (Main::default(), Task::none()))
}

impl Main {
    fn update(&mut self, msg: Message) -> Task<Message> {
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
            Message::Start => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
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

        column![
            iced::widget::vertical_space().width(Length::Fill),
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                username_input,
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                password_input,
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                dbname_input,
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                host_input,
                port_input,
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                iced::widget::button("Start")
                    .on_press(Message::Start)
                    .width(Length::FillPortion(8)),
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
            iced::widget::vertical_space().width(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}

#[derive(Debug)]
enum ConnErr {
    TokioError(tokio_postgres::Error),
}

impl std::fmt::Display for ConnErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnErr")
            .field(
                "value",
                match self {
                    Self::TokioError(err) => err as &dyn std::fmt::Debug,
                },
            )
            .finish()
    }
}

impl std::error::Error for ConnErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TokioError(err) => err.source(),
        }
    }
}

impl From<tokio_postgres::Error> for ConnErr {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::TokioError(value)
    }
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
