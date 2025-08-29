import { useEffect, useRef, useState } from "preact/hooks";

declare global {
  interface Window {
    wasm_bindgen: any;
    GraphicsEngine: any;
  }
  var wasm_bindgen: any;
}

export default function RustWasmGraphics() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<any>(null);
  const [status, setStatus] = useState("Loading...");
  const [rotation, setRotation] = useState(0);
  const [scale, setScale] = useState(1);
  const [color, setColor] = useState({ r: 1, g: 0.5, b: 0.2 });
  const [shape, setShape] = useState<"triangle" | "cube">("triangle");
  const [isAnimating, setIsAnimating] = useState(false);
  const [translation, setTranslation] = useState({ x: 0, y: 0 });
  const [backgroundColor, setBackgroundColor] = useState({ r: 0, g: 0, b: 0, a: 1 });
  const [wireframeMode, setWireframeMode] = useState(false);
  const [cameraDistance, setCameraDistance] = useState(3);
  const [cameraAngles, setCameraAngles] = useState({ x: 0, y: 0 });

  useEffect(() => {
    let animationId: number;

    const loadWasm = async () => {
      try {
        setStatus("Loading WASM script...");
        
        // Create and load the WASM script
        const script = document.createElement('script');
        script.src = '/wasm-simple/graphics_engine.js';
        
        await new Promise((resolve, reject) => {
          script.onload = () => {
            console.log('WASM script loaded');
            // Check all possible locations for wasm_bindgen
            console.log('window.wasm_bindgen:', typeof window.wasm_bindgen);
            console.log('globalThis.wasm_bindgen:', typeof (globalThis as any).wasm_bindgen);
            
            // Since wasm_bindgen is declared with 'let', we need to wait for it to be assigned
            // Let's try accessing it from the global scope after a delay
            setTimeout(() => {
              try {
                // Try to access it via eval (it should be in global scope after script execution)
                const wasmBindgenCheck = eval('typeof wasm_bindgen');
                console.log('eval wasm_bindgen type:', wasmBindgenCheck);
                
                if (wasmBindgenCheck === 'function') {
                  (window as any).wasm_bindgen = eval('wasm_bindgen');
                  console.log('Successfully captured wasm_bindgen');
                }
              } catch (e) {
                console.log('Could not eval wasm_bindgen:', e);
              }
              resolve();
            }, 200);
          };
          script.onerror = reject;
          document.head.appendChild(script);
        });
        
        setStatus("Checking WASM availability...");
        
        // Now try to get wasm_bindgen
        let wasmBindgen = (window as any).wasm_bindgen;
        
        if (typeof wasmBindgen !== 'function') {
          throw new Error(`wasm_bindgen still not available after loading. Type: ${typeof wasmBindgen}`);
        }
        
        setStatus("Initializing WASM module...");
        await wasmBindgen();
        
        setStatus("Creating graphics engine...");
        
        if (canvasRef.current && wasmBindgen.GraphicsEngine) {
          const engine = new wasmBindgen.GraphicsEngine("rust-canvas");
          engineRef.current = engine;
          
          // Set initial values and render
          engine.set_rotation(rotation);
          engine.set_scale(scale);
          engine.set_color(color.r, color.g, color.b);
          engine.set_translation(translation.x, translation.y);
          engine.set_background_color(backgroundColor.r, backgroundColor.g, backgroundColor.b, backgroundColor.a);
          engine.set_wireframe_mode(wireframeMode);
          engine.set_camera_distance(cameraDistance);
          engine.set_camera_angles(cameraAngles.x, cameraAngles.y);
          engine.render();
          
          setStatus("Rust + WebAssembly Ready! 🦀✨");
        } else {
          throw new Error("Failed to create GraphicsEngine or canvas not found");
        }
      } catch (error) {
        console.error("WASM loading error:", error);
        setStatus(`Error: ${error.message}`);
      }
    };

    loadWasm();

    return () => {
      if (animationId) {
        cancelAnimationFrame(animationId);
      }
    };
  }, []);

  useEffect(() => {
    if (engineRef.current) {
      try {
        engineRef.current.set_rotation(rotation);
        engineRef.current.set_scale(scale);
        engineRef.current.set_color(color.r, color.g, color.b);
        engineRef.current.set_translation(translation.x, translation.y);
        engineRef.current.set_background_color(backgroundColor.r, backgroundColor.g, backgroundColor.b, backgroundColor.a);
        engineRef.current.set_wireframe_mode(wireframeMode);
        engineRef.current.set_camera_distance(cameraDistance);
        engineRef.current.set_camera_angles(cameraAngles.x, cameraAngles.y);
        
        if (shape === "cube") {
          engineRef.current.render_cube();
        } else {
          engineRef.current.render();
        }
      } catch (error) {
        console.error("Render error:", error);
      }
    }
  }, [rotation, scale, color, shape, translation, backgroundColor, wireframeMode, cameraDistance, cameraAngles]);

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
        backgroundColor: status.includes("Ready") ? "#d4edda" : status.startsWith("Error") ? "#f8d7da" : "#d1ecf1",
        border: `1px solid ${status.includes("Ready") ? "#c3e6cb" : status.startsWith("Error") ? "#f5c6cb" : "#bee5eb"}`,
        borderRadius: "4px",
        color: status.startsWith("Error") ? "#721c24" : "#155724"
      }}>
        <strong>Status:</strong> {status}
      </p>
      
      <canvas
        ref={canvasRef}
        id="rust-canvas"
        width={600}
        height={600}
        style={{
          border: "2px solid #333",
          margin: "20px 0",
          display: "block",
          backgroundColor: "#111"
        }}
      />
      
      {engineRef.current && (
        <div style={{ 
          display: "grid", 
          gridTemplateColumns: "1fr 1fr 1fr", 
          gap: "30px",
          maxWidth: "1200px"
        }}>
          <div>
            <h3>🔄 Transform</h3>
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                Rotation: {rotation.toFixed(2)} rad
              </label>
              <input
                type="range"
                min="0"
                max={Math.PI * 2}
                step="0.01"
                value={rotation}
                onInput={(e) => setRotation(parseFloat(e.currentTarget.value))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                Scale: {scale.toFixed(2)}x
              </label>
              <input
                type="range"
                min="0.1"
                max="3"
                step="0.01"
                value={scale}
                onInput={(e) => setScale(parseFloat(e.currentTarget.value))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>Translation X: {translation.x.toFixed(2)}</label>
              <input
                type="range"
                min="-2"
                max="2"
                step="0.01"
                value={translation.x}
                onInput={(e) => setTranslation(prev => ({ ...prev, x: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>Translation Y: {translation.y.toFixed(2)}</label>
              <input
                type="range"
                min="-2"
                max="2"
                step="0.01"
                value={translation.y}
                onInput={(e) => setTranslation(prev => ({ ...prev, y: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "8px", fontWeight: "bold" }}>Shape:</label>
              <div style={{ display: "flex", gap: "10px" }}>
                <button
                  onClick={() => setShape("triangle")}
                  style={{
                    padding: "10px 20px",
                    backgroundColor: shape === "triangle" ? "#007bff" : "#f8f9fa",
                    color: shape === "triangle" ? "white" : "#333",
                    border: "1px solid #dee2e6",
                    borderRadius: "4px",
                    cursor: "pointer",
                    fontWeight: shape === "triangle" ? "bold" : "normal"
                  }}
                >
                  🔺 Triangle
                </button>
                <button
                  onClick={() => setShape("cube")}
                  style={{
                    padding: "10px 20px",
                    backgroundColor: shape === "cube" ? "#007bff" : "#f8f9fa",
                    color: shape === "cube" ? "white" : "#333",
                    border: "1px solid #dee2e6",
                    borderRadius: "4px",
                    cursor: "pointer",
                    fontWeight: shape === "cube" ? "bold" : "normal"
                  }}
                >
                  🧊 Cube
                </button>
              </div>
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "flex", alignItems: "center", gap: "10px", fontWeight: "bold" }}>
                <input
                  type="checkbox"
                  checked={wireframeMode}
                  onChange={(e) => setWireframeMode(e.currentTarget.checked)}
                />
                📐 Wireframe Mode
              </label>
            </div>
          </div>
          
          <div>
            <h3>🎨 Color</h3>
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                🔴 Red: {color.r.toFixed(2)}
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={color.r}
                onInput={(e) => setColor(prev => ({ ...prev, r: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                🟢 Green: {color.g.toFixed(2)}
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={color.g}
                onInput={(e) => setColor(prev => ({ ...prev, g: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                🔵 Blue: {color.b.toFixed(2)}
              </label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={color.b}
                onInput={(e) => setColor(prev => ({ ...prev, b: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>Background Alpha: {backgroundColor.a.toFixed(2)}</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={backgroundColor.a}
                onInput={(e) => setBackgroundColor(prev => ({ ...prev, a: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <button
              onClick={() => setIsAnimating(!isAnimating)}
              style={{
                width: "100%",
                padding: "12px",
                backgroundColor: isAnimating ? "#dc3545" : "#28a745",
                color: "white",
                border: "none",
                borderRadius: "4px",
                cursor: "pointer",
                fontSize: "16px",
                fontWeight: "bold"
              }}
            >
              {isAnimating ? "⏸️ Stop Animation" : "▶️ Start Animation"}
            </button>
          </div>
          
          <div>
            <h3>🎥 Camera & Environment</h3>
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>🔍 Camera Distance: {cameraDistance.toFixed(2)}</label>
              <input
                type="range"
                min="1"
                max="10"
                step="0.1"
                value={cameraDistance}
                onInput={(e) => setCameraDistance(parseFloat(e.currentTarget.value))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>📐 Camera Angle X: {cameraAngles.x.toFixed(2)} rad</label>
              <input
                type="range"
                min={-Math.PI}
                max={Math.PI}
                step="0.01"
                value={cameraAngles.x}
                onInput={(e) => setCameraAngles(prev => ({ ...prev, x: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>🔄 Camera Angle Y: {cameraAngles.y.toFixed(2)} rad</label>
              <input
                type="range"
                min={-Math.PI}
                max={Math.PI}
                step="0.01"
                value={cameraAngles.y}
                onInput={(e) => setCameraAngles(prev => ({ ...prev, y: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>🔴 Background Red: {backgroundColor.r.toFixed(2)}</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={backgroundColor.r}
                onInput={(e) => setBackgroundColor(prev => ({ ...prev, r: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>🟢 Background Green: {backgroundColor.g.toFixed(2)}</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={backgroundColor.g}
                onInput={(e) => setBackgroundColor(prev => ({ ...prev, g: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
            
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>🔵 Background Blue: {backgroundColor.b.toFixed(2)}</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={backgroundColor.b}
                onInput={(e) => setBackgroundColor(prev => ({ ...prev, b: parseFloat(e.currentTarget.value) }))}
                style={{ width: "100%" }}
              />
            </div>
          </div>
        </div>
      )}
      
      {engineRef.current && (
        <div style={{ marginTop: "30px", textAlign: "center" }}>
          <button
            onClick={() => {
              setRotation(0);
              setScale(1);
              setColor({ r: 1, g: 0.5, b: 0.2 });
              setShape("triangle");
              setIsAnimating(false);
              setTranslation({ x: 0, y: 0 });
              setBackgroundColor({ r: 0, g: 0, b: 0, a: 1 });
              setWireframeMode(false);
              setCameraDistance(3);
              setCameraAngles({ x: 0, y: 0 });
            }}
            style={{
              padding: "12px 24px",
              backgroundColor: "#6c757d",
              color: "white",
              border: "none",
              borderRadius: "4px",
              cursor: "pointer",
              fontSize: "16px"
            }}
          >
            🔄 Reset All
          </button>
        </div>
      )}
      
      <div style={{ marginTop: "20px", fontSize: "14px", color: "#666" }}>
        <p>
          <strong>Tech Stack:</strong> Rust → WebAssembly → WebGL → TypeScript → Preact → Deno Fresh
        </p>
      </div>
    </div>
  );
}