# Path of Goodies - Game Development Roadmap

## Project Overview

A 2D top-down action/adventure game built in Rust, targeting web, desktop, and mobile platforms with a unified, cross-platform codebase.

**Core Requirements:**
- 2D tilted/isometric top-down view
- Native Rust implementation
- Cross-platform: Web (WASM), Desktop (Windows/Mac/Linux), Mobile (iOS/Android)
- Initial focus: Web implementation
- All code native and portable

---

## Technology Stack

### Core Engine
**Bevy Game Engine** (v0.15+)
- ECS (Entity Component System) architecture
- Excellent WASM support for web deployment
- Cross-platform by design
- Active community and ecosystem
- 2D sprite rendering with transform hierarchies

### Platform Targets
1. **Web (Priority 1)**: WebAssembly (wasm32-unknown-unknown)
2. **Desktop (Priority 2)**: Native binaries (Windows, macOS, Linux)
3. **Mobile (Priority 3)**: iOS and Android via cargo-mobile2

### Key Dependencies
```toml
bevy = "0.15"           # Core game engine
bevy_ecs_tilemap = "*"  # Tilemap rendering (if using tilemaps)
bevy_rapier2d = "*"     # 2D physics (optional, for collisions)
rand = "*"              # Random number generation
serde = "*"             # Serialization for save files
```

### Build Tools
- `wasm-bindgen` - Web bindings
- `trunk` - WASM web bundler and dev server
- `cargo-mobile2` - Mobile deployment (future)

---

## Development Phases

### Phase 0: Foundation (Weeks 1-2)
**Goal**: Set up development environment and basic project structure

#### Milestones:
- [x] Initialize Rust project with Cargo
- [ ] Configure `.gitignore` for Rust/game assets
- [ ] Create basic Bevy app structure
- [ ] Set up web build pipeline with Trunk
- [ ] Test WASM deployment locally
- [ ] Document build and run instructions

#### Deliverables:
- `Cargo.toml` with core dependencies
- `src/main.rs` with minimal Bevy app
- `index.html` for web deployment
- `README.md` with setup instructions
- Working local web build

---

### Phase 1: Core Game Loop (Weeks 3-5)
**Goal**: Implement basic game mechanics and rendering

#### Milestones:
- [ ] Camera system with top-down view
- [ ] Player entity with sprite rendering
- [ ] Basic input handling (keyboard/touch)
- [ ] Player movement system
- [ ] Simple world/level representation
- [ ] Sprite loading and animation system

#### Technical Tasks:
- Create camera controller with zoom/pan
- Implement 8-directional movement
- Set up sprite sheet loading
- Create animation state machine
- Basic collision detection
- Frame-rate independent movement

#### Deliverables:
- Playable character on screen
- Smooth movement controls
- Working camera system
- Web demo deployed

---

### Phase 2: World & Assets (Weeks 6-8)
**Goal**: Create game world, environment, and visual assets

#### Milestones:
- [ ] Tilemap system for ground/environment
- [ ] Asset pipeline for sprites and textures
- [ ] Multiple map/level support
- [ ] Basic environmental objects (trees, rocks, etc.)
- [ ] Parallax backgrounds (optional)
- [ ] Visual effects system

#### Technical Tasks:
- Integrate bevy_ecs_tilemap or custom tile system
- Create asset loading strategy
- Implement level/map data format (JSON/RON)
- Design coordinate system for isometric/tilted view
- Optimize sprite batching for performance

#### Deliverables:
- At least 2 playable levels/maps
- Reusable asset loading system
- Map editor integration or data format

---

### Phase 3: Gameplay Systems (Weeks 9-12)
**Goal**: Implement core gameplay mechanics

#### Milestones:
- [ ] Item/collectible system ("goodies")
- [ ] Inventory management
- [ ] Enemy/NPC entities
- [ ] Basic AI for enemies
- [ ] Combat/interaction system
- [ ] Health and damage system
- [ ] Score/progression tracking

