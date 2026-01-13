# UCM - Universal Calendar Manager

## Project Skill Tracker

---

## Overview

MCP server providing date/time calculations for Claude Desktop. Addresses LLM limitations with temporal reasoning and date arithmetic. Built in Rust using the `rmcp` SDK.

**Design Doc**: [UCM_DESIGN.md](UCM_DESIGN.md)

---

## Current Status

**Build #**: 1
**Phase**: Core Complete
**Last Updated**: 2026-01-13

### Summary

All 6 MCP tools implemented and working. 24 unit tests passing. Release binary built successfully.

**Tools Implemented:**
- `ucm_now` - Get current date/time
- `ucm_parse` - Parse natural language dates
- `ucm_diff` - Calculate date differences
- `ucm_add` - Date arithmetic
- `ucm_convert` - Duration conversions
- `ucm_info` - Detailed date information

---

## Todo List

### Phase 1: Project Setup
- [x] Initialize Cargo project with dependencies
- [x] Set up project structure (src/tools, src/parser, src/types)
- [x] Verify rmcp and two_timer dependencies work

### Phase 2: Core Implementation
- [x] Implement natural language parser wrapper (src/parser/natural.rs)
- [x] Implement duration breakdown types (src/types/duration.rs)
- [x] Implement response types (src/types/responses.rs)

### Phase 3: MCP Tools
- [x] Implement `ucm_now` - Get current datetime
- [x] Implement `ucm_parse` - Parse natural language dates
- [x] Implement `ucm_diff` - Calculate date differences
- [x] Implement `ucm_add` - Date arithmetic
- [x] Implement `ucm_convert` - Duration unit conversion
- [x] Implement `ucm_info` - Rich date information

### Phase 4: Integration
- [x] Implement MCP server handler (main.rs)
- [x] Write unit tests (24 tests)
- [ ] Test with Claude Desktop
- [ ] Create GitHub repository

### Phase 5: Polish
- [ ] Error handling refinement
- [ ] Documentation
- [ ] Release build optimization

---

## Lessons Learned

1. **rmcp API Evolution**: The design doc specified `rmcp = "0.1"` but the published crate had significant API changes. The git version from `https://github.com/modelcontextprotocol/rust-sdk` works better and has current documentation.

2. **Tool Parameter Handling**: Use `Parameters<T>` wrapper from `rmcp::handler::server::wrapper::Parameters` for tool parameters. The parameter struct must derive `Deserialize` and `schemars::JsonSchema`.

3. **Tool Router Pattern**: The server struct needs a `tool_router: ToolRouter<Self>` field, initialized with `Self::tool_router()` in the constructor. Use `#[tool_router]` on the impl block with tools and `#[tool_handler]` on the ServerHandler impl.

4. **Return Types**: Tool methods can return simple types like `String` directly - the framework converts them to `CallToolResult` automatically.

---

## Build Instructions

### Prerequisites
- Rust toolchain (rustup recommended)
- Cargo

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Test MCP Interface
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/ucm.exe
```

### Binary Location
- Debug: `./target/debug/ucm.exe`
- Release: `./target/release/ucm.exe`

### Claude Desktop Configuration
Add to `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "ucm": {
      "command": "D:/Projects/UCM/target/release/ucm.exe",
      "args": []
    }
  }
}
```

---

## Build History

| Build # | Date | Notes |
|---------|------|-------|
| 0 | 2026-01-13 | Project initialized, design doc complete |
| 1 | 2026-01-13 | All 6 tools implemented, 24 tests passing, release binary built |

---

## Git/GitHub

### Initial Setup
```bash
git init
git add .
git commit -m "Initial commit: UCM project with design doc"
git branch -M main
git remote add origin <your-github-repo-url>
git push -u origin main
```

### After Each Build
```bash
git add .
git commit -m "Build #N: <description>"
git push
```

---

*Skill.md maintained by Claude Code*
