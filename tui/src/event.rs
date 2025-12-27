use crossterm::event::Event;
use rat_salsa::event::RenderedEvent;

#[derive(Debug)]
pub enum AppEvent {
    Event(Event),
    Rendered,
}

impl From<RenderedEvent> for AppEvent {
    fn from(_: RenderedEvent) -> Self {
        Self::Rendered
    }
}

impl From<Event> for AppEvent {
    fn from(value: Event) -> Self {
        Self::Event(value)
    }
}
