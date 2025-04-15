use iced::{Element, Task};

mod auth;
mod manage;
mod theme;

fn main() -> iced::Result {
    iced::application("RDBS_Interface", View::update, View::view)
        .executor::<tokio::runtime::Runtime>()
        .theme(|_val| theme::get())
        .run_with(|| (View::default(), Task::none()))
}

#[derive(Clone, Debug)]
enum Message {
    Auth(auth::Message),
    Manage(manage::Message),
}

enum View {
    Auth(auth::View),
    Manage(manage::View),
}

impl Default for View {
    fn default() -> Self {
        Self::Auth(Default::default())
    }
}

impl View {
    fn view(&self) -> Element<'_, Message> {
        match self {
            View::Auth(view) => view.view().map(Message::Auth),
            View::Manage(view) => view.view().map(Message::Manage),
        }
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Auth(auth::Message::Connected(client)) => {
                let Ok(mut lock) = client.lock() else {
                    return Task::done(Message::Auth(auth::Message::Error(
                        "Didn't lock the guard: poisoned!",
                    )));
                };

                let Some(client) = lock.take() else {
                    return Task::done(Message::Auth(auth::Message::Error(
                        "We are too quick: DB client not created!",
                    )));
                };

                let manage = manage::View::new(client);
                *self = View::Manage(manage);

                Task::none()
            }
            Message::Auth(message) => {
                let &mut Self::Auth(ref mut auth) = self else {
                    *self = Self::Auth(Default::default());

                    return Task::done(Message::Auth(auth::Message::Error(
                        "We weren't authenticating. How did we get authenticated?",
                    )));
                };

                auth.update(message).map(Message::Auth)
            }
            Message::Manage(message) => {
                let &mut Self::Manage(ref mut manage) = self else {
                    *self = Default::default();
                    return Task::done(Message::Auth(auth::Message::Error(
                        "Weren't in a Manage state. Might be improper login?",
                    )));
                };

                manage.update(message).map(Message::Manage)
            }
        }
    }
}