#### Technical Tasks:
- Design component architecture for entities
- Create inventory UI system
- Implement pathfinding for NPCs
- Add collision layers and masks
- Create event system for interactions
- Save/load game state

#### Deliverables:
- Working combat mechanics
- Functional inventory system
- Enemy behavior
- Game progression system

---

### Phase 4: Polish & UX (Weeks 13-15)
**Goal**: Enhance user experience and visual quality

#### Milestones:
- [ ] UI/HUD implementation
- [ ] Menu system (main menu, pause, settings)
- [ ] Sound effects and music integration
- [ ] Particle effects
- [ ] Screen transitions and animations
- [ ] Settings/preferences system

#### Technical Tasks:
- Integrate bevy_ui or custom UI
- Add audio system with bevy_kira_audio
- Create particle system
- Implement screen shake and juice
- Add touch/mobile controls overlay
- Settings persistence

#### Deliverables:
- Complete UI/UX flow
- Audio integration
- Visual polish
- Mobile-friendly controls

---

### Phase 5: Cross-Platform Deployment (Weeks 16-18)
**Goal**: Deploy to all target platforms

#### Web (WASM)
- [ ] Optimize WASM bundle size
- [ ] Test in multiple browsers
- [ ] Progressive Web App (PWA) setup
- [ ] Host on GitHub Pages or itch.io

#### Desktop
- [ ] Build native executables
- [ ] Package for Windows/Mac/Linux
- [ ] Create installers/DMG/AppImage
- [ ] Test on target platforms

#### Mobile (Future)
- [ ] Set up cargo-mobile2
- [ ] Android build and APK
- [ ] iOS build and TestFlight
- [ ] Touch controls optimization
- [ ] Mobile performance profiling

#### Technical Tasks:
- CI/CD pipeline for automated builds
- Asset optimization for each platform
- Platform-specific configurations
- Performance benchmarking

---

### Phase 6: Content & Release (Weeks 19-24)
**Goal**: Create content and prepare for release

#### Milestones:
- [ ] Complete level design (minimum 10 levels)
- [ ] Balanced difficulty progression
- [ ] Tutorial/onboarding
- [ ] Achievements/unlockables
- [ ] Leaderboards (optional)
- [ ] Localization support (optional)

#### Release Tasks:
- [ ] Play testing and bug fixes
- [ ] Performance optimization
- [ ] Write game documentation
- [ ] Create marketing materials
- [ ] Beta testing program
- [ ] Public release

---

## Technical Architecture

### Project Structure
```
path-of-goodies/
├── assets/
│   ├── sprites/        # Character and object sprites
│   ├── tiles/          # Tilemap textures
│   ├── audio/          # Music and sound effects
│   ├── fonts/          # UI fonts
│   └── maps/           # Level data (JSON/RON)
├── src/
│   ├── main.rs         # Entry point
│   ├── game.rs         # Core game state
│   ├── systems/        # ECS systems
│   │   ├── movement.rs
│   │   ├── combat.rs
│   │   ├── ai.rs
│   │   └── rendering.rs
│   ├── components/     # ECS components
│   │   ├── player.rs
│   │   ├── enemy.rs
│   │   └── item.rs
│   ├── resources/      # Shared resources
│   │   ├── assets.rs
│   │   └── config.rs
│   └── ui/             # UI systems
│       ├── hud.rs
│       └── menus.rs
├── Cargo.toml
├── Trunk.toml          # Web build config
├── index.html          # Web entry point
└── README.md
```

### Key Design Patterns

1. **Entity Component System (ECS)**
   - All game objects as entities
   - Behavior as systems operating on components
   - Decoupled and performant architecture

2. **Asset Management**
   - Lazy loading for web
   - Asset handles and hot-reloading in dev
   - Compressed assets for production

