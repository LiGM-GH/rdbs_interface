use iced::{
    widget::{button, column}, Element, Task
};
use tokio_postgres::Client;

use crate::helpers::centered_row;

pub struct View<DB: AsRef<Client>> {
    client: DB,
}

#[derive(Clone, Debug)]
pub enum Message {}

impl<DB: AsRef<Client>> View<DB> {
    pub fn new(client: DB) -> Self {
        Self { client }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![centered_row(button("Thing"),)].into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        Task::none()
    }
}
