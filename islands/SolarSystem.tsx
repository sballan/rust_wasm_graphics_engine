import { useEffect, useRef, useState } from "preact/hooks";

declare global {
  interface Window {
    wasm_bindgen: any;
    GraphicsEngine: any;
  }
}

export default function SolarSystem() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<any>(null);
  const animationRef = useRef<number>(null);
  const lastTimeRef = useRef<number>(0);
  
  const [status, setStatus] = useState("Loading...");
  const [timeScale, setTimeScale] = useState(1);
  const [isAnimating, setIsAnimating] = useState(true);
  const [wireframeMode, setWireframeMode] = useState(false);
  const [cameraDistance, setCameraDistance] = useState(5);
  const [cameraAngles, setCameraAngles] = useState({ x: -0.3, y: 0 });
  const [selectedPlanet, setSelectedPlanet] = useState<number | null>(null);
  const [planetNames, setPlanetNames] = useState<string[]>([]);
  
  useEffect(() => {
    const loadWasm = async () => {
      try {
        setStatus("Loading WASM module...");
        
        // Wait for GraphicsEngine to be available
        let GraphicsEngine;
        let attempts = 0;
        const maxAttempts = 50;
        
        while (attempts < maxAttempts) {
          GraphicsEngine = (window as any).GraphicsEngine;
          console.log(`Attempt ${attempts}: GraphicsEngine type:`, typeof GraphicsEngine);
          
          if (typeof GraphicsEngine === 'function') {
            console.log('GraphicsEngine found!');
            break;
          }
          await new Promise(resolve => setTimeout(resolve, 100));
          attempts++;
        }
        
        if (typeof GraphicsEngine !== 'function') {
          console.error('Available window properties:', Object.keys(window).filter(k => k.includes('wasm') || k.includes('Graphics')));
          throw new Error("WASM module not loaded after " + attempts + " attempts");
        }
        
        setStatus("Creating Solar System...");
        
        if (canvasRef.current && GraphicsEngine) {
          console.log('Creating GraphicsEngine instance...');
          const engine = new GraphicsEngine("solar-canvas");
          engineRef.current = engine;
          console.log('Engine created successfully');
          
          // Set initial values
          engine.set_background_color(0.02, 0.02, 0.05, 1.0);
          engine.set_wireframe_mode(wireframeMode);
          engine.set_camera_distance(cameraDistance);
          engine.set_camera_angles(cameraAngles.x, cameraAngles.y);
          engine.set_time_scale(timeScale);
          console.log('Initial values set');
          
          // Get planet names
          const names = [];
          const count = engine.get_planet_count();
          console.log('Planet count:', count);
          for (let i = 0; i < count; i++) {
            names.push(engine.get_planet_name(i));
          }
          setPlanetNames(names);
          console.log('Planet names:', names);
          
          // Initial render
          engine.render_solar_system();
          console.log('Initial render complete');
          
          setStatus("Solar System Ready! 🌍");
        } else {
          throw new Error("Canvas or GraphicsEngine not available");
        }
      } catch (error) {
        console.error("WASM loading error:", error);
        setStatus(`Error: ${error.message}`);
      }
    };
    
    loadWasm();
    
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, []);
  
  useEffect(() => {
    if (engineRef.current) {
      engineRef.current.set_wireframe_mode(wireframeMode);
      engineRef.current.set_camera_distance(cameraDistance);
      engineRef.current.set_camera_angles(cameraAngles.x, cameraAngles.y);
      engineRef.current.set_time_scale(timeScale);
    }
  }, [wireframeMode, cameraDistance, cameraAngles, timeScale]);
  
  useEffect(() => {
    if (isAnimating && engineRef.current) {
      const animate = (currentTime: number) => {
        if (lastTimeRef.current === 0) {
          lastTimeRef.current = currentTime;
        }
        
        const deltaTime = (currentTime - lastTimeRef.current) / 1000; // Convert to seconds
        lastTimeRef.current = currentTime;
        
        // Update solar system physics
        engineRef.current.update_solar_system(deltaTime);
        
        // Render the scene
        engineRef.current.render_solar_system();
        
        animationRef.current = requestAnimationFrame(animate);
      };
      
      animationRef.current = requestAnimationFrame(animate);
    } else {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
        animationRef.current = null;
        lastTimeRef.current = 0;
      }
    }
    
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
        animationRef.current = null;
      }
    };
  }, [isAnimating]);
  
  return (
    <div style={{ padding: "20px", fontFamily: "system-ui", backgroundColor: "#0a0a0f", color: "#fff", minHeight: "100vh" }}>
      <h1 style={{ textAlign: "center", marginBottom: "10px" }}>🌌 Solar System Model</h1>
      <p style={{ 
        textAlign: "center",
        padding: "10px", 
        backgroundColor: status.includes("Ready") ? "#1a3d1a" : status.startsWith("Error") ? "#4a1a1a" : "#1a2d3d",
        borderRadius: "4px",
        marginBottom: "20px"
      }}>
        <strong>Status:</strong> {status}
      </p>
      
      <canvas
        ref={canvasRef}
        id="solar-canvas"
        width={800}
        height={600}
        style={{
          display: "block",
          margin: "0 auto 30px",
          border: "2px solid #333",
          backgroundColor: "#000",
          borderRadius: "8px"
        }}
      />
      
      {engineRef.current && (
        <div style={{ maxWidth: "1000px", margin: "0 auto" }}>
          <div style={{ 
            display: "grid", 
            gridTemplateColumns: "1fr 1fr 1fr", 
            gap: "20px",
            marginBottom: "30px"
          }}>
            <div style={{ backgroundColor: "#1a1a2e", padding: "20px", borderRadius: "8px" }}>
              <h3>⏱️ Time Controls</h3>
              <div style={{ marginBottom: "15px" }}>
                <label style={{ display: "block", marginBottom: "5px" }}>
                  Speed: {timeScale.toFixed(1)}x
                </label>
                <input
                  type="range"
                  min="0"
                  max="50"
                  step="0.5"
                  value={timeScale}
                  onInput={(e) => setTimeScale(parseFloat(e.currentTarget.value))}
                  style={{ width: "100%" }}
                />
              </div>
              
              <button
                onClick={() => setIsAnimating(!isAnimating)}
                style={{
                  width: "100%",
                  padding: "10px",
                  backgroundColor: isAnimating ? "#dc3545" : "#28a745",
                  color: "white",
                  border: "none",
                  borderRadius: "4px",
                  cursor: "pointer",
                  fontSize: "16px"
                }}
              >
                {isAnimating ? "⏸️ Pause" : "▶️ Play"}
              </button>
            </div>
            
            <div style={{ backgroundColor: "#1a1a2e", padding: "20px", borderRadius: "8px" }}>
              <h3>🎥 Camera</h3>
              <div style={{ marginBottom: "15px" }}>
                <label style={{ display: "block", marginBottom: "5px" }}>
                  Distance: {cameraDistance.toFixed(1)}
                </label>
                <input
                  type="range"
                  min="1"
                  max="15"
                  step="0.1"
                  value={cameraDistance}
                  onInput={(e) => setCameraDistance(parseFloat(e.currentTarget.value))}
                  style={{ width: "100%" }}
                />
              </div>
              
              <div style={{ marginBottom: "15px" }}>
                <label style={{ display: "block", marginBottom: "5px" }}>
                  Vertical Angle: {(cameraAngles.x * 180 / Math.PI).toFixed(0)}°
                </label>
                <input
                  type="range"
                  min={-Math.PI/2}
                  max={Math.PI/2}
                  step="0.01"
                  value={cameraAngles.x}
                  onInput={(e) => setCameraAngles(prev => ({ ...prev, x: parseFloat(e.currentTarget.value) }))}
                  style={{ width: "100%" }}
                />
              </div>
              
              <div style={{ marginBottom: "15px" }}>
                <label style={{ display: "block", marginBottom: "5px" }}>
                  Horizontal Angle: {(cameraAngles.y * 180 / Math.PI).toFixed(0)}°
                </label>
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
            </div>
            
            <div style={{ backgroundColor: "#1a1a2e", padding: "20px", borderRadius: "8px" }}>
              <h3>🎨 Display</h3>
              <div style={{ marginBottom: "15px" }}>
                <label style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                  <input
                    type="checkbox"
                    checked={wireframeMode}
                    onChange={(e) => setWireframeMode(e.currentTarget.checked)}
                  />
                  Wireframe Mode
                </label>
              </div>
              
              <button
                onClick={() => {
                  setTimeScale(1);
                  setCameraDistance(5);
                  setCameraAngles({ x: -0.3, y: 0 });
                  setWireframeMode(false);
                }}
                style={{
                  width: "100%",
                  padding: "10px",
                  backgroundColor: "#6c757d",
                  color: "white",
                  border: "none",
                  borderRadius: "4px",
                  cursor: "pointer"
                }}
              >
                🔄 Reset View
              </button>
            </div>
          </div>
          
          <div style={{ backgroundColor: "#1a1a2e", padding: "20px", borderRadius: "8px" }}>
            <h3>🪐 Planets</h3>
            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(100px, 1fr))", gap: "10px" }}>
              {planetNames.map((name, index) => (
                <button
                  key={index}
                  onClick={() => setSelectedPlanet(index === selectedPlanet ? null : index)}
                  style={{
                    padding: "8px",
                    backgroundColor: index === selectedPlanet ? "#4a5f7a" : "#2a3f5a",
                    color: "white",
                    border: "1px solid #3a4f6a",
                    borderRadius: "4px",
                    cursor: "pointer",
                    fontSize: "14px"
                  }}
                >
                  {name}
                </button>
              ))}
            </div>
            {selectedPlanet !== null && (
              <div style={{ marginTop: "15px", padding: "10px", backgroundColor: "#2a3f5a", borderRadius: "4px" }}>
                <strong>Selected:</strong> {planetNames[selectedPlanet]}
              </div>
            )}
          </div>
        </div>
      )}
      
      <div style={{ marginTop: "30px", textAlign: "center", fontSize: "14px", color: "#888" }}>
        <p>Built with Rust 🦀 + WebAssembly + WebGL</p>
        <p style={{ fontSize: "12px", marginTop: "10px" }}>
          Note: Sizes and distances are not to scale for better visualization
        </p>
      </div>
    </div>
  );
}