use crate::defines::{FONT_SOURCE_CODE_PRO, ICON};
use egui::{FontData, FontDefinitions, FontFamily::Monospace, FontId, IconData, Style, TextStyle};

pub fn load_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let source_code = FontData::from_static(FONT_SOURCE_CODE_PRO);

    fonts
        .font_data
        .insert("source_code_pro".into(), source_code);

    fonts
        .families
        .get_mut(&Monospace)
        .unwrap()
        .insert(0, "source_code_pro".to_owned());

    fonts
}

pub fn register_fonts(style: &mut Style) {
    style.text_styles = [
        (TextStyle::Body, FontId::new(13.0, Monospace)),
        (TextStyle::Button, FontId::new(13.0, Monospace)),
        (TextStyle::Monospace, FontId::new(13.0, Monospace)),
        (TextStyle::Small, FontId::new(10.0, Monospace)),
        (TextStyle::Heading, FontId::new(18.0, Monospace)),
    ]
    .into();
}

pub fn load_icon() -> IconData {
    eframe::icon_data::from_png_bytes(ICON).unwrap()
}
