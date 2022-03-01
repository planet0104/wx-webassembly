use js_sys::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[wasm_bindgen(js_namespace = wx)]
    fn showModal(param: &Object);
}

fn on_load() -> Result<JsValue, JsValue> {
    let param = Object::new();
    Reflect::set(&param, &JsValue::from("title"), &JsValue::from("提示"))?;
    Reflect::set(
        &param,
        &JsValue::from("content"),
        &JsValue::from("这是一个模态弹窗"),
    )?;
    showModal(&param);
    Ok(JsValue::TRUE)
}

#[wasm_bindgen(js_name = onLoad)]
#[allow(non_snake_case)]
pub fn onLoad() {
    match on_load() {
        Err(err) => error(&format!("{:?}", err)),
        _ => (),
    };
}

#[wasm_bindgen(js_name = sha1)]
pub fn sha1(src: Uint8ClampedArray) -> JsValue {
    use sha1::{Sha1, Digest};
    
    let mut hasher = Sha1::new();
    hasher.update(src.to_vec());
    let result = hasher.finalize().to_vec();

    // let output = Uint8ClampedArray::new_with_length(result.len() as u32);
    // output.copy_from(&result);

    JsValue::from(hex::encode(result))
}

#[wasm_bindgen(start)]
pub fn run() {
    log("start");
}