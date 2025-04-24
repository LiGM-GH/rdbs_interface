use iced::{
    Element, Renderer, Theme,
    advanced::{Widget, widget::operation::Focusable},
    keyboard::{Key, key::Named},
};

struct State {
    has_focus: bool,
}

pub trait FunForMessage: Clone {
    type Message;
    fn call(self, event: iced::Event) -> Option<Self::Message>;
}

impl<Message, T: Fn(iced::Event) -> Option<Message> + Clone> FunForMessage
    for T
{
    type Message = Message;

    fn call(self, event: iced::Event) -> Option<Self::Message> {
        (self)(event)
    }
}

pub struct EventCatcher<'a, Message, Fun: FunForMessage<Message = Message>> {
    id: iced::advanced::widget::Id,
    inner: iced::Element<'a, Message>,
    fun: Fun,
}

pub fn enter_catcher<Message: Clone>(
    msg: Message,
) -> impl FunForMessage<Message = Message> {
    move |event| match event {
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
            key: Key::Named(Named::Enter),
            ..
        }) => Some(msg.clone()),
        _ => None,
    }
}

pub fn event_catcher<'a, Message, Fun: FunForMessage<Message = Message>>(
    inner: impl Into<iced::Element<'a, Message>>,
    fun: Fun,
) -> EventCatcher<'a, Message, Fun> {
    EventCatcher::new(inner.into(), fun)
}

impl<'a, Message, Fun: FunForMessage<Message = Message>>
    EventCatcher<'a, Message, Fun>
{
    pub fn new(inner: iced::Element<'a, Message>, fun: Fun) -> Self {
        Self {
            id: iced::advanced::widget::Id::unique(),
            inner,
            fun,
        }
    }
}

impl<'a, Message: 'a, Fun: FunForMessage<Message = Message> + 'a>
    From<EventCatcher<'a, Message, Fun>> for Element<'a, Message>
{
    fn from(value: EventCatcher<'a, Message, Fun>) -> Self {
        Element::new(value)
    }
}

impl<Message, Fun: FunForMessage<Message = Message>>
    Widget<Message, Theme, Renderer> for EventCatcher<'_, Message, Fun>
{
    fn size(&self) -> iced::Size<iced::Length> {
        self.inner.as_widget().size()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let tree = tree.children.first_mut().unwrap();

        self.inner.as_widget().layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let tree = tree.children.first().unwrap();
        let layout = layout.children().next().unwrap();

        self.inner
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn on_event(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        'block: {
            if let Some(msg) =
                FunForMessage::call(self.fun.clone(), event.clone())
            {
                let state = tree.state.downcast_mut::<State>();

                if state.is_focused() {
                    log::trace!("Self is focused!");
                    shell.publish(msg);
                    return iced::advanced::graphics::core::event::Status::Captured;
                } else {
                    log::trace!("Self is not focused!");
                    break 'block;
                }
            } else {
                log::trace!("Event is not our event!");
                break 'block;
            }
        }

        let state = tree.children.first_mut().unwrap();
        let layout = layout.children().next().unwrap();

        self.inner.as_widget_mut().on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(&self.inner)]
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.inner.as_widget().tag()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        tree.diff_children(&[&self.inner])
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(State {
            has_focus: false,
        })
    }

    fn operate(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();

        operation.focusable(state, Some(&self.id));

        let child = tree.children.get_mut(0).unwrap();

        self.inner.as_widget().operate(
            child,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'a, Message, Theme, Renderer>>
    {
        let state = tree.children.first_mut().unwrap();
        let layout = layout.children().next().unwrap();

        self.inner
            .as_widget_mut()
            .overlay(state, layout, renderer, translation)
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        self.inner.as_widget().size_hint()
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        let state = tree.children.first().unwrap();
        let layout = layout.children().next().unwrap();

        self.inner
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }
}

impl iced::advanced::widget::operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.has_focus
    }

    fn focus(&mut self) {
        log::trace!("Self is being focused!");
        self.has_focus = true;
    }

    fn unfocus(&mut self) {
        log::trace!("Self is being unfocused!");
        self.has_focus = false;
    }
}
