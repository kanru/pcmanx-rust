use cairo;
use gdk;
use gtk::signal::Inhibit;
use gtk::traits::*;
use gtk;
use std::cell::Cell;
use std::rc::Rc;

pub struct TermView {
    widget: gtk::DrawingArea,
    state: Cell<TermViewState>,
}

#[derive(Copy,Clone,Debug)]
struct TermViewState;

impl TermView {
    pub fn new() -> Rc<Self> {
        let instance = Rc::new(TermView {
            widget: gtk::DrawingArea::new().unwrap(),
            state: Cell::new(TermViewState),
        });
        let this = instance.clone();
        instance.widget.connect_configure_event(move |_, evt| {
            this.on_configure_event(evt)
        });
        let this = instance.clone();
        instance.widget.connect_draw(move |_, context| {
            this.on_draw(context)
        });
        let this = instance.clone();
        instance.widget.connect_focus_in_event(move |_, evt| {
            this.on_focus_in(evt)
        });
        let this = instance.clone();
        instance.widget.connect_focus_out_event(move |_, evt| {
            this.on_focus_out(evt)
        });
        instance.widget.set_can_focus(true);

        instance
    }
    pub fn as_widget(&self) -> &gtk::DrawingArea {
        &self.widget
    }
    fn on_configure_event(&self, evt: &gdk::EventConfigure) -> Inhibit {
        Inhibit(true)
    }
    fn on_draw(&self, context: cairo::Context) -> Inhibit {
        Inhibit(true)
    }
    fn on_focus_in(&self, evt: &gdk::EventFocus) -> Inhibit {
        Inhibit(true)
    }
    fn on_focus_out(&self, evt: &gdk::EventFocus) -> Inhibit {
        Inhibit(true)
    }
}
