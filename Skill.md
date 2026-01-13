# UCM - Universal Calendar Manager

## Project Skill Tracker

---

## Overview

MCP server providing date/time calculations for Claude Desktop. Addresses LLM limitations with temporal reasoning and date arithmetic. Built in Rust using the `rmcp` SDK.

**Design Doc**: [UCM_DESIGN.md](UCM_DESIGN.md)

---

## Current Status

**Build #**: 0
**Phase**: Not Started
**Last Updated**: 2026-01-13

### Summary

Project initialized. Design document complete. Implementation not yet started.

---

## Todo List

### Phase 1: Project Setup
- [ ] Initialize Cargo project with dependencies
- [ ] Set up project structure (src/tools, src/parser, src/types)
- [ ] Verify rmcp and two_timer dependencies work

### Phase 2: Core Implementation
- [ ] Implement natural language parser wrapper (src/parser/natural.rs)
- [ ] Implement duration breakdown types (src/types/duration.rs)
- [ ] Implement response types (src/types/responses.rs)

### Phase 3: MCP Tools
- [ ] Implement `ucm_now` - Get current datetime
- [ ] Implement `ucm_parse` - Parse natural language dates
- [ ] Implement `ucm_diff` - Calculate date differences
- [ ] Implement `ucm_add` - Date arithmetic
- [ ] Implement `ucm_convert` - Duration unit conversion
- [ ] Implement `ucm_info` - Rich date information

### Phase 4: Integration
- [ ] Implement MCP server handler (main.rs)
- [ ] Write integration tests
- [ ] Write parser tests
- [ ] Test with Claude Desktop

### Phase 5: Polish
- [ ] Error handling refinement
- [ ] Documentation
- [ ] Release build optimization

---

## Lessons Learned

*This section will be updated as we encounter notable discoveries, gotchas, or solutions.*

1. *(none yet)*

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
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | ./target/release/ucm
```

### Binary Location
- Debug: `./target/debug/ucm`
- Release: `./target/release/ucm`

---

## Build History

| Build # | Date | Notes |
|---------|------|-------|
| 0 | 2026-01-13 | Project initialized, design doc complete |

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
