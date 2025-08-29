import { useEffect, useRef, useState } from "preact/hooks";

export default function SimpleGraphics() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<any>(null);
  const [status, setStatus] = useState("Initializing...");
  const [rotation, setRotation] = useState(0);

  useEffect(() => {
    const initWasm = async () => {
      try {
        setStatus("Loading WASM...");
        
        // Load WASM using instantiateStreaming for better performance
        const wasmModule = await WebAssembly.instantiateStreaming(
          fetch('/graphics_engine_bg.wasm'),
          {}
        );
        
        setStatus("Creating graphics context...");
        
        // Get canvas context directly and try simple WebGL rendering
        const canvas = canvasRef.current;
        if (!canvas) {
          throw new Error("Canvas not found");
        }
        
        const gl = canvas.getContext('webgl');
        if (!gl) {
          throw new Error("WebGL not supported");
        }
        
        // Simple WebGL test - clear to red
        gl.clearColor(1.0, 0.5, 0.2, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        
        setStatus("WebGL test successful! Creating triangle...");
        
        // Create a simple triangle shader
        const vertexShaderSource = `
          attribute vec2 a_position;
          uniform float u_rotation;
          void main() {
            float cos_r = cos(u_rotation);
            float sin_r = sin(u_rotation);
            mat2 rotation = mat2(cos_r, -sin_r, sin_r, cos_r);
            vec2 rotated = rotation * a_position;
            gl_Position = vec4(rotated, 0.0, 1.0);
          }
        `;
        
        const fragmentShaderSource = `
          precision mediump float;
          uniform vec3 u_color;
          void main() {
            gl_FragColor = vec4(u_color, 1.0);
          }
        `;
        
        // Compile shaders
        const vertexShader = gl.createShader(gl.VERTEX_SHADER)!;
        gl.shaderSource(vertexShader, vertexShaderSource);
        gl.compileShader(vertexShader);
        
        const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER)!;
        gl.shaderSource(fragmentShader, fragmentShaderSource);
        gl.compileShader(fragmentShader);
        
        // Create program
        const program = gl.createProgram()!;
        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);
        gl.useProgram(program);
        
        // Triangle vertices
        const vertices = new Float32Array([
          0.0,  0.8,
         -0.8, -0.8,
          0.8, -0.8
        ]);
        
        const buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);
        
        const positionLocation = gl.getAttribLocation(program, 'a_position');
        gl.enableVertexAttribArray(positionLocation);
        gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);
        
        const rotationLocation = gl.getUniformLocation(program, 'u_rotation');
        const colorLocation = gl.getUniformLocation(program, 'u_color');
        
        // Create a simple graphics engine object
        engineRef.current = {
          gl,
          program,
          rotationLocation,
          colorLocation,
          render: (rot: number, color: [number, number, number]) => {
            gl.clearColor(0.0, 0.0, 0.0, 1.0);
            gl.clear(gl.COLOR_BUFFER_BIT);
            gl.uniform1f(rotationLocation, rot);
            gl.uniform3f(colorLocation, color[0], color[1], color[2]);
            gl.drawArrays(gl.TRIANGLES, 0, 3);
          }
        };
        
        // Initial render
        engineRef.current.render(0, [1.0, 0.5, 0.2]);
        
        setStatus("Ready! Pure WebGL triangle rendered ✨");
        
      } catch (error) {
        console.error("Initialization error:", error);
        setStatus(`Error: ${error.message}`);
      }
    };

    initWasm();
  }, []);

  useEffect(() => {
    if (engineRef.current) {
      engineRef.current.render(rotation, [1.0, 0.5, 0.2]);
    }
  }, [rotation]);

  return (
    <div style={{ padding: "20px", fontFamily: "system-ui" }}>
      <h1>🔺 Simple WebGL Triangle (Fallback)</h1>
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
        <div>
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
          
          <p style={{ fontSize: "14px", color: "#666", marginTop: "20px" }}>
            This is a fallback implementation using pure WebGL since WASM loading had issues.
            The triangle should rotate as you move the slider.
          </p>
        </div>
      )}
    </div>
  );
}