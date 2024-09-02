use super::Control;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Control, js_namespace = ["L", "Control"])]
    pub type LayersControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "layers")]
    fn constructor_layers(options: &JsValue) -> LayersControl;
}

impl LayersControl {
    #[must_use]
    pub fn new(options: &JsValue) -> Self {
        constructor_layers(options)
    }
}

