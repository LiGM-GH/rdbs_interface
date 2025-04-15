//! This module contains Manage view and its messages

use iced::{
    Alignment, Length, Task,
    widget::{column, row},
};
use tokio_postgres::Client;

pub struct View {
    client: Option<Client>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Hello,
}

impl View {
    pub fn new(client: Client) -> Self {
        Self {
            client: Some(client),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        column![
            iced::widget::vertical_space().width(Length::Fill),
            row![
                iced::widget::horizontal_space().width(Length::FillPortion(1)),
                iced::widget::button("OK, let's GO!")
                    .on_press(Message::Hello)
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

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Hello => Task::none(),
        }
    }
}
