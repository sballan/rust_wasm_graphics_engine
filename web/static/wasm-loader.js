// WASM Loader for Rust Graphics Engine
let wasmModule = null;
let wasmInitialized = false;

async function initWasm() {
  if (wasmInitialized) return wasmModule;
  
  try {
    // Load the WASM binary
    const wasmResponse = await fetch('/graphics_engine_bg.wasm');
    const wasmArrayBuffer = await wasmResponse.arrayBuffer();
    
    // Create imports object for WASM instantiation
    const imports = {
      wbg: {
        __wbindgen_string_new: function(ptr, len) {
          const mem = new Uint8Array(wasmModule.memory.buffer);
          const str = new TextDecoder().decode(mem.subarray(ptr, ptr + len));
          return str;
        },
        __wbindgen_throw: function(ptr, len) {
          const mem = new Uint8Array(wasmModule.memory.buffer);
          const msg = new TextDecoder().decode(mem.subarray(ptr, ptr + len));
          throw new Error(msg);
        },
        __wbg_log_1d3ae0273d8f4f8a: function(ptr, len) {
          const mem = new Uint8Array(wasmModule.memory.buffer);
          const msg = new TextDecoder().decode(mem.subarray(ptr, ptr + len));
          console.log(msg);
        },
        // Add other WebGL and DOM bindings as needed
        __wbg_getContext_df50fa48a8876636: function(arg0, arg1, arg2) {
          const canvas = document.getElementById('graphics-canvas');
          return canvas ? canvas.getContext('webgl') : null;
        }
      }
    };
    
    // Instantiate WASM module
    const wasmInstance = await WebAssembly.instantiate(wasmArrayBuffer, imports);
    
    wasmModule = wasmInstance.instance;
    wasmInitialized = true;
    
    // Initialize the module
    if (wasmModule.exports._start) {
      wasmModule.exports._start();
    }
    
    return wasmModule;
  } catch (error) {
    console.error('Failed to initialize WASM:', error);
    throw error;
  }
}

// Export the loader function
window.initWasm = initWasm;