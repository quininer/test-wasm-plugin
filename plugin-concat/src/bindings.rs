mod test_plugin {
  #[export_name = "start"]
  unsafe extern "C" fn __wit_bindgen_start() -> i32{
    let result0 = <super::TestPlugin as TestPlugin>::start();
    wit_bindgen_rust::rt::as_i32(result0)
  }
  #[export_name = "process"]
  unsafe extern "C" fn __wit_bindgen_process(arg0: i32, arg1: i32, arg2: i32, ) -> i32{
    let len0 = arg2 as usize;
    let result1 = <super::TestPlugin as TestPlugin>::process(arg0 as u32, String::from_utf8(Vec::from_raw_parts(arg1 as *mut _, len0, len0)).unwrap());
    wit_bindgen_rust::rt::as_i32(result1)
  }
  #[export_name = "result"]
  unsafe extern "C" fn __wit_bindgen_result(arg0: i32, ) -> i32{
    let result0 = <super::TestPlugin as TestPlugin>::result(arg0 as u32);
    let vec1 = (result0.into_bytes()).into_boxed_slice();
    let ptr1 = vec1.as_ptr() as i32;
    let len1 = vec1.len() as i32;
    core::mem::forget(vec1);
    let ptr2 = RET_AREA.as_mut_ptr() as i32;
    *((ptr2 + 8) as *mut i32) = len1;
    *((ptr2 + 0) as *mut i32) = ptr1;
    ptr2
  }
  pub trait TestPlugin {
    fn start() -> u32;
    fn process(ptr: u32,msg: String,) -> i32;
    fn result(ptr: u32,) -> String;
  }
  static mut RET_AREA: [i64; 2] = [0; 2];
}
