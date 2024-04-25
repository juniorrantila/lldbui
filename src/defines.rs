pub const APP_NAME: &str = "lldbui";
pub const ICON: &[u8] = include_bytes!("../resources/icon.png");

pub mod fonts {
    pub const FONT_SOURCE_CODE_PRO: &[u8] =
        include_bytes!("../resources/SourceCodePro-Regular.ttf");
    pub const FONT_NOTO_SYMBOLS: &[u8] = include_bytes!("../resources/NotoSansSymbols-Regular.ttf");
    pub const FONT_NOTO_SYMBOLS2: &[u8] =
        include_bytes!("../resources/NotoSansSymbols2-Regular.ttf");
}