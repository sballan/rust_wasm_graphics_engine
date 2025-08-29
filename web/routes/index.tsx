import RustWasmGraphics from "../islands/RustWasmGraphics.tsx";

export default function Home() {
  return (
    <div style={{
      minHeight: "100vh",
      backgroundColor: "#f8f9fa"
    }}>
      <RustWasmGraphics />
    </div>
  );
}