3. **State Management**
   - Bevy States for game flow (Menu, Playing, Paused, etc.)
   - Clear state transitions
   - State-specific system scheduling

4. **Input Abstraction**
   - Platform-agnostic input handling
   - Support keyboard, gamepad, touch
   - Rebindable controls

---

## Cross-Platform Considerations

### Web (WASM)
**Advantages:**
- Instant access via browser
- No installation required
- Easy distribution

**Challenges:**
- Bundle size optimization critical
- Limited threading support
- Browser compatibility testing

**Solutions:**
- Asset streaming and lazy loading
- Use wasm-opt for optimization
- Progressive enhancement approach

### Desktop
**Advantages:**
- Full performance potential
- Native file system access
- Gamepad support

**Challenges:**
- Multi-platform testing
- Distribution and updates

**Solutions:**
- Conditional compilation for OS-specific features
- CI/CD for multi-platform builds

### Mobile
**Advantages:**
- Touch-native controls
- Largest potential audience
- App store distribution

**Challenges:**
- Performance on lower-end devices
- Touch UI design
- App store compliance

**Solutions:**
- Dynamic quality settings
- Mobile-specific UI overlay
- Regular profiling and optimization

---

## Performance Targets

### Web (WASM)
- 60 FPS on modern browsers
- < 10MB initial bundle size
- < 3s load time on fast connection

### Desktop
- 144 FPS capable on mid-range hardware
- < 100MB RAM usage
- Instant startup (< 1s)

### Mobile
- 60 FPS on devices from 2020+
- < 200MB storage
- Battery-efficient rendering

---

## Testing Strategy

1. **Unit Tests**: Core game logic and systems
2. **Integration Tests**: System interactions
3. **Browser Testing**: Chrome, Firefox, Safari
4. **Device Testing**: Various screen sizes and capabilities
5. **Performance Profiling**: Regular frame time analysis
6. **Play Testing**: User feedback iterations

---

## Resources & Learning

### Bevy Resources
- [Official Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Bevy Assets](https://bevyengine.org/assets/)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)

### WASM/Web
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [Trunk Documentation](https://trunkrs.dev/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

### Game Development
- [Game Programming Patterns](https://gameprogrammingpatterns.com/)
- [Red Blob Games](https://www.redblobgames.com/) (algorithms and visualization)
- [itch.io](https://itch.io/) (distribution and community)

### Communities
- Bevy Discord
- /r/rust_gamedev
- Rust GameDev Working Group

---

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Mobile support complexity | High | Start with web/desktop, add mobile later |
| WASM performance issues | Medium | Early profiling, asset optimization |
| Scope creep | High | Strict phase milestones, MVP first |
| Cross-platform bugs | Medium | CI testing, platform-specific testing |
| Asset creation bottleneck | Medium | Use placeholder art, open-source assets initially |

---

## Success Metrics

### Phase 1 (MVP)
- ✅ Playable demo on web
- ✅ Basic movement and interaction
- ✅ 60 FPS web performance

### Phase 2 (Alpha)
- ✅ Complete core gameplay loop
- ✅ 3+ levels
- ✅ Desktop builds available

### Phase 3 (Beta)
- ✅ All platforms supported
- ✅ 10+ levels
- ✅ Polish and juice

### Phase 4 (Release)
- ✅ Public release on itch.io/web
- ✅ 100+ players
- ✅ Positive feedback

---

## Next Steps

1. **Immediate**: Set up Cargo project and Bevy
2. **This Week**: Get web build working with Trunk
3. **This Month**: Complete Phase 1 - Core Game Loop
4. **This Quarter**: Reach playable alpha with core mechanics

**Start Date**: 2025-10-21
**Target MVP**: 2025-12-15 (8 weeks)
**Target Alpha**: 2026-02-15 (16 weeks)
**Target Release**: 2026-04-15 (24 weeks)

---

*This roadmap is a living document. Update it as the project evolves and priorities shift.*
