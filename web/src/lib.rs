use js_sys::{ WebAssembly, Object };
use wasm_bindgen::prelude::*;


#[wasm_bindgen(module = "/js/intrinsics.js")]
extern "C" {
    fn data_view();
}

#[wasm_bindgen(module = "/js/test-plugin.js")]
extern "C" {
    type TestPlugin;

    #[wasm_bindgen(constructor)]
    fn new() -> TestPlugin;

    #[wasm_bindgen(method)]
    async fn instantiate(this: &TestPlugin, module: WebAssembly::Instance, imports: Object);

    #[wasm_bindgen(method)]
    fn start(this: &TestPlugin) -> u32;
    #[wasm_bindgen(method)]
    fn process(this: &TestPlugin, ptr: u32, msg: &str) -> i32;
    #[wasm_bindgen(method)]
    fn result(this: &TestPlugin, ptr: u32) -> String;
}

struct WasmPlugin {
    plugin: TestPlugin,
    ptr: Option<u32>
}

impl wasm_plugin::Plugin for WasmPlugin {
    type Error = js_sys::Error;

    fn start(&mut self) -> Result<(), Self::Error> {
        if self.ptr.is_some() {
            return Err(js_sys::Error::new("plugin has already started"));
        }

        self.ptr = Some(self.plugin.start());
        Ok(())
    }

    fn process(&mut self, msg: &str) -> Result<(), Self::Error> {
        let ptr = self.ptr.clone().ok_or_else(|| js_sys::Error::new("plugin not start"))?;
        let ret = self.plugin.process(ptr, msg);
        if ret == 0 {
            Ok(())
        } else {
            return Err(js_sys::Error::new(&format!("process failed: {}", ret)));
        }
    }

    fn result(&mut self) -> Result<String, Self::Error> {
        let ptr = self.ptr.take().ok_or_else(|| js_sys::Error::new("plugin not start"))?;
        let output = self.plugin.result(ptr);
        Ok(output)
    }
}

#[wasm_bindgen]
pub async fn wasm_command(wasm_instance: WebAssembly::Instance) -> String {
    command(wasm_instance).await
}

pub async fn command(wasm_instance: WebAssembly::Instance) -> String {
    let plugin = TestPlugin::new();
    plugin.instantiate(wasm_instance, Object::new()).await;

    let mut plugin = WasmPlugin { plugin, ptr: None };

    let result = wasm_plugin::command(&mut plugin, vec![
        String::from("123"),
        String::from("456"),
        String::from("789"),
    ]).unwrap();

    result
}
