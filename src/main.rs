use gio::prelude::SettingsExtManual;
use iced::{
    Alignment, Element, Length, Task,
    widget::{column, row, text_input},
};

#[derive(Default)]
struct Main {
    dbname: Option<String>,
    username: Option<String>,
    password: Option<String>,
    this_thing_expanded: bool,
}

#[derive(Debug, Clone)]
enum Message {
    DbNameChange(String),
    UsernameInput(String),
    PasswordInput(String),
    ThisThingPress,
    ThisThingDismissed,
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
                self.this_thing_expanded = true;
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
            Message::ThisThingPress => {
                self.username = Some("OH NO!".into());
                Task::none()
            }
            Message::ThisThingDismissed => {
                self.this_thing_expanded = false;
                Task::none()
            }
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

        let dbname_input = iced_aw::DropDown::new(
            text_input(
                "DB name",
                self.dbname.as_ref().unwrap_or(&String::new()),
            )
            .width(Length::FillPortion(8))
            .on_input(Message::DbNameChange),
            iced::widget::button("This is a thing")
                .on_press(Message::ThisThingPress),
            self.this_thing_expanded,
        )
        .on_dismiss(Message::ThisThingDismissed);

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
            iced::widget::vertical_space().width(Length::Fill),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}
