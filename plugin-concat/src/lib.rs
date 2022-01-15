include!("bindings.rs");

pub struct TestPlugin {
    output: String
}

impl test_plugin::TestPlugin for TestPlugin {
    fn start() -> u32 {
        let plugin = Box::new(TestPlugin { output: String::new() });
        Box::into_raw(plugin) as u32
    }

    fn process(ptr: u32,msg: String,) -> i32 {
        let plugin = unsafe { &mut *(ptr as *mut TestPlugin) };
        plugin.output.push_str(&msg);
        0
    }

    fn result(ptr: u32,) -> String {
        let plugin = unsafe { Box::from_raw(ptr as *mut TestPlugin) };
        plugin.output
    }
}
