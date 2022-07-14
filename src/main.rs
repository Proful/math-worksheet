use printpdf::*;
use rand::Rng;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    pdf("out/add-14-jul-v1.pdf", "+");
}

fn num_pair() -> (String, String) {
    let mut rng = rand::thread_rng();

    let (mut n1, mut n2) = (rng.gen_range(10000..99999), rng.gen_range(10000..99999));
    if n1 < n2 {
        (n1, n2) = (n2, n1);
    }
    (format!("{}", n1), format!("{}", n2))
}

fn pdf(file_name: &str, op: &str) {
    const WIDTH: f64 = 210.0;
    const HEIGHT: f64 = 297.0;
    const COL1_X: f64 = 10.0;
    const COL2_X: f64 = 120.0;
    const ROW1_Y: f64 = 267.0;
    const ROW2_Y: f64 = 167.0;
    const ROW3_Y: f64 = 67.0;

    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(WIDTH), Mm(HEIGHT), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesBold).unwrap();

    for i in 1..=10 {
        let current_layer = if i == 1 {
            doc.get_page(page1).get_layer(layer1)
        } else {
            let (page2, layer) = doc.add_page(Mm(WIDTH), Mm(HEIGHT), &format!("Page {}", i));
            doc.get_page(page2).get_layer(layer)
        };

        add_text(&current_layer, &font, COL1_X, ROW1_Y, op);
        add_text(&current_layer, &font, COL2_X, ROW1_Y, op);
        add_text(&current_layer, &font, COL1_X, ROW2_Y, op);
        add_text(&current_layer, &font, COL2_X, ROW2_Y, op);
        add_text(&current_layer, &font, COL1_X, ROW3_Y, op);
        add_text(&current_layer, &font, COL2_X, ROW3_Y, op);
    }

    doc.save(&mut BufWriter::new(File::create(file_name).unwrap()))
        .unwrap();
}

fn add_text(current_layer: &PdfLayerReference, font: &IndirectFontRef, x: f64, y: f64, op: &str) {
    current_layer.begin_text_section();

    current_layer.set_font(&font, 24.0);
    current_layer.set_text_cursor(Mm(x), Mm(y));
    current_layer.set_line_height(46.0);
    current_layer.set_character_spacing(30.0);

    let (a, b) = num_pair();
    let hy = "-".repeat(a.len() + 2);
    current_layer.write_text(format!(" {a}"), &font);
    current_layer.add_line_break();
    current_layer.write_text(format!("{op}{b}"), &font);
    current_layer.add_line_break();
    current_layer.write_text(hy.clone(), &font);
    current_layer.add_line_break();
    current_layer.write_text(hy.clone(), &font);

    current_layer.end_text_section();
}
