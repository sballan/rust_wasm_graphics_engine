import SimpleWasm from "../islands/SimpleWasm.tsx";

export default function Home() {
  return (
    <div style={{
      minHeight: "100vh",
      backgroundColor: "#f8f9fa"
    }}>
      <SimpleWasm />
    </div>
  );
}