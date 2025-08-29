import { useEffect, useRef, useState } from "preact/hooks";

export default function GraphicsViewer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<any>(null);
  const [status, setStatus] = useState("Initializing...");
  const [rotation, setRotation] = useState(0);
  const [scale, setScale] = useState(1);
  const [color, setColor] = useState({ r: 1, g: 0.5, b: 0.2 });
  const [shape, setShape] = useState<"triangle" | "cube">("triangle");
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    let animationId: number;

    const initWasm = async () => {
      try {
        setStatus("Loading WASM binary...");
        
        // Load the WASM binary directly
        const wasmResponse = await fetch('/graphics_engine_bg.wasm');
        if (!wasmResponse.ok) {
          throw new Error(`Failed to fetch WASM binary: ${wasmResponse.status}`);
        }
        const wasmBytes = await wasmResponse.arrayBuffer();
        
        setStatus("Loading WASM JavaScript bindings...");
        
        // Load the JavaScript bindings
        const jsResponse = await fetch('/graphics_engine.js');
        if (!jsResponse.ok) {
          throw new Error(`Failed to fetch WASM JS: ${jsResponse.status}`);
        }
        
        let wasmCode = await jsResponse.text();
        
        // Replace the WASM URL resolution with our pre-loaded bytes
        wasmCode = wasmCode.replace(
          /new URL\('graphics_engine_bg\.wasm', import\.meta\.url\)/g,
          'wasmBytes'
        );
        
        // Add the wasmBytes variable at the top of the module
        wasmCode = `const wasmBytes = new Uint8Array([${Array.from(new Uint8Array(wasmBytes)).join(',')}]);\n` + wasmCode;
        
        setStatus("Creating WASM module...");
        
        // Create a blob with the modified code
        const blob = new Blob([wasmCode], { type: 'application/javascript' });
        const moduleUrl = URL.createObjectURL(blob);
        
        setStatus("Importing WASM module...");
        const wasmModule = await import(moduleUrl);
        
        setStatus("Initializing WASM...");
        await wasmModule.default();
        
        setStatus("Creating graphics engine...");
        if (canvasRef.current) {
          const engine = new wasmModule.GraphicsEngine("graphics-canvas");
          engineRef.current = engine;
          
          // Initial render
          engine.set_rotation(rotation);
          engine.set_scale(scale);
          engine.set_color(color.r, color.g, color.b);
          engine.render();
          
          setStatus("Ready! 🎉");
          
          // Clean up the blob URL
          URL.revokeObjectURL(moduleUrl);
        }
      } catch (error) {
        console.error("WASM loading error:", error);
        setStatus(`Error: ${error.message}`);
      }
    };

    initWasm();

    return () => {
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
    };
  }, []);

  useEffect(() => {
    if (engineRef.current) {
      engineRef.current.set_rotation(rotation);
      engineRef.current.set_scale(scale);
      engineRef.current.set_color(color.r, color.g, color.b);
      
      if (shape === "cube") {
        engineRef.current.render_cube();
      } else {
        engineRef.current.render();
      }
    }
  }, [rotation, scale, color, shape]);

  useEffect(() => {
    let animationId: number;
    
    if (isAnimating && engineRef.current) {
      const animate = () => {
        setRotation(prev => (prev + 0.02) % (Math.PI * 2));
        animationId = requestAnimationFrame(animate);
      };
      animate();
    }

    return () => {
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
    };
  }, [isAnimating]);

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h1>🦀 Rust + WebAssembly Graphics Engine</h1>
      <p style={{ 
        padding: "10px", 
        backgroundColor: status.startsWith("Error") ? "#fee" : "#efe",
        border: `1px solid ${status.startsWith("Error") ? "#faa" : "#afa"}`,
        borderRadius: "4px"
      }}>
        <strong>Status:</strong> {status}
      </p>
      
      <canvas
        ref={canvasRef}
        id="graphics-canvas"
        width={500}
        height={500}
        style={{
          border: "2px solid #333",
          margin: "20px 0",
          display: "block",
          backgroundColor: "#000"
        }}
      />
      
      {engineRef.current && (
        <div style={{ 
          display: "grid", 
          gridTemplateColumns: "1fr 1fr", 
          gap: "20px",
          maxWidth: "600px"
        }}>
          <div>
            <h3>Transform</h3>
            <label style={{ display: "block", margin: "10px 0" }}>
              Rotation: {rotation.toFixed(2)}
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
            
            <label style={{ display: "block", margin: "10px 0" }}>
              Scale: {scale.toFixed(2)}
              <input
                type="range"
                min="0.1"
                max="2"
                step="0.01"
                value={scale}
                onInput={(e) => setScale(parseFloat(e.currentTarget.value))}
                style={{ width: "100%", marginTop: "5px" }}
              />
            </label>
            
            <div style={{ margin: "10px 0" }}>
              <strong>Shape:</strong>
              <div style={{ marginTop: "5px" }}>
                <button
                  onClick={() => setShape("triangle")}
                  style={{
                    padding: "8px 16px",
                    marginRight: "10px",
                    backgroundColor: shape === "triangle" ? "#007acc" : "#f0f0f0",
                    color: shape === "triangle" ? "white" : "black",
                    border: "1px solid #ccc",
                    cursor: "pointer"
                  }}
                >
                  Triangle
                </button>
                <button
                  onClick={() => setShape("cube")}
                  style={{
                    padding: "8px 16px",
                    backgroundColor: shape === "cube" ? "#007acc" : "#f0f0f0",
                    color: shape === "cube" ? "white" : "black",
                    border: "1px solid #ccc",
                    cursor: "pointer"
                  }}
                >
                  Cube
                </button>
              </div>
            </div>
          </div>
          
          <div>
            <h3>Color</h3>
            <label style={{ display: "block", margin: "10px 0" }}>
              Red: {color.r.toFixed(2)}
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={color.r}
                onInput={(e) => setColor(prev => ({ ...prev, r: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%", marginTop: "5px" }}
              />
            </label>
            
            <label style={{ display: "block", margin: "10px 0" }}>
              Green: {color.g.toFixed(2)}
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={color.g}
                onInput={(e) => setColor(prev => ({ ...prev, g: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%", marginTop: "5px" }}
              />
            </label>
            
            <label style={{ display: "block", margin: "10px 0" }}>
              Blue: {color.b.toFixed(2)}
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={color.b}
                onInput={(e) => setColor(prev => ({ ...prev, b: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%", marginTop: "5px" }}
              />
            </label>
            
            <button
              onClick={() => setIsAnimating(!isAnimating)}
              style={{
                width: "100%",
                padding: "10px",
                marginTop: "10px",
                backgroundColor: isAnimating ? "#dc3545" : "#28a745",
                color: "white",
                border: "none",
                cursor: "pointer",
                fontSize: "16px"
              }}
            >
              {isAnimating ? "⏸️ Stop Animation" : "▶️ Start Animation"}
            </button>
          </div>
        </div>
      )}
      
      {engineRef.current && (
        <div style={{ marginTop: "20px" }}>
          <button
            onClick={() => {
              setRotation(0);
              setScale(1);
              setColor({ r: 1, g: 0.5, b: 0.2 });
              setIsAnimating(false);
            }}
            style={{
              padding: "10px 20px",
              backgroundColor: "#6c757d",
              color: "white",
              border: "none",
              cursor: "pointer"
            }}
          >
            Reset All
          </button>
        </div>
      )}
    </div>
  );
}