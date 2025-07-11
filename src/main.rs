use ab_glyph::Font;

fn main() {
    let text = "Roboto";

    let font = include_bytes!("../RobotoSerif[grad,opsz,wdth,wgth].ttf");
    // let font = include_bytes!("../NotoArabic.ttf"); // works !

    let ab_font = ab_glyph::FontRef::try_from_slice(font).unwrap();

    {
        eprintln!("RustyBuzz");

        let rb_font = rustybuzz::Face::from_slice(font, 0).unwrap();
        let mut buf = rustybuzz::UnicodeBuffer::new();
        buf.push_str(text);
        let rb_output = rustybuzz::shape(&rb_font, &[], buf);

        let rb_r_glyphid = rb_output.glyph_infos()[0].glyph_id; // should be for R

        let ab_glyph_id = ab_glyph::GlyphId(rb_r_glyphid as u16);
        eprintln!("{}: {:?}", rb_r_glyphid, ab_font.outline(ab_glyph_id));
    }

    {
        eprintln!("HarfBuzz");

        let hb_font = harfbuzz_rs::Face::from_bytes(font, 0);
        let hb_font = harfbuzz_rs::Font::new(hb_font);
        let buf = harfbuzz_rs::UnicodeBuffer::new().add_str(text);
        let hb_output = harfbuzz_rs::shape(&hb_font, buf, &[]);

        let hb_r_glyphid = hb_output.get_glyph_infos()[0].codepoint; // should be for R

        let ab_glyph_id = ab_glyph::GlyphId(hb_r_glyphid as u16);
        eprintln!("{}: {:?}", hb_r_glyphid, ab_font.outline(ab_glyph_id));
    };

    // println!("Hello, world!");
}
