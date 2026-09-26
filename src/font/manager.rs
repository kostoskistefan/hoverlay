use cosmic_text::{Fallback, FontSystem, fontdb};
use sys_locale::get_locale;
use unicode_script::Script;

const FALLBACK_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/NotoSans-Regular.ttf"
));

pub struct FontManager {
    font_system: FontSystem,
}

impl FontManager {
    pub fn new() -> Self {
        let mut db = fontdb::Database::new();

        db.load_system_fonts();
        db.load_font_data(FALLBACK_FONT.to_vec());

        let locale = get_locale()
            .unwrap_or_else(|| "en-US".to_string())
            .replace('_', "-");

        let font_system = FontSystem::new_with_locale_and_db_and_fallback(locale, db, FontFallback);

        Self { font_system }
    }

    pub fn font_system_mut(&mut self) -> &mut FontSystem {
        &mut self.font_system
    }
}

struct FontFallback;

impl Fallback for FontFallback {
    fn common_fallback(&self) -> &[&'static str] {
        &["Noto Sans"]
    }

    fn forbidden_fallback(&self) -> &[&'static str] {
        &[]
    }

    fn script_fallback(&self, _script: Script, _locale: &str) -> &[&'static str] {
        &[]
    }
}
