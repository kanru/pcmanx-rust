use cairo;
use gdk::cairo_interaction::*;
use gdk;
use gtk::signal::Inhibit;
use gtk::traits::*;
use gtk;
use std::cell::{Cell,RefCell};
use std::rc::Rc;

pub struct TermView {
    widget: gtk::DrawingArea,
    state: Cell<TermViewState>,
    term_data: Option<RefCell<TermData>>,
}

#[derive(Copy,Clone,Debug)]
struct TermViewState {
    left_margin: i32,
    top_margin: i32,
    char_w: i32,
    char_h: i32,
}
struct TermData {
    cols_per_page: i32,
    rows_per_page: i32,
}

impl TermView {
    pub fn new() -> Rc<Self> {
        let instance = Rc::new(TermView {
            widget: gtk::DrawingArea::new().unwrap(),
            state: Cell::new(TermViewState {
                left_margin: 0,
                top_margin: 0,
                char_w: 0,
                char_h: 0,
            }),
            term_data: None,
        });
        instance.widget.connect_destroy({
            let this = instance.clone();
            move |_| {
                this.on_destroy()
            }
        });
        instance.widget.connect_realize({
            let this = instance.clone();
            move |_| {
                this.on_realize()
            }
        });
        instance.widget.connect_configure_event({
            let this = instance.clone();
            move |_, evt| {
                this.on_configure_event(evt)
            }
        });
        instance.widget.connect_draw({
            let this = instance.clone();
            move |_, context| {
                this.on_draw(context)
            }
        });
        instance.widget.connect_focus_in_event({
            let this = instance.clone();
            move |_, evt| {
                this.on_focus_in(evt)
            }
        });
        instance.widget.connect_focus_out_event({
            let this = instance.clone();
            move |_, evt| {
                this.on_focus_out(evt)
            }
        });
        instance.widget.set_can_focus(true);

        instance
    }
    pub fn as_widget(&self) -> &gtk::DrawingArea {
        &self.widget
    }
    pub fn refresh(&self) {
        if !self.widget.get_realized() {
            return;
        }
        self.widget.queue_draw();
    }
    fn point_to_line_col(&self, x: i32, y: i32) -> (i32, i32) {
        (x, y)
    }
    fn draw_char(&self, row: i32, col: i32) {
    }
    fn on_destroy(&self) {
    }
    fn on_realize(&self) {
    }
    fn on_configure_event(&self, evt: &gdk::EventConfigure) -> Inhibit {
        Inhibit(true)
    }
    fn on_draw(&self, cr: cairo::Context) -> Inhibit {
        let rect = cr.get_clip_rectangle();
        if rect.is_none() {
            return Inhibit(false);
        }
        match self.term_data {
            Some(ref term_data) => {
                let term_data = term_data.borrow();
                // Only redraw the invalid area to enhance performance
                let cairo::RectangleInt { x, y, width, height } = rect.unwrap();
                let (mut left, mut top) = self.point_to_line_col(x, y);
                let (mut right, mut bottom) = self.point_to_line_col(width, height);

                // FIXME
                if right < term_data.cols_per_page {
                    right += 1;
                }
                if bottom < term_data.rows_per_page {
                    bottom += 1;
                }
                if top > 0 {
                    if top > 1 {
                        top -= 2;
                    } else {
                        top -= 1;
                    }
                }

                for row in top..bottom {
                    for col in left..right {
                        self.draw_char(row, col);
                    }
                }

                let w = self.widget.get_allocated_width();
                let h = self.widget.get_allocated_height();
                let left_margin = self.state.get().left_margin;
                let top_margin = self.state.get().top_margin;
                let left = term_data.cols_per_page * self.state.get().char_w - 2;
                let top = term_data.rows_per_page * self.state.get().char_h;
                cr.set_source_rgb(0.0, 0.0, 0.0);
                cr.rectangle(0.0, 0.0, left_margin as f64, h as f64);
                cr.rectangle((left + left_margin) as f64, 0.0, (w - left) as f64, h as f64);
                cr.rectangle(0.0, 0.0, w as f64, top_margin as f64);
                cr.rectangle(0.0, (top + top_margin) as f64, w as f64, (h - top) as f64);
                cr.fill();
            },
            None => {
                cr.set_source_rgb(0.0, 0.0, 0.0);
                cr.paint();
            }
        }
        Inhibit(true)
    }
    fn on_focus_in(&self, evt: &gdk::EventFocus) -> Inhibit {
        Inhibit(true)
    }
    fn on_focus_out(&self, evt: &gdk::EventFocus) -> Inhibit {
        Inhibit(true)
    }
}
