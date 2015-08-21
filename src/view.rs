use gtk;

struct TermView {
    canvas: gtk::DrawingArea,
}

impl TermView {
    fn new() -> Self {
        let term = TermView { canvas: gtk::DrawingArea::new().unwrap() };
        {
            let canvas = &term.canvas;
        }
        term
    }
}
