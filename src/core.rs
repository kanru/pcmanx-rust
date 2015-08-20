extern crate gdk;
extern crate gtk;

pub trait View {
    fn on_paint(&self, evt: &gdk::EventExpose);
    fn on_size(&self, evt: &gdk::EventConfigure);
    fn on_set_focus(&self, evt: &gdk::EventFocus);
    fn on_kill_focus(&self, evt: &gdk::EventFocus);

    fn set_context_menu(&self, widget: &gtk::Widget);
}

pub trait TermView : View {
    fn on_pre_keydown(&self, evt: &gdk::EventKey);
    fn on_keydown(&self, evt: &gdk::EventKey);
    fn on_text_input(&self, string: &String);
    fn draw_char(&self, row: u32, col: u32);
}

pub trait TermCharAttr {
}

pub trait TermData {
}

pub trait TermSelection {
}

pub trait Caret {
}

pub trait Font {
}
