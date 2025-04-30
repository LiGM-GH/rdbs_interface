use iced::{Element, Task};
use tokio_postgres::Client;

pub trait AsClient: AsRef<Client> + Clone + Send + Sync + 'static {}

impl<T: AsRef<Client> + Clone + Send + Sync + 'static> AsClient for T {}

pub trait Viewable {
    type Message: Clone;
    fn back(&self) -> Self::Message;
    fn view(&self) -> Element<'_, Self::Message>;
    fn update(&mut self, msg: Self::Message) -> Task<Self::Message>;
}

pub trait GetDb {
    type DB: AsClient;
    fn get_db(&self) -> Self::DB;
}
