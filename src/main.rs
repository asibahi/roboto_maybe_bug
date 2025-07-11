use rustybuzz::ttf_parser::{self, GlyphId, OutlineBuilder};

fn main() {
    let text = "Roboto";

    let font = include_bytes!("../RobotoSerif[grad,opsz,wdth,wgth].ttf");
    // let font = include_bytes!("../NotoArabic.ttf"); // works !

    {
        eprintln!("RustyBuzz");

        let rb_font = rustybuzz::Face::from_slice(font, 0).unwrap();
        let mut buf = rustybuzz::UnicodeBuffer::new();
        buf.push_str(text);
        let rb_output = rustybuzz::shape(&rb_font, &[], buf);

        let rb_r_glyphid = rb_output.glyph_infos()[0].glyph_id; // should be for R

        {
            let ttf_font = ttf_parser::Face::parse(font, 0).unwrap();
            let mut mock = OutlineMocker;
            ttf_font.outline_glyph(GlyphId(rb_r_glyphid as u16), &mut mock);
        }
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
