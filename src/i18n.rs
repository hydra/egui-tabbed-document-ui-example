pub fn init() {
    let en_us = String::from_utf8_lossy(include_bytes!("../assets/translations/en-US.ftl"));
    egui_i18n::load_translations_from_text("en-US", en_us).unwrap();

    egui_i18n::set_language("en-US");
    egui_i18n::set_fallback("en-US");
}

pub mod fluent_argument_helpers {
    use egui::TextBuffer;
    use egui_i18n::fluent_bundle::types::{FluentNumber, FluentNumberOptions};
    use egui_i18n::fluent_bundle::{FluentArgs, FluentValue};
    use log::trace;
    use serde_json::Value;
    use std::borrow::Cow;
    use std::collections::HashMap;

    pub fn build_fluent_args<'a>(params: &'a HashMap<Cow<'_, str>, Value>) -> FluentArgs<'a> {
        let mut args = egui_i18n::fluent::FluentArgs::new();
        for (key, value) in params.iter() {
            match value {
                Value::Null => {
                    trace!("encountered null value for field: {}", key);
                }
                Value::Bool(_) => todo!(),
                Value::Number(number) => {
                    // TODO make sure this is correct!  perhaps write some integration tests to prove the conversion is correct.
                    if number.is_f64() {
                        let value = FluentValue::Number(FluentNumber::new(
                            number.as_f64().unwrap(),
                            FluentNumberOptions::default(),
                        ));
                        args.set(key.as_str(), value);
                    } else if number.is_i64() {
                        let value = FluentValue::Number(FluentNumber::new(
                            number.as_i64().unwrap() as f64,
                            FluentNumberOptions::default(),
                        ));
                        args.set(key.as_str(), value);
                    } else if number.is_u64() {
                        let value = FluentValue::Number(FluentNumber::new(
                            number.as_u64().unwrap() as f64,
                            FluentNumberOptions::default(),
                        ));
                        args.set(key.as_str(), value);
                    } else {
                        unreachable!()
                    }
                }
                Value::String(string) => {
                    let value = FluentValue::String(string.into());
                    args.set(key.as_str(), value);
                }
                Value::Array(_) => todo!(),
                Value::Object(_) => todo!(),
            }
        }
        args
    }
}
