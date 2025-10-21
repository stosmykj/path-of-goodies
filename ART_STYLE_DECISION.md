# Path of Goodies - Art Style Decision

**Decision Date**: 2025-10-21
**Status**: ✅ APPROVED - Pixel Art with Future Polish

---

## Final Decision: Pixel Art First

After evaluating both options, we've decided to proceed with **pixel art** as the initial art style, with the option to polish and potentially upgrade to hand-drawn or hybrid style post-launch.

### Rationale

1. **Timeline Alignment** - 6-month MVP timeline is achievable
2. **Moddability** - Easier for community to create pixel art mods
3. **Performance** - Better for web (WASM) deployment
4. **Iteration Speed** - Fast prototyping and testing
5. **Budget Friendly** - Solo/small team can create assets
6. **Proven Success** - Games like Stardew Valley show pixel art works

---

## Art Specifications

### Resolution Standards

```
Tiles:
- Base tile: 16x16 pixels
- Isometric footprint: 16x8 pixels (2:1 ratio)
- Seamless tiling required

Characters & NPCs:
- Guards/Enemies: 32x32 pixels
- Player wagon: 48x48 pixels (hero object)
- Companions: 32x32 pixels
- Wildlife: 24x24 to 32x32 pixels

UI Elements:
- Icons: 16x16 pixels
- Buttons: 32x16 to 64x32 pixels
- Portraits: 64x64 pixels (optional)
- Font: 8x8 or custom pixel font

Props & Objects:
- Small props: 16x16 pixels (rocks, shrubs)
- Medium props: 32x32 pixels (trees, signs)
- Large props: 64x64 pixels (buildings, gates)
```

### Color Palette

**Total Colors**: 48-64 colors
**Style**: Warm medieval fantasy tones

```
Primary Palette:
█ #8B4513 - Wood (dark brown)
█ #D2B48C - Wood (light tan)
█ #654321 - Wood (very dark)
█ #228B22 - Grass (main green)
█ #90EE90 - Grass (light green)
█ #006400 - Grass (dark green)
█ #8B7355 - Road (dirt brown)
█ #4169E1 - Guard (blue armor)
█ #FFD700 - Gold/UI accent
█ #DC143C - Danger/Enemy red
█ #2F4F4F - Shadow/Enemy dark
█ #FFF8DC - Skin/Highlights

Secondary (as needed):
- Forest greens
- Mountain grays
- Desert yellows/oranges
- Water blues
- Fire reds/oranges
```

### Perspective & Camera

**View**: Isometric (2:1 ratio)
**Angle**: 45-degree from top
**Grid**: Diamond tile layout
**Camera**: Orthographic projection, follows wagon

### Animation Standards

```
Character Animations:
- Walk cycle: 4-8 frames
- Attack: 3-4 frames
- Idle: 2-4 frames (breathing)
- Death: 4-6 frames
- Frame rate: 8-12 FPS (game running at 60)

Wagon Animation:
- Wheels rotating: 4 frames loop
- Bouncing: 2-4 frames subtle
- Damaged states: 3 variations

Environmental:
- Grass sway: 2-4 frames
- Water flow: 4 frames
- Fire/particles: 4-6 frames
```

---

## Asset Creation Guidelines

### For Pixel Artists

1. **Anti-aliasing**: None (crisp pixels only)
2. **Dithering**: Minimal, use for gradients/shadows
3. **Outlines**: 1-pixel dark outlines for clarity
4. **Consistency**: Maintain light source (top-left)
5. **Readable**: Clear at 100% zoom
6. **Modular**: Design for recoloring/reuse

### Tools Recommended

- **Aseprite** (best for pixel art, animation)
- **LibreSprite** (free Aseprite fork)
- **Piskel** (web-based, free)
- **GraphicsGale** (free, Windows)
- **Pyxel Edit** (tilemap focused)

### Reference Games

Primary style inspiration:
- **Stardew Valley** - Character detail level
- **Graveyard Keeper** - Medieval setting, atmosphere
- **Loop Hero** - Minimalist but effective
- **Pathway** - Isometric combat

---

## Asset Pipeline

### Development Workflow

```
1. Concept Sketch (optional)
   ↓
2. Block out in pixels (rough shapes)
   ↓
3. Define colors (limited palette)
   ↓
4. Add details and shading
   ↓
5. Create animation frames
   ↓
6. Export as sprite sheet
   ↓
7. Import to Bevy and test
   ↓
8. Iterate based on gameplay feel
```

### File Organization

```
assets/sprites/
├── characters/
│   ├── guards/
│   │   ├── archer.png (sprite sheet)
│   │   ├── swordsman.png
│   │   └── shield_bearer.png
│   ├── enemies/
│   │   ├── bandit.png
│   │   ├── wolf.png
│   │   └── bear.png
│   └── player/
│       └── wagon.png
├── tiles/
│   ├── terrain.png (tileset)
│   ├── roads.png
│   └── objects.png
├── ui/
│   ├── icons.png
│   ├── buttons.png
│   └── frames.png
└── fx/
    ├── particles.png
    ├── impacts.png
    └── magic.png
```

### Export Settings

```
Format: PNG (transparency supported)
Color depth: 8-bit indexed (palette mode)
Sprite sheets: Row-based or grid
Naming: lowercase_with_underscores.png
Metadata: Include .json or .ron with frame data
```

---

## Modding Considerations

### Why Pixel Art is Mod-Friendly

1. **Lower Skill Barrier** - Easier for hobbyists to create
2. **Smaller File Sizes** - Faster mod downloads
3. **Clear Templates** - Easy to provide base sprites
4. **Recolor-Friendly** - Palette swaps for variants
5. **Mix-and-Match** - Modular parts (heads, bodies, weapons)

