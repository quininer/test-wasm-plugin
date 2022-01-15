include!("bindings.rs");

use std::path::PathBuf;
use anyhow::Context;
use wasmtime::{ Engine, Module, Instance, Store };
use argh::FromArgs;
use wasm_plugin::Plugin;

/// wasmtime test
#[derive(FromArgs)]
struct Options {
    /// wasm plugin path
    #[argh(positional)]
    plugin: PathBuf
}

struct WasmPlugin {
    store: Store<test_plugin::TestPluginData>,
    plugin: test_plugin::TestPlugin<test_plugin::TestPluginData>,
    ptr: Option<u32>
}

impl Plugin for WasmPlugin {
    type Error = anyhow::Error;

    fn start(&mut self) -> Result<(), Self::Error> {
        if self.ptr.is_some() {
            anyhow::bail!("plugin has already started");
        }

        let ptr = self.plugin.start(&mut self.store)?;
        self.ptr = Some(ptr);
        Ok(())
    }

    fn process(&mut self, msg: &str) -> Result<(), Self::Error> {
        let ptr = self.ptr.clone().context("plugin not start")?;
        let ret = self.plugin.process(&mut self.store, ptr, msg)?;
        if ret == 0 {
            Ok(())
        } else {
            anyhow::bail!("process failed: {}", ret)
        }
    }

    fn result(&mut self) -> Result<String, Self::Error> {
        let ptr = self.ptr.take().context("plugin not start")?;
        let output = self.plugin.result(&mut self.store, ptr)?;
        Ok(output)
    }
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    let engine = Engine::default();
    let module = Module::from_file(&engine, &options.plugin)?;
    let mut store: Store<test_plugin::TestPluginData> = Store::new(&engine, test_plugin::TestPluginData {});
    let instance = Instance::new(&mut store, &module, &[])?;

    let plugin = test_plugin::TestPlugin::<test_plugin::TestPluginData>::new(&mut store, &instance, |data| data)?;
    let mut plugin = WasmPlugin { store, plugin, ptr: None };

    let result = wasm_plugin::command(&mut plugin, vec![
        String::from("123"),
        String::from("456"),
        String::from("789"),
    ])?;

    println!("result: {:?}", result);

    Ok(())
}
