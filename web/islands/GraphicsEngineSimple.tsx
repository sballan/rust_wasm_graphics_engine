import { useEffect, useRef, useState } from "preact/hooks";

interface GraphicsEngineInstance {
  set_rotation(rotation: number): void;
  set_scale(scale: number): void;
  set_color(r: number, g: number, b: number): void;
  render(): void;
  render_cube(): void;
}

export default function GraphicsEngineSimple() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<GraphicsEngineInstance | null>(null);
  const animationRef = useRef<number>(0);
  
  const [rotation, setRotation] = useState(0);
  const [scale, setScale] = useState(1);
  const [color, setColor] = useState({ r: 1, g: 1, b: 1 });
  const [shape, setShape] = useState<"triangle" | "cube">("triangle");
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    const loadWasm = async () => {
      try {
        const wasm = await import("/wasm/graphics_engine.js");
        await wasm.default();
        
        if (canvasRef.current) {
          engineRef.current = new wasm.GraphicsEngine("graphics-canvas");
          renderFrame();
        }
      } catch (error) {
        console.error("Failed to load WASM module:", error);
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
      engineRef.current.set_rotation(rotation);
      engineRef.current.set_scale(scale);
      engineRef.current.set_color(color.r, color.g, color.b);
      renderFrame();
    }
  }, [rotation, scale, color, shape]);

  useEffect(() => {
    if (isAnimating) {
      animate();
    } else if (animationRef.current) {
      cancelAnimationFrame(animationRef.current);
      animationRef.current = 0;
    }
  }, [isAnimating]);

  const renderFrame = () => {
    if (engineRef.current) {
      if (shape === "cube") {
        engineRef.current.render_cube();
      } else {
        engineRef.current.render();
      }
    }
  };

  const animate = () => {
    setRotation((prev) => (prev + 0.02) % (Math.PI * 2));
    animationRef.current = requestAnimationFrame(animate);
  };

  const handleColorChange = (component: "r" | "g" | "b", value: number) => {
    setColor((prev) => ({ ...prev, [component]: value }));
  };

  return (
    <div style={{
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      gap: "20px",
      padding: "20px"
    }}>
      <h1 style={{ fontSize: "24px", fontWeight: "bold" }}>WebAssembly Graphics Engine</h1>
      
      <canvas
        ref={canvasRef}
        id="graphics-canvas"
        width={600}
        height={600}
        style={{
          border: "2px solid #ccc",
          borderRadius: "8px",
          boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)"
        }}
      />

      <div style={{
        display: "grid",
        gridTemplateColumns: "1fr 1fr",
        gap: "20px",
        width: "100%",
        maxWidth: "600px"
      }}>
        <div style={{ display: "flex", flexDirection: "column", gap: "15px" }}>
          <div>
            <label style={{ display: "block", fontSize: "14px", marginBottom: "8px" }}>
              Rotation: {rotation.toFixed(2)}
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

          <div>
            <label style={{ display: "block", fontSize: "14px", marginBottom: "8px" }}>
              Scale: {scale.toFixed(2)}
            </label>
            <input
              type="range"
              min="0.1"
              max="2"
              step="0.01"
              value={scale}
              onInput={(e) => setScale(parseFloat(e.currentTarget.value))}
              style={{ width: "100%" }}
            />
          </div>

          <div>
            <label style={{ display: "block", fontSize: "14px", marginBottom: "8px" }}>Shape</label>
            <div style={{ display: "flex", gap: "8px" }}>
              <button
                onClick={() => setShape("triangle")}
                style={{
                  padding: "8px 16px",
                  borderRadius: "4px",
                  border: "1px solid #ccc",
                  backgroundColor: shape === "triangle" ? "#3b82f6" : "#f9f9f9",
                  color: shape === "triangle" ? "white" : "black",
                  cursor: "pointer"
                }}
              >
                Triangle
              </button>
              <button
                onClick={() => setShape("cube")}
                style={{
                  padding: "8px 16px",
                  borderRadius: "4px",
                  border: "1px solid #ccc",
                  backgroundColor: shape === "cube" ? "#3b82f6" : "#f9f9f9",
                  color: shape === "cube" ? "white" : "black",
                  cursor: "pointer"
                }}
              >
                Cube
              </button>
            </div>
          </div>
        </div>

        <div style={{ display: "flex", flexDirection: "column", gap: "15px" }}>
          <div>
            <label style={{ display: "block", fontSize: "14px", marginBottom: "8px" }}>
              Red: {color.r.toFixed(2)}
            </label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              value={color.r}
              onInput={(e) =>
                handleColorChange("r", parseFloat(e.currentTarget.value))
              }
              style={{ width: "100%" }}
            />
          </div>

          <div>
            <label style={{ display: "block", fontSize: "14px", marginBottom: "8px" }}>
              Green: {color.g.toFixed(2)}
            </label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              value={color.g}
              onInput={(e) =>
                handleColorChange("g", parseFloat(e.currentTarget.value))
              }
              style={{ width: "100%" }}
            />
          </div>

          <div>
            <label style={{ display: "block", fontSize: "14px", marginBottom: "8px" }}>
              Blue: {color.b.toFixed(2)}
            </label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.01"
              value={color.b}
              onInput={(e) =>
                handleColorChange("b", parseFloat(e.currentTarget.value))
              }
              style={{ width: "100%" }}
            />
          </div>

          <button
            onClick={() => setIsAnimating(!isAnimating)}
            style={{
              width: "100%",
              padding: "8px 16px",
              borderRadius: "4px",
              border: "1px solid #ccc",
              backgroundColor: isAnimating ? "#ef4444" : "#10b981",
              color: "white",
              cursor: "pointer",
              fontSize: "14px",
              fontWeight: "500"
            }}
          >
            {isAnimating ? "Stop Animation" : "Start Animation"}
          </button>
        </div>
      </div>

      <div style={{ display: "flex", gap: "16px" }}>
        <button
          onClick={() => {
            setRotation(0);
            setScale(1);
            setColor({ r: 1, g: 1, b: 1 });
          }}
          style={{
            padding: "8px 16px",
            backgroundColor: "#6b7280",
            color: "white",
            borderRadius: "4px",
            border: "none",
            cursor: "pointer"
          }}
        >
          Reset
        </button>
      </div>
    </div>
  );
}