### Mod Asset Guidelines

```
Mods should follow same specs:
- Same resolutions (16x16 tiles, 32x32 characters)
- Same isometric perspective
- Compatible color palette (can extend)
- Same animation frame counts (for consistency)
- Transparent backgrounds
```

---

## Future Polish Options

### Post-Launch Enhancement Path

**Option 1: Higher-Res Pixel Art**
- Upgrade to 64x64 characters
- More detail and animation frames
- Keep pixel art aesthetic
- Effort: Medium
- Timeline: 2-3 months

**Option 2: Hybrid Style**
- Keep pixel art for world/gameplay
- Add hand-drawn portraits for characters
- Hand-drawn UI elements
- Effort: Medium-High
- Timeline: 3-4 months

**Option 3: Full Hand-Drawn Remaster**
- Complete art overhaul
- Painterly style like Hades
- "HD Edition" or "Remastered"
- Effort: Very High
- Timeline: 6-8 months

**Option 4: 3D Low-Poly**
- Keep top-down view but 3D models
- Pixel shader for pixel-art-like look
- Dynamic lighting
- Effort: Very High
- Timeline: 6-12 months

### Decision Point

We'll evaluate art style upgrade at:
- ✅ **6 months** - After MVP release
- ✅ **1,000+ players** - If game is successful
- ✅ **Revenue positive** - If financially viable
- ✅ **Community feedback** - If players want it

---

## Starting Assets Needed

### Phase 1 (Weeks 3-6) - MVP Core

**Essential Assets:**
- [ ] Terrain tileset (grass, road, basic)
- [ ] Player wagon sprite (4 directions)
- [ ] Guard sprites (archer, swordsman) x2 each
- [ ] Enemy sprite (bandit) x2
- [ ] Basic UI icons (food, water, gold)
- [ ] HUD frame/border

**Can Use Placeholders:**
- Colored squares for guards
- Simple circles for enemies
- Programmer art for UI

### Phase 2 (Weeks 7-10) - Expanded Content

**Additional Assets:**
- [ ] More biome tiles (forest, desert, mountain)
- [ ] More enemy types (wolf, bear)
- [ ] More guard types (shield bearer)
- [ ] Settlement buildings (city, village)
- [ ] Environmental objects (trees, rocks)
- [ ] Complete UI set
- [ ] Particle effects

### Phase 3 (Weeks 11-14) - Polish

**Final Assets:**
- [ ] All animations complete
- [ ] Visual effects polish
- [ ] UI/UX refinement
- [ ] Character portraits (optional)
- [ ] Cutscene art (optional)

---

## Asset Sources

### Option 1: Create Custom (Recommended for Final)
**Pros**: Unique, exactly what you need
**Cons**: Time-consuming
**Tools**: Aseprite, LibreSprite

### Option 2: Asset Packs (Good for MVP)
**Free Sources**:
- OpenGameArt.org
- Itch.io (pixel art category)
- Kenney.nl (some free packs)

**Paid Sources**:
- Itch.io marketplace ($5-50)
- Humble Bundle asset packs
- GraphicRiver / Envato

**Examples**:
- "Medieval Fantasy Character Pack" on itch.io
- "LPC (Liberated Pixel Cup) Assets" (free, large collection)
- "Tiny" series by Kenney

### Option 3: AI-Generated (Supplement)
**Use AI prompts** from IMAGE_GENERATION_PROMPTS.md
**Then**:
- Clean up in Aseprite
- Ensure consistency
- Fix perspective issues
- Touch up details

### Option 4: Commission Artist
**Cost**: $500-5,000 depending on scope
**Timeline**: 1-3 months
**Best for**: Phase 3+ (polish stage)

---

## Action Items

### Immediate (This Week)
- [ ] Gather/create basic tileset (grass, road)
- [ ] Create or find wagon sprite (48x48)
- [ ] Create or find 2 guard types (32x32)
- [ ] Create or find 1 enemy type (32x32)
- [ ] Basic UI icons (16x16)

### Short-Term (Phase 1)
- [ ] Set up Aseprite or pixel art tool
- [ ] Create art style guide document
- [ ] Build sprite sheet template
- [ ] Test sprite loading in Bevy
- [ ] Create reusable color palette file

### Long-Term (Phase 2+)
- [ ] Expand asset library
- [ ] Create animation variants
- [ ] Commission custom art if needed
- [ ] Build mod template pack
- [ ] Community asset submissions

---

## Quality Standards

### Minimum Acceptable Quality (MVP)
- Readable and clear
- Consistent perspective
- Fits color palette
- No visual bugs (missing pixels, wrong alpha)

### Target Quality (Launch)
- Polished and detailed
- Smooth animations
- Cohesive art style
- Professional presentation

### Aspirational Quality (Post-Launch)
- Best-in-class pixel art
- Rich animations
- Expressive characters
- Award-worthy visuals

---

## Community & Modding

### Providing Resources to Modders

**We will provide**:
- Base sprite templates (blank 32x32 character)
- Color palette (.ase or .gpl file)
- Sprite sheet format documentation
- Example mod assets
- Modding tutorial/guide

**Community can create**:
- Custom guards/enemies
- New biome tilesets
- Custom wagons
- UI themes/skins
- Seasonal variants (Christmas, Halloween)
- Texture packs (HD, minimalist, etc.)

---

## Conclusion

✅ **Decision: Pixel Art (32x32 characters, 16x16 tiles)**
✅ **Timeline: Start immediately in Phase 1**
✅ **Quality: MVP-acceptable → Polish over time**
✅ **Moddable: Design for community content**
✅ **Future: Open to enhancement post-launch**

Let's create a beautiful, moddable pixel art game!

---

**Approved**: 2025-10-21
**Next Review**: After Phase 1 (Week 6)
