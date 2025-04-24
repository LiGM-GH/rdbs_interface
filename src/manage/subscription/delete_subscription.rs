use iced::{
    Element, Task,
    widget::{button, column},
};

use crate::{helpers::centered_row, manage::AsClient};

#[derive(Debug)]
pub struct View<DB: AsClient> {
    client: DB,
}

#[derive(Clone, Debug)]
pub enum Message {}

impl<DB: AsClient> View<DB> {
    pub fn get_db(&self) -> DB {
        self.client.clone()
    }

    pub fn new(client: DB) -> Self {
        Self { client }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![centered_row(button("Add table"),)].into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        Task::none()
    }
}
