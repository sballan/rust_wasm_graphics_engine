// Initialize WASM module for Rust Graphics Engine
import init, { GraphicsEngine } from './graphics_engine.js';

(async () => {
  try {
    console.log('Starting WASM initialization...');
    
    // Initialize the WASM module
    await init();
    
    // Make wasm_bindgen and exports available globally
    window.wasm_bindgen = init;
    window.wasm_bindgen.GraphicsEngine = GraphicsEngine;
    window.GraphicsEngine = GraphicsEngine;  // Make it directly available
    
    console.log('WASM module loaded successfully');
    console.log('Available exports:', { GraphicsEngine });
    console.log('window.GraphicsEngine:', window.GraphicsEngine);
  } catch (error) {
    console.error('Failed to initialize WASM:', error);
  }
})();