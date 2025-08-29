import RustWasmGraphics from "../islands/RustWasmGraphics.tsx";

export default function Home() {
  return (
    <div style={{
      minHeight: "100vh",
      backgroundColor: "#f8f9fa"
    }}>
      <div style={{ padding: "20px", textAlign: "center" }}>
        <h1>Rust WASM Graphics Demos</h1>
        <div style={{ display: "flex", gap: "20px", justifyContent: "center", marginBottom: "20px" }}>
          <a 
            href="/solar-system" 
            style={{
              padding: "15px 30px",
              backgroundColor: "#007bff",
              color: "white",
              textDecoration: "none",
              borderRadius: "8px",
              fontSize: "18px",
              fontWeight: "bold"
            }}
          >
            🌌 Solar System Model
          </a>
          <a 
            href="#basic-demo" 
            style={{
              padding: "15px 30px",
              backgroundColor: "#28a745",
              color: "white",
              textDecoration: "none",
              borderRadius: "8px",
              fontSize: "18px",
              fontWeight: "bold"
            }}
          >
            🔺 Basic Shapes Demo
          </a>
        </div>
      </div>
      <div id="basic-demo">
        <RustWasmGraphics />
      </div>
    </div>
  );
}