# Path of Goodies

A 2D top-down adventure game built in Rust using the Bevy game engine.

## Overview

Path of Goodies is a cross-platform 2D game featuring:
- Top-down tilted perspective gameplay
- Native Rust implementation
- Web, Desktop, and Mobile support (web-first)
- Modern ECS architecture with Bevy

## Quick Start

### Prerequisites

- Rust 1.85+ (install from [rustup.rs](https://rustup.rs/))
- For web builds: `trunk` (install with `cargo install trunk`)

### Running Locally (Desktop)

```bash
cargo run
```

### Running on Web (WASM)

```bash
# Install trunk if you haven't already
cargo install trunk

# Serve the web version
trunk serve

# Then open http://localhost:8080 in your browser
```

### Building for Release

**Desktop:**
```bash
cargo build --release
```

**Web:**
```bash
trunk build --release
```

The web build will be in the `dist/` directory.

## Project Structure

```
path-of-goodies/
├── assets/          # Game assets (sprites, audio, maps)
├── src/             # Rust source code
│   └── main.rs      # Entry point
├── Cargo.toml       # Rust dependencies
├── index.html       # Web entry point
├── Trunk.toml       # Web build configuration
├── ROADMAP.md       # Development roadmap
└── README.md        # This file
```

## Development Roadmap

See [ROADMAP.md](ROADMAP.md) for the complete development plan, milestones, and timeline.

## Technology Stack

- **Engine**: [Bevy](https://bevyengine.org/) - Modern, data-driven game engine
- **Language**: Rust 2021 edition
- **Web**: WebAssembly (WASM) via trunk
- **Build Tool**: Cargo with trunk for web builds

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Web (WASM) | 🚧 In Progress | Primary target, WebGL2 |
| Windows | 📋 Planned | Phase 2 |
| macOS | 📋 Planned | Phase 2 |
| Linux | 📋 Planned | Phase 2 |
| iOS | 📋 Planned | Phase 3 |
| Android | 📋 Planned | Phase 3 |

## Controls

*Coming soon - controls will be documented as gameplay systems are implemented*

## Contributing

This project is currently in early development. Contributions and feedback are welcome!

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Rust GameDev Working Group](https://gamedev.rs/)

---

**Current Status**: Phase 0 - Foundation Setup
**Last Updated**: 2025-10-21
