use iced::{
    Element, Task,
    widget::{column, text},
};

#[derive(Default)]
struct Main;

#[derive(Debug)]
enum Message {}

fn main() {
    iced::application("TITLE", Main::update, Main::view)
        .run_with(|| (Main, Task::none()));
}

impl Main {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {}
    }

    fn view(&self) -> Element<'_, Message> {
        column![text("view")].into()
    }
}
