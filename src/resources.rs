use crate::defines::fonts::{FONT_NOTO_SYMBOLS, FONT_NOTO_SYMBOLS2, FONT_SOURCE_CODE_PRO};
use egui::{FontData, FontDefinitions, FontFamily::Monospace, FontId, Style, TextStyle};

pub fn load_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let source_code = FontData::from_static(FONT_SOURCE_CODE_PRO);
    let symbols = FontData::from_static(FONT_NOTO_SYMBOLS);
    let symbols2 = FontData::from_static(FONT_NOTO_SYMBOLS2);

    fonts
        .font_data
        .insert("source_code_pro".into(), source_code);
    fonts.font_data.insert("noto_symbols".into(), symbols);
    fonts.font_data.insert("noto_symbols2".into(), symbols2);

    fonts
        .families
        .get_mut(&Monospace)
        .unwrap()
        .push("source_code_pro".to_owned());
    fonts
        .families
        .get_mut(&Monospace)
        .unwrap()
        .push("noto_symbols".to_owned());
    fonts
        .families
        .get_mut(&Monospace)
        .unwrap()
        .push("noto_symbols2".to_owned());

    fonts
}

pub fn register_fonts(style: &mut Style) {
    style.text_styles = [
        (TextStyle::Body, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Monospace)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Small, FontId::new(9.0, Monospace)),
        (TextStyle::Heading, FontId::new(18.0, Monospace)),
    ]
    .into();
}
