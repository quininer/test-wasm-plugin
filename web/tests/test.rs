use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use js_sys::{ WebAssembly, Object, Reflect };
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_web_plugin() {
    static WASM: &[u8] = include_bytes!(env!("TEST_WASM_PLUGIN"));

    let value = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &Object::new())).await.unwrap();
    let wasm_instance = Reflect::get(&value, &"instance".into()).unwrap()
        .dyn_into::<WebAssembly::Instance>().unwrap();

    let result = wasm_plugin_web::command(wasm_instance).await;
    console_log!("result: {}", result);
}
