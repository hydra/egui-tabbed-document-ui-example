
pub fn init() {
    let en_us =
        String::from_utf8_lossy(include_bytes!("../assets/translations/en-US.ftl"));
    egui_i18n::load_translations_from_text("en-US", en_us).unwrap();

    egui_i18n::set_language("en-US");
    egui_i18n::set_fallback("en-US");
}
