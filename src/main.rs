use iced::{
    Alignment, Element, Length, Task,
    widget::{column, row, text, text_input},
};

#[derive(Default)]
struct Main {
    dbname: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    DbNameChange(String),
}

fn main() -> iced::Result {
    iced::application("RDBS_Interface", Main::update, Main::view)
        .theme(|_val| iced::Theme::GruvboxDark)
        .run_with(|| (Main::default(), Task::none()))
}

impl Main {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::DbNameChange(name) => {
                self.dbname = Some(name);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let dbname_input = text_input(
            "DB name",
            self.dbname.as_ref().unwrap_or(&String::new()),
        )
        .width(Length::FillPortion(8))
        .on_input(Message::DbNameChange);

        column![
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                dbname_input,
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Center),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .into()
    }
}
