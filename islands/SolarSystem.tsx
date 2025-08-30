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
  const [planetNames, setPlanetNames] = useState<string[]>([]);
  const [followPlanet, setFollowPlanet] = useState<number | null>(null);
  const [canvasSize, setCanvasSize] = useState({ width: 800, height: 600 });
  const [controlsVisible, setControlsVisible] = useState(true);
  
  const updateCanvasSize = () => {
    const width = window.innerWidth - 40; // 20px padding on each side
    const height = Math.min(window.innerHeight * 0.7, 600); // 70% of viewport or 600px max
    
    setCanvasSize({ width, height });
    
    // Use setTimeout to ensure DOM has updated before resizing WebGL
    setTimeout(() => {
      if (canvasRef.current) {
        canvasRef.current.width = width;
        canvasRef.current.height = height;
        
        // Update WebGL viewport if engine exists
        if (engineRef.current && engineRef.current.resize_canvas) {
          engineRef.current.resize_canvas(width, height);
          // Force a re-render after viewport update
          if (engineRef.current.render_solar_system) {
            engineRef.current.render_solar_system();
          }
        }
      }
    }, 0);
  };
  
  useEffect(() => {
    updateCanvasSize();
    window.addEventListener('resize', updateCanvasSize);
    
    // Also use ResizeObserver to catch any layout changes
    let resizeObserver: ResizeObserver | null = null;
    if (typeof ResizeObserver !== 'undefined') {
      resizeObserver = new ResizeObserver(() => {
        updateCanvasSize();
      });
      if (document.body) {
        resizeObserver.observe(document.body);
      }
    }
    
    return () => {
      window.removeEventListener('resize', updateCanvasSize);
      if (resizeObserver) {
        resizeObserver.disconnect();
      }
    };
  }, []);
  
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
          
          // Update canvas size now that engine is ready
          updateCanvasSize();
          
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
      // Camera angles work with planet following - they control viewing angle
      engineRef.current.set_camera_angles(cameraAngles.x, cameraAngles.y);
      engineRef.current.set_time_scale(timeScale);
    }
  }, [wireframeMode, cameraDistance, cameraAngles, timeScale]);

  useEffect(() => {
    if (engineRef.current && engineRef.current.set_follow_planet) {
      // Set planet to follow in WASM (-1 means no planet)
      engineRef.current.set_follow_planet(followPlanet === null ? -1 : followPlanet);
    }
  }, [followPlanet]);
  
  // Update canvas size when controls visibility changes, as it may affect layout
  useEffect(() => {
    if (engineRef.current) {
      setTimeout(() => updateCanvasSize(), 10);
    }
  }, [controlsVisible]);
  
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
        
        // Planet following is now handled internally by WASM
        // No need for frequent boundary crossings!
        
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
  }, [isAnimating, followPlanet]);
  
  return (
    <div style={{ fontFamily: "system-ui", backgroundColor: "#0a0a0f", color: "#fff", minHeight: "100vh", position: "relative" }}>
      <h1 style={{ textAlign: "center", padding: "20px 20px 10px", margin: 0 }}>🌌 Solar System Model</h1>
      <p style={{ 
        textAlign: "center",
        padding: "10px",
        margin: "0 20px 20px",
        backgroundColor: status.includes("Ready") ? "#1a3d1a" : status.startsWith("Error") ? "#4a1a1a" : "#1a2d3d",
        borderRadius: "4px"
      }}>
        <strong>Status:</strong> {status}
      </p>
      
      <div style={{ position: "relative", width: "100%", height: canvasSize.height + "px" }}>
        <canvas
          ref={canvasRef}
          id="solar-canvas"
          width={canvasSize.width}
          height={canvasSize.height}
          style={{
            display: "block",
            margin: "0 auto",
            border: "2px solid #333",
            backgroundColor: "#000",
            borderRadius: "8px",
            maxWidth: "100%"
          }}
        />
      
        {engineRef.current && (
          <>
            <button
              onClick={() => setControlsVisible(!controlsVisible)}
              style={{
                position: "absolute",
                top: "10px",
                right: "10px",
                zIndex: 20,
                padding: "8px 12px",
                backgroundColor: "rgba(26, 26, 46, 0.95)",
                color: "white",
                border: "1px solid rgba(255, 255, 255, 0.2)",
                borderRadius: "6px",
                cursor: "pointer",
                fontSize: "14px",
                backdropFilter: "blur(10px)"
              }}
            >
              {controlsVisible ? "👁️ Hide Controls" : "⚙️ Show Controls"}
            </button>
            
            {controlsVisible && (
              <div style={{
                position: "absolute",
                top: "10px",
                left: "10px",
                right: "60px",
                display: "flex",
                gap: "10px",
                flexWrap: "wrap",
                zIndex: 10
              }}>
                <div style={{ 
                  backgroundColor: "rgba(26, 26, 46, 0.95)",
                  padding: "15px",
                  borderRadius: "8px",
                  backdropFilter: "blur(10px)",
                  minWidth: "200px"
                }}>
                  <h4 style={{ margin: "0 0 10px 0", fontSize: "14px" }}>⏱️ Time Controls</h4>
                  <div style={{ marginBottom: "10px" }}>
                    <label style={{ display: "block", marginBottom: "5px", fontSize: "12px" }}>
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
                      padding: "8px",
                      backgroundColor: isAnimating ? "#dc3545" : "#28a745",
                      color: "white",
                      border: "none",
                      borderRadius: "4px",
                      cursor: "pointer",
                      fontSize: "14px"
                    }}
                  >
                    {isAnimating ? "⏸️ Pause" : "▶️ Play"}
                  </button>
                </div>
                
                <div style={{ 
                  backgroundColor: "rgba(26, 26, 46, 0.95)",
                  padding: "15px",
                  borderRadius: "8px",
                  backdropFilter: "blur(10px)",
                  minWidth: "200px"
                }}>
                  <h4 style={{ margin: "0 0 10px 0", fontSize: "14px" }}>🎥 Camera</h4>
                  <div style={{ marginBottom: "10px" }}>
                    <label style={{ display: "block", marginBottom: "5px", fontSize: "12px" }}>
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
                  
                  <div style={{ marginBottom: "10px" }}>
                    <label style={{ display: "block", marginBottom: "5px", fontSize: "12px" }}>
                      Vertical: {(cameraAngles.x * 180 / Math.PI).toFixed(0)}°
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
                  
                  <div style={{ marginBottom: "10px" }}>
                    <label style={{ display: "block", marginBottom: "5px", fontSize: "12px" }}>
                      Horizontal: {(cameraAngles.y * 180 / Math.PI).toFixed(0)}°
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
                
                <div style={{ 
                  backgroundColor: "rgba(26, 26, 46, 0.95)",
                  padding: "15px",
                  borderRadius: "8px",
                  backdropFilter: "blur(10px)",
                  minWidth: "200px"
                }}>
                  <h4 style={{ margin: "0 0 10px 0", fontSize: "14px" }}>🎯 Follow Planet</h4>
                  <select
                    value={followPlanet === null ? "" : followPlanet.toString()}
                    onChange={(e) => {
                      const value = e.currentTarget.value;
                      setFollowPlanet(value === "" ? null : parseInt(value));
                    }}
                    style={{
                      width: "100%",
                      padding: "8px",
                      backgroundColor: "rgba(42, 63, 90, 0.8)",
                      color: "white",
                      border: "1px solid rgba(255, 255, 255, 0.2)",
                      borderRadius: "4px",
                      fontSize: "14px",
                      cursor: "pointer",
                      marginBottom: "10px"
                    }}
                  >
                    <option value="">Free Camera</option>
                    {planetNames.map((name, index) => (
                      <option key={index} value={index}>
                        Follow {name}
                      </option>
                    ))}
                  </select>
                  
                  <button
                    onClick={() => {
                      setTimeScale(1);
                      setCameraDistance(5);
                      setCameraAngles({ x: -0.3, y: 0 });
                      setWireframeMode(false);
                      setFollowPlanet(null);
                    }}
                    style={{
                      width: "100%",
                      padding: "8px",
                      backgroundColor: "rgba(108, 117, 125, 0.8)",
                      color: "white",
                      border: "none",
                      borderRadius: "4px",
                      cursor: "pointer",
                      fontSize: "14px"
                    }}
                  >
                    🔄 Reset View
                  </button>
                </div>
              </div>
            )}
          </>
        )}
      </div>
      
      <div style={{ marginTop: "30px", textAlign: "center", fontSize: "14px", color: "#888" }}>
        <p>Built with Rust 🦀 + WebAssembly + WebGL</p>
        <p style={{ fontSize: "12px", marginTop: "10px" }}>
          Note: Sizes and distances are not to scale for better visualization
        </p>
      </div>
    </div>
  );
}