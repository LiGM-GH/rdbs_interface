//! This module contains helpers for views:
//! `centered_row`:
//! | [space1] VALUE [space1] |
//!  <---1---><--N--><---1--->

use iced::{widget::row, Alignment, Element, Length};

pub fn centered_row<'that, Message: 'that>(
    val: impl Into<Element<'that, Message>>,
) -> Element<'that, Message> {
    row![
        iced::widget::horizontal_space().width(Length::FillPortion(1)),
        Into::<Element<Message>>::into(val),
        iced::widget::horizontal_space().width(Length::FillPortion(1)),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}

pub fn centered_row2<'that, Message: 'that>(
    val1: impl Into<Element<'that, Message>>,
    val2: impl Into<Element<'that, Message>>,
) -> Element<'that, Message> {
    row![
        iced::widget::horizontal_space().width(Length::FillPortion(1)),
        Into::<Element<Message>>::into(val1),
        Into::<Element<Message>>::into(val2),
        iced::widget::horizontal_space().width(Length::FillPortion(1)),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center)
    .into()
}

