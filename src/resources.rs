use crate::defines::fonts::{FONT_NOTO_SYMBOLS, FONT_NOTO_SYMBOLS2};
use egui::{FontData, FontDefinitions, FontFamily::Monospace};

pub fn load_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let symbols = FontData::from_static(FONT_NOTO_SYMBOLS);
    let symbols2 = FontData::from_static(FONT_NOTO_SYMBOLS2);

    fonts.font_data.insert("noto_symbols".into(), symbols);
    fonts.font_data.insert("noto_symbols2".into(), symbols2);
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
