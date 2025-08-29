import { useEffect, useRef, useState } from "preact/hooks";

interface GraphicsEngineInstance {
  set_rotation(rotation: number): void;
  set_scale(scale: number): void;
  set_color(r: number, g: number, b: number): void;
  render(): void;
  render_cube(): void;
}

export default function GraphicsEngine() {
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
    <div class="flex flex-col items-center gap-6 p-8">
      <h1 class="text-3xl font-bold">WebAssembly Graphics Engine</h1>

      <canvas
        ref={canvasRef}
        id="graphics-canvas"
        width={600}
        height={600}
        class="border-2 border-gray-300 rounded-lg shadow-lg"
      />

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 w-full max-w-2xl">
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">
              Rotation: {rotation.toFixed(2)}
            </label>
            <input
              type="range"
              min="0"
              max={Math.PI * 2}
              step="0.01"
              value={rotation}
              onInput={(e) => setRotation(parseFloat(e.currentTarget.value))}
              class="w-full"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">
              Scale: {scale.toFixed(2)}
            </label>
            <input
              type="range"
              min="0.1"
              max="2"
              step="0.01"
              value={scale}
              onInput={(e) => setScale(parseFloat(e.currentTarget.value))}
              class="w-full"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">Shape</label>
            <div class="flex gap-2">
              <button
                onClick={() => setShape("triangle")}
                class={`px-4 py-2 rounded ${shape === "triangle"
                    ? "bg-blue-500 text-white"
                    : "bg-gray-200"
                  }`}
              >
                Triangle
              </button>
              <button
                onClick={() => setShape("cube")}
                class={`px-4 py-2 rounded ${shape === "cube"
                    ? "bg-blue-500 text-white"
                    : "bg-gray-200"
                  }`}
              >
                Cube
              </button>
            </div>
          </div>
        </div>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium mb-2">
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
              class="w-full"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">
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
              class="w-full"
            />
          </div>

          <div>
            <label class="block text-sm font-medium mb-2">
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
              class="w-full"
            />
          </div>

          <button
            onClick={() => setIsAnimating(!isAnimating)}
            class={`w-full px-4 py-2 rounded font-medium ${isAnimating
                ? "bg-red-500 text-white"
                : "bg-green-500 text-white"
              }`}
          >
            {isAnimating ? "Stop Animation" : "Start Animation"}
          </button>
        </div>
      </div>

      <div class="flex gap-4">
        <button
          onClick={() => {
            setRotation(0);
            setScale(1);
            setColor({ r: 1, g: 1, b: 1 });
          }}
          class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600"
        >
          Reset
        </button>
      </div>
    </div>
  );
}