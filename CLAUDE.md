# Claude Development Notes

## Important Workflow Reminders

### ALWAYS Test After Changes
After making any code changes that affect the WASM module or web interface:

1. **Rebuild WASM** (if Rust code changed):
   ```bash
   deno task build:wasm
   ```
   Or manually:
   ```bash
   wasm-pack build --target web --out-dir static
   ```

2. **Check server status**:
   ```bash
   curl -I http://localhost:8000/ || curl -I http://localhost:8001/ || curl -I http://localhost:8002/
   ```

3. **Test the actual functionality** by visiting the page in browser and checking console

### Current Project Status
- **Solar System Model**: Working with full 3D camera controls
- **Server**: Deno Fresh development server (usually port 8000-8002)
- **WASM Build Location**: `static/` (root level)
- **Rust Target Location**: `engine/target/`
- **Main Routes**:
  - `/` - Home page with demo links
  - `/solar-system` - Interactive solar system model

### Common Commands
```bash
# Start dev server
deno task start

# Build WASM module
deno task build:wasm

# Build everything (WASM + Fresh)
deno task build:all

# Production preview
deno task preview

# Check server health
curl -I http://localhost:8000/

# Background process monitoring
# Use BashOutput tool with bash_1 ID to check server logs
```

### Debugging Checklist
When things break:
1. Check browser console for errors
2. Check server logs via BashOutput
3. Verify WASM files are being served correctly
4. Test basic HTTP response with curl
5. Rebuild WASM if Rust changes were made

### Project Architecture
- **Engine**: Rust/WASM graphics engine with WebGL (source: `engine/src/`, builds to: `engine/target/`)
- **Frontend**: Deno Fresh with Preact/TypeScript (routes: `routes/`, islands: `islands/`)
- **Static Assets**: Served from `static/` (includes WASM output)
- **Solar System**: 9 celestial bodies with orbital mechanics
- **Camera**: 3D orbital camera with distance/rotation controls

### Directory Structure
```
root/
├── engine/src/           # Rust source code
├── engine/target/        # Rust build artifacts
├── routes/               # Fresh routes
├── islands/              # Fresh islands (client components)
├── components/           # Shared components
├── static/               # Static files & WASM output
├── Cargo.toml           # Rust configuration
├── deno.json            # Deno tasks & dependencies
├── fresh.config.ts      # Fresh configuration
└── dev.ts, main.ts      # Fresh entry points
```