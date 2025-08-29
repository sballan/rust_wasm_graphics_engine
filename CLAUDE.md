# Claude Development Notes

## Important Workflow Reminders

### ALWAYS Test After Changes
After making any code changes that affect the WASM module or web interface:

1. **Rebuild WASM** (if Rust code changed):
   ```bash
   cd engine && wasm-pack build --target web --out-dir ../web/static
   ```

2. **Check server status**:
   ```bash
   curl -I http://localhost:8000/ || curl -I http://localhost:8001/ || curl -I http://localhost:8002/
   ```

3. **Test the actual functionality** by visiting the page in browser and checking console

### Current Project Status
- **Solar System Model**: Working with full 3D camera controls
- **Server**: Deno Fresh development server (usually port 8000-8002)
- **WASM Build Location**: `web/static/`
- **Main Routes**:
  - `/` - Home page with demo links
  - `/solar-system` - Interactive solar system model

### Common Commands
```bash
# Start dev server
cd web && deno task start

# Build WASM module
cd engine && wasm-pack build --target web --out-dir ../web/static

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
- **Engine**: Rust/WASM graphics engine with WebGL
- **Web**: Deno Fresh frontend with Preact/TypeScript
- **Solar System**: 9 celestial bodies with orbital mechanics
- **Camera**: 3D orbital camera with distance/rotation controls