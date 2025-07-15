use skrifa::{MetadataProvider, outline::OutlinePen, prelude::Size};
use ttf_parser::{self, GlyphId, OutlineBuilder};

fn main() {
    let font = include_bytes!("../RobotoSerif[grad,opsz,wdth,wgth].ttf");

    let rb_r_glyphid = 130; // R in Roboto font.
    let mut mock = OutlineMocker;

    {
        eprintln!("ttf_parser");
        // only succeeds if `gvar-alloc` feature is turned on.
        let ttf_font = ttf_parser::Face::parse(font, 0).unwrap();
        let res = ttf_font.outline_glyph(GlyphId(rb_r_glyphid as u16), &mut mock);
        dbg!(res.is_some());
    }
    {
        eprintln!("skrifa");
        let sk_font = skrifa::FontRef::new(font).unwrap();
        let sk_outlines = sk_font
            .outline_glyphs()
            .get(skrifa::GlyphId::new(rb_r_glyphid))
            .unwrap();
        let res = sk_outlines.draw(Size::unscaled(), &mut mock);
        dbg!(res.is_ok());
    }
}

struct OutlineMocker;

impl OutlineBuilder for OutlineMocker {
    fn move_to(&mut self, x: f32, y: f32) {
        eprintln!("move_to {x}, {y}");
    }

    fn line_to(&mut self, x: f32, y: f32) {
        eprintln!("line_to {x}, {y}");
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        eprintln!("quad_to {x1}, {y1}, {x}, {y}");
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        eprintln!("curv_to {x1}, {y1}, {x2}, {y2}, {x}, {y}");
    }

    fn close(&mut self) {
        eprintln!("close");
    }
}

impl OutlinePen for OutlineMocker {
    fn move_to(&mut self, x: f32, y: f32) {
        eprintln!("move_to {x}, {y}");
    }

    fn line_to(&mut self, x: f32, y: f32) {
        eprintln!("line_to {x}, {y}");
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        eprintln!("quad_to {cx0}, {cy0}, {x}, {y}");
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        eprintln!("curv_to {cx0}, {cy0}, {cx1}, {cy1}, {x}, {y}");
    }

    fn close(&mut self) {
        eprintln!("close");
    }
}
