use iced::{Element, Renderer, Theme, advanced::Widget};

pub trait FunForMessage: Clone {
    type Message;
    fn call(self, event: iced::Event) -> Option<Self::Message>;
}

impl<Message, T: Fn(iced::Event) -> Option<Message> + Clone> FunForMessage for T {
    type Message = Message;

    fn call(self, event: iced::Event) -> Option<Self::Message> {
        (self)(event)
    }
}

pub struct EventCatcher<'a, Message, Fun: FunForMessage<Message = Message>> {
    inner: iced::Element<'a, Message>,
    fun: Fun,
}

pub fn event_catcher<Message, Fun: FunForMessage<Message = Message>>(
    inner: iced::Element<'_, Message>,
    fun: Fun,
) -> EventCatcher<'_, Message, Fun> {
    EventCatcher::new(inner, fun)
}

impl<'a, Message, Fun: FunForMessage<Message = Message>>
    EventCatcher<'a, Message, Fun>
{
    pub fn new(inner: iced::Element<'a, Message>, fun: Fun) -> Self {
        Self { inner, fun }
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
        self.inner
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn on_event(
        &mut self,
        state: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        FunForMessage::call(self.fun.clone(), event.clone())
            .map(|msg| shell.publish(msg));

        self.inner.as_widget_mut().on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        self.inner.as_widget().children()
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.inner.as_widget().tag()
    }

    fn diff(&self, _tree: &mut iced::advanced::widget::Tree) {
        self.inner.as_widget().diff(_tree)
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.inner.as_widget().state()
    }

    fn operate(
        &self,
        state: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.inner
            .as_widget()
            .operate(state, layout, renderer, operation);
    }

    fn overlay<'a>(
        &'a mut self,
        state: &'a mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'a, Message, Theme, Renderer>>
    {
        self.inner
            .as_widget_mut()
            .overlay(state, layout, renderer, translation)
    }

    fn size_hint(&self) -> iced::Size<iced::Length> {
        self.inner.as_widget().size_hint()
    }

    fn mouse_interaction(
        &self,
        state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.inner
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }
}
