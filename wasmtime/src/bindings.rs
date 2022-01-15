pub mod test_plugin {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  
  /// Auxiliary data associated with the wasm exports.
  ///
  /// This is required to be stored within the data of a
  /// `Store<T>` itself so lifting/lowering state can be managed
  /// when translating between the host and wasm.
  #[derive(Default)]
  pub struct TestPluginData {
  }
  pub struct TestPlugin<T> {
    get_state: Box<dyn Fn(&mut T) -> &mut TestPluginData + Send + Sync>,
    canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
    canonical_abi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
    memory: wasmtime::Memory,
    process: wasmtime::TypedFunc<(i32,i32,i32,), (i32,)>,
    result: wasmtime::TypedFunc<(i32,), (i32,)>,
    start: wasmtime::TypedFunc<(), (i32,)>,
  }
  impl<T> TestPlugin<T> {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `linker` provided.
    ///
    /// The `get_state` closure is required to access the
    /// auxiliary data necessary for these wasm exports from
    /// the general store's state.
    pub fn add_to_linker(
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut TestPluginData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()> {
      Ok(())
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `linker` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_linker` beforehand. This function will
    /// instantiate the `module` otherwise using `linker`, and
    /// both an instance of this structure and the underlying
    /// `wasmtime::Instance` will be returned.
    ///
    /// The `get_state` parameter is used to access the
    /// auxiliary state necessary for these wasm exports from
    /// the general store state `T`.
    pub fn instantiate(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    module: &wasmtime::Module,
    linker: &mut wasmtime::Linker<T>,
    get_state: impl Fn(&mut T) -> &mut TestPluginData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<(Self, wasmtime::Instance)> {
      Self::add_to_linker(linker, get_state)?;
      let instance = linker.instantiate(&mut store, module)?;
      Ok((Self::new(store, &instance,get_state)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// defined within `store` and wrap them all up in the
    /// returned structure which can be used to interact with
    /// the wasm module.
    pub fn new(
    mut store: impl wasmtime::AsContextMut<Data = T>,
    instance: &wasmtime::Instance,
    get_state: impl Fn(&mut T) -> &mut TestPluginData + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<Self> {
      let mut store = store.as_context_mut();
      let canonical_abi_free= instance.get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
      let canonical_abi_realloc= instance.get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "canonical_abi_realloc")?;
      let memory= instance
      .get_memory(&mut store, "memory")
      .ok_or_else(|| {
        anyhow::anyhow!("`memory` export not a memory")
      })?
      ;
      let process= instance.get_typed_func::<(i32,i32,i32,), (i32,), _>(&mut store, "process")?;
      let result= instance.get_typed_func::<(i32,), (i32,), _>(&mut store, "result")?;
      let start= instance.get_typed_func::<(), (i32,), _>(&mut store, "start")?;
      Ok(TestPlugin{
        canonical_abi_free,
        canonical_abi_realloc,
        memory,
        process,
        result,
        start,
        get_state: Box::new(get_state),
        
      })
    }
    pub fn start(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,)-> Result<u32, wasmtime::Trap> {
      let (result0_0,) = self.start.call(&mut caller, ())?;
      Ok(result0_0 as u32)
    }
    pub fn process(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,ptr: u32,msg: & str,)-> Result<i32, wasmtime::Trap> {
      let func_canonical_abi_realloc = &self.canonical_abi_realloc;
      let memory = &self.memory;
      let vec0 = msg;
      let ptr0 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec0.len() as i32) * 1))?;
      memory.data_mut(&mut caller).store_many(ptr0, vec0.as_ref())?;
      let (result1_0,) = self.process.call(&mut caller, (wit_bindgen_wasmtime::rt::as_i32(ptr), ptr0, vec0.len() as i32, ))?;
      Ok(result1_0)
    }
    pub fn result(&self, mut caller: impl wasmtime::AsContextMut<Data = T>,ptr: u32,)-> Result<String, wasmtime::Trap> {
      let func_canonical_abi_free = &self.canonical_abi_free;
      let memory = &self.memory;
      let (result0_0,) = self.result.call(&mut caller, (wit_bindgen_wasmtime::rt::as_i32(ptr), ))?;
      let load1 = memory.data_mut(&mut caller).load::<i32>(result0_0 + 0)?;
      let load2 = memory.data_mut(&mut caller).load::<i32>(result0_0 + 8)?;
      let ptr3 = load1;
      let len3 = load2;
      Ok(String::from_utf8(
      copy_slice(
      &mut caller,
      memory,
      func_canonical_abi_free,
      ptr3, len3, 1
      )?
      )
      .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?)
    }
  }
  use wit_bindgen_wasmtime::rt::RawMem;
  use wit_bindgen_wasmtime::rt::copy_slice;
}
