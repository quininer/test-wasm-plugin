import { clamp_host, data_view, UTF8_DECODER, utf8_encode, UTF8_ENCODED_LEN } from './intrinsics.js';
export class TestPlugin {
  addToImports(imports) {
  }
  
  async instantiate(module, imports) {
    imports = imports || {};
    this.addToImports(imports);
    
    if (module instanceof WebAssembly.Instance) {
      this.instance = module;
    } else if (module instanceof WebAssembly.Module) {
      this.instance = await WebAssembly.instantiate(module, imports);
    } else if (module instanceof ArrayBuffer || module instanceof Uint8Array) {
      const { instance } = await WebAssembly.instantiate(module, imports);
      this.instance = instance;
    } else {
      const { instance } = await WebAssembly.instantiateStreaming(module, imports);
      this.instance = instance;
    }
    this._exports = this.instance.exports;
  }
  start() {
    const ret = this._exports['start']();
    return ret >>> 0;
  }
  process(arg0, arg1) {
    const memory = this._exports.memory;
    const realloc = this._exports["canonical_abi_realloc"];
    const ptr0 = utf8_encode(arg1, realloc, memory);
    const len0 = UTF8_ENCODED_LEN;
    const ret = this._exports['process'](clamp_host(arg0, 0, 4294967295), ptr0, len0);
    return ret;
  }
  result(arg0) {
    const memory = this._exports.memory;
    const free = this._exports["canonical_abi_free"];
    const ret = this._exports['result'](clamp_host(arg0, 0, 4294967295));
    const ptr0 = data_view(memory).getInt32(ret + 0, true);
    const len0 = data_view(memory).getInt32(ret + 8, true);
    const list0 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0));
    free(ptr0, len0, 1);
    return list0;
  }
}
