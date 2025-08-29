import { useEffect, useRef, useState } from "preact/hooks";

declare global {
  interface Window {
    wasm_bindgen: any;
  }
  var wasm_bindgen: any;
}

export default function SimpleWasm() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<any>(null);
  const [status, setStatus] = useState("Checking WASM...");
  const [rotation, setRotation] = useState(0);

  useEffect(() => {
    const initWasm = async () => {
      try {
        setStatus("Checking for WASM...");
        
        // Wait a bit for the script to load completely
        await new Promise(resolve => setTimeout(resolve, 500));
        
        // Check all possible locations for wasm_bindgen
        console.log('Checking wasm_bindgen availability:');
        console.log('- window.wasm_bindgen:', typeof window.wasm_bindgen);
        console.log('- globalThis.wasm_bindgen:', typeof (globalThis as any).wasm_bindgen);
        
        // Try to access the global wasm_bindgen variable
        let wasmBindgen;
        
        // Try different ways to access it
        if (typeof window.wasm_bindgen === 'function') {
          wasmBindgen = window.wasm_bindgen;
          console.log('Found wasm_bindgen on window');
        } else if (typeof (globalThis as any).wasm_bindgen === 'function') {
          wasmBindgen = (globalThis as any).wasm_bindgen;
          console.log('Found wasm_bindgen on globalThis');
        } else {
          // Try to eval it since it should be in global scope
          try {
            wasmBindgen = eval('wasm_bindgen');
            console.log('Found wasm_bindgen via eval, type:', typeof wasmBindgen);
          } catch (e) {
            console.log('Could not eval wasm_bindgen:', e);
          }
        }
        
        if (typeof wasmBindgen !== 'function') {
          // List all global properties to debug
          const globals = Object.getOwnPropertyNames(globalThis).filter(name => 
            name.includes('wasm') || name.includes('bindgen')
          );
          console.log('Global properties containing wasm/bindgen:', globals);
          
          throw new Error(`wasm_bindgen not found. Type: ${typeof wasmBindgen}`);
        }
        
        setStatus("Initializing WASM...");
        console.log('Calling wasm_bindgen init...');
        
        // Initialize the WASM module
        await wasmBindgen();
        
        setStatus("Creating graphics engine...");
        console.log('WASM initialized, checking for GraphicsEngine...');
        console.log('wasmBindgen.GraphicsEngine:', typeof wasmBindgen.GraphicsEngine);
        
        if (canvasRef.current && wasmBindgen.GraphicsEngine) {
          const engine = new wasmBindgen.GraphicsEngine("simple-canvas");
          engineRef.current = engine;
          
          // Try basic rendering
          engine.set_rotation(0);
          engine.set_scale(1);
          engine.set_color(1, 0.5, 0.2);
          engine.render();
          
          setStatus("🎉 Rust WASM Graphics Ready!");
        } else {
          throw new Error("GraphicsEngine class not available or canvas not found");
        }
        
      } catch (error) {
        console.error("WASM initialization error:", error);
        setStatus(`❌ Error: ${error.message}`);
      }
    };

    initWasm();
  }, []);

  useEffect(() => {
    if (engineRef.current) {
      try {
        engineRef.current.set_rotation(rotation);
        engineRef.current.render();
      } catch (error) {
        console.error("Render error:", error);
      }
    }
  }, [rotation]);

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h1>🦀 Simple Rust WASM Test</h1>
      
      <p style={{ 
        padding: "10px", 
        backgroundColor: status.includes("🎉") ? "#d4edda" : status.includes("❌") ? "#f8d7da" : "#d1ecf1",
        border: "1px solid #ccc",
        borderRadius: "4px"
      }}>
        <strong>Status:</strong> {status}
      </p>
      
      <canvas
        ref={canvasRef}
        id="simple-canvas"
        width={400}
        height={400}
        style={{
          border: "2px solid #333",
          margin: "20px 0",
          display: "block",
          backgroundColor: "#000"
        }}
      />
      
      {engineRef.current && (
        <div>
          <label style={{ display: "block", margin: "10px 0" }}>
            <strong>Rotation:</strong> {rotation.toFixed(2)}
            <br />
            <input
              type="range"
              min="0"
              max={Math.PI * 2}
              step="0.01"
              value={rotation}
              onInput={(e) => setRotation(parseFloat(e.currentTarget.value))}
              style={{ width: "100%", marginTop: "5px" }}
            />
          </label>
          
          <p style={{ fontSize: "12px", color: "#666", marginTop: "20px" }}>
            If you can see a triangle and it rotates when you move the slider, 
            then Rust + WebAssembly is working! 🎉
          </p>
        </div>
      )}
      
      <details style={{ marginTop: "20px", fontSize: "12px" }}>
        <summary>Debug Info</summary>
        <p>Check the browser console for detailed loading information.</p>
      </details>
    </div>
  );
}