# Sprite Specifications

## Art Style Overview

**Style**: Pixel art with modern sensibilities
**Perspective**: Top-down 45° isometric-inspired (but not true isometric)
**Color Depth**: 16-32 colors per sprite
**Resolution**: Native pixel art, scaled 2x-4x for display
**Animation**: Minimal but purposeful

## Design Principles

1. **Readable at Small Size** - Clear at 16x16, detailed at 32x32
2. **Consistent Palette** - Limited colors, shared across sprites
3. **Expressive Simplicity** - Personality without complexity
4. **Functional Beauty** - Every pixel serves gameplay or aesthetics
5. **Moddable Structure** - Easy to replace/modify

---

## Master Color Palette

### Core Palette (32 Colors)

```
## Terrain & Environment
Ground_Dark:    #3e2731 ████
Ground_Mid:     #5a4a52 ████
Ground_Light:   #8b7665 ████
Grass_Dark:     #2d5016 ████
Grass_Mid:      #4a7c2e ████
Grass_Light:    #6db043 ████

## Wood & Structure
Wood_Dark:      #3e2731 ████
Wood_Mid:       #5a4a52 ████
Wood_Light:     #a0826d ████

## Stone & Buildings
Stone_Dark:     #4a4e69 ████
Stone_Mid:       #6b7398 ████
Stone_Light:    #9badb7 ████

## Metals
Metal_Dark:     #333333 ████
Metal_Mid:      #6b6b6b ████
Metal_Light:    #b8b8b8 ████

## Fabrics & Cloth
Cloth_Red:      #8b2e2e ████
Cloth_Blue:     #2e4a8b ████
Cloth_Green:    #2e8b4a ████
Cloth_Brown:    #6b4a2e ████

## Accents
Gold:           #d4a03d ████
Silver:         #c0c0c0 ████
Leather_Dark:   #5a3a2e ████
Leather_Light:  #8b6a5a ████

## UI Elements
UI_Dark:        #1a1a2e ████
UI_Mid:         #2d3561 ████
UI_Light:       #ffffff ████
UI_Highlight:   #ffd700 ████
UI_Warning:     #ff6b6b ████
UI_Success:     #6bff6b ████

## Special
Outline:        #000000 ████
Transparent:    #ff00ff (magic pink)
```

### Usage Rules

✓ **Use palette colors only** - No custom colors without documentation
✓ **Consistent shading** - 3-tone shading (dark, mid, light)
✓ **Black outlines** - 1px outlines on all sprites for clarity
✓ **Transparent backgrounds** - Use magic pink (#ff00ff) for alpha

---

## Sprite Catalog

### 1. Player Wagon

**Filename**: `wagon_idle.png`, `wagon_moving.png`
**Size**: 48x48 pixels
**Origin**: Center-bottom
**Animation**: 2 frames (idle, moving)

```
Visual Description:
- Wooden cart with large wheels
- Visible cargo covered by brown tarp
- Two-wheeled design (side view)
- Driver seat at front
- Sturdy and well-maintained appearance

Details:
[48x48 pixel art of medieval wagon]
┌──────────┐
│  ╔════╗  │  Covered cargo area
│  ║░░░░║  │
│ ┌╚════╝┐ │
│ │ [__] │ │  Driver seat
│ O      O │  Large wooden wheels
└──────────┘

Colors:
- Wood: Wood_Mid, Wood_Light
- Wheels: Wood_Dark
- Tarp: Cloth_Brown
- Metal fittings: Metal_Dark
- Outline: Black
```

**Animation**:
- Frame 1 (Idle): Wheels static
- Frame 2 (Moving): Wheels rotated 45°
- Duration: 0.3s per frame

---

### 2. Horse

**Filename**: `horse_idle.png`, `horse_walk.png`
**Size**: 32x32 pixels
**Origin**: Center-bottom
**Animation**: 4 frames walking cycle

```
Visual Description:
- Brown/chestnut horse
- Side view profile
- Simple but expressive
- Visible harness and reins
- Tail and mane visible

Details:
[32x32 pixel art]
     ╭─╮
     │◉│  Head
   ┌─╯ ╰─┐
   │░░░░░│ Body
   │░░░░░│
   ╯ ╯╰ ╰ Legs

Colors:
- Body: Leather_Dark, Leather_Light
- Harness: Cloth_Brown
- Hooves: Wood_Dark
- Eyes: UI_Light (white dot)
- Outline: Black

Health States:
- Healthy (80-100%): Normal colors
- Tired (50-80%): Slightly desaturated
- Weak (20-50%): Very desaturated
- Critical (<20%): Red tint overlay
```

**Animations**:
- Idle: Gentle breathing (head bob)
- Walk: 4-frame walk cycle
- Death: Fall over animation (4 frames)

---

### 3. Guard Characters

**Filename**: `guard_[type]_idle.png`, `guard_[type]_combat.png`
**Size**: 24x24 pixels each
**Origin**: Center-bottom
**Types**: Swordsman, Archer, Spearman, Crossbowman

#### Swordsman
```
[24x24 pixel art]
    ╭─╮
    │◉│   Head with helmet
  ╭─╯█╰─╮
  │ ║░║ │ Body with armor
  │ ║░║ │
  ╯ │ │ ╰ Legs

Weapon: Sword (held at side)
Armor: Medium (chainmail look)
Colors:
- Armor: Metal_Mid
- Cloth: Cloth_Red
- Skin: Leather_Light
- Weapon: Metal_Light
```

#### Archer
```
[24x24 pixel art]
    ╭─╮
    │◉│   Head with hood
  ╭─╯ ╰─╮
  │ )░) │ Body with bow
  │ )░) │
  ╯ │ │ ╰ Legs

Weapon: Bow (held in hand)
Armor: Light (leather look)
Colors:
- Leather: Leather_Dark
- Cloth: Cloth_Green
- Bow: Wood_Mid
- Quiver: Cloth_Brown
```

#### Spearman
```
Similar to Swordsman but:
- Holds spear vertically
- Shield on other arm
- Heavy armor look
- Colors: Metal_Mid, Cloth_Blue
```

#### Crossbowman
```
Similar to Archer but:
- Holds crossbow
- Heavier armor
- Wider stance
- Colors: Metal_Dark, Cloth_Brown
```

**Animations**:
- Idle: Gentle breathing
- Attack: Weapon swing/shoot (3 frames)
- Hit: Recoil (2 frames)
- Death: Fall down (3 frames)

---

### 4. Villages & Buildings

#### Hamlet Icon
**Filename**: `village_hamlet.png`
**Size**: 32x32 pixels

```
[32x32 pixel art]
    ╱╲
   ╱  ╲     Single cottage
  ╱────╲
  │ ▓▓ │    Door and window
  └────┘

Colors:
- Roof: Cloth_Red
- Walls: Stone_Light
- Door: Wood_Dark
- Window: UI_Mid
```

#### Village Icon
**Filename**: `village_village.png`
**Size**: 48x48 pixels

```
[48x48 pixel art]
  ╱╲   ╱╲
 ╱  ╲ ╱  ╲   Two buildings
╱────┴────╲
│ ▓▓ │ ▓▓ │  Doors
└────┴────┘

Colors: Same as Hamlet
Size: Two connected buildings
```

#### Town Icon
**Filename**: `village_town.png`
**Size**: 64x64 pixels

```
[64x64 pixel art]
     │││     Tower with flag
    ╱───╲
   │ ╱╲  │   Main building
   │╱──╲ │
  ╱───────╲  Town hall look
  │▓▓▓▓▓▓│
  └───────┘

Colors:
- Tower: Stone_Mid
- Flag: Cloth_Red
- Walls: Stone_Light
- Architecture: More detailed
```

#### City Icon
**Filename**: `village_city.png`
**Size**: 96x96 pixels

```
[96x96 pixel art]
   ╭──────╮
   │ │││  │   Castle towers
   │ ╱══╲ │   City walls
  ╱│╱────╲│╲  Multiple buildings
 │ ▓▓▓▓▓▓▓ │  Large gate
 └──────────┘

Colors:
- Walls: Stone_Mid, Stone_Dark
- Towers: Stone_Dark
- Details: Metal accents
- Most detailed sprite
```

---

### 5. UI Elements

#### Resource Icons
**Size**: 16x16 pixels each
**Style**: Simple, iconic, readable

```
Gold Coin (16x16):
  ╱╲
 │◎│   Circular coin with symbol
  ╲╱
Colors: Gold, Metal_Dark (outline)

Food/Bread (16x16):
  ╱╲
 ╱──╲   Loaf of bread
│░░░░│
└────┘
Colors: Wood_Light, Wood_Mid

Water/Flask (16x16):
  ╭╮
 │░░│   Water flask
 │░░│
  ╲╱
Colors: UI_Mid (glass), UI_Light (water reflection)

Stamina/Lightning (16x16):
  │
 ╱╲╱╲   Lightning bolt
 ╲╱ ╲╱
   │
Colors: UI_Highlight (gold), UI_Warning (yellow)

Health/Heart (16x16):
 ╱╲╱╲
│░░░░│  Heart shape
 ╲░░╱
  ╲╱
Colors: UI_Warning (red), Cloth_Red (dark)
```

#### UI Panels
**Style**: Medieval parchment/wood
**Corners**: Decorated with ornate details
**Borders**: 2-4px decorative frame

```
Panel Corner (8x8):
╔═══
║ ╱
║╱
Colors: Wood_Mid, Wood_Dark

Panel Background:
Pattern: Subtle parchment texture
Color: UI_Mid with slight grain
```

---

### 6. Path & Terrain Elements

#### Path Texture
**Filename**: `path_segment.png`
**Size**: 32x32 pixels (tileable)
**Style**: Dirt road

```
[32x32 tileable texture]
░░▓▓░░▓░░▓▓░░   Dirt/gravel texture
▓░░░▓░░▓░░░▓
░▓░░░░░░░▓░░   Random pattern
▓░░▓░░▓░░░░▓
░░▓▓░░░▓▓░░░

Colors:
- Ground_Dark (shadows)
- Ground_Mid (dirt)
- Ground_Light (highlights)
```

#### Grass Texture
**Filename**: `grass_tile.png`
**Size**: 32x32 pixels (tileable)

```
[32x32 tileable texture]
░░░░│░░│░░░░   Grass blades
│░░░░░░░░│░░
░│░░░│░░░░░░   Random tufts
░░░│░░░░░│░░

Colors:
- Grass_Dark (base)
- Grass_Mid (main)
- Grass_Light (highlights)
```

---

### 7. Encounter Sprites

#### Bandit
**Filename**: `encounter_bandit.png`
**Size**: 32x32 pixels

```
[32x32 pixel art]
    ╭─╮
    │X│   Masked face
  ╭─╯ ╰─╮
  │ ║░║ │ Rough armor
  │\║░║/│ Aggressive pose
  ╯ │ │ ╰

Colors:
- Dark, menacing colors
- Cloth_Brown, Metal_Dark
- Red accent (bandana)
```

#### Merchant
**Filename**: `encounter_merchant.png`
**Size**: 32x32 pixels

```
[32x32 pixel art]
    ╭─╮
    │☺│   Friendly face
  ╭─╯ ╰─╮
  │ $░$ │ Fancy clothes
  │ ║░║ │ Money bags
  ╯ │ │ ╰

Colors:
- Bright, wealthy colors
- Cloth_Blue, Gold accents
- Clean and well-dressed
```

#### Wolf/Wild Animal
**Filename**: `encounter_wolf.png`
**Size**: 24x24 pixels

```
[24x24 pixel art]
  ╭─╮ ╭╮
  │◉│╭╯╰╮  Wolf head
 ╭╯ ╰╯  ╰╮
 │░░░░░░░│ Body
 ╯╯ ╯╯ ╯╯ Legs

Colors:
- Grass_Dark, Grass_Mid (wolf fur)
- UI_Light (teeth/eyes)
- Menacing but not cartoonish
```

---

## Animation Standards

### Frame Timing
```
Idle: 0.5-1.0s per frame (slow, calm)
Walk: 0.2-0.3s per frame (natural pace)
Run: 0.1-0.15s per frame (fast)
Attack: 0.1s per frame (snappy)
```

### Animation Principles
1. **Anticipation** - Wind up before action
2. **Follow-through** - Complete the motion
3. **Ease in/out** - Not linear timing
4. **Minimal frames** - 2-4 frames usually enough

### Common Animations
```
Walk Cycle: 4 frames
│ ╰╯ │ ╯╰ │ ╰╯ │ ╯╰
  1    2    3    4

Idle Breathing: 2 frames
│═│ │≈│  Subtle movement
 1   2

Attack: 3 frames
│\  │-  │/   Swing arc
 1   2   3
```

---

## File Naming Conventions

### Format
```
[category]_[name]_[variant].png

Examples:
character_wagon_idle.png
character_wagon_moving.png
character_horse_walk_01.png
guard_swordsman_idle.png
guard_swordsman_attack.png
village_hamlet.png
village_town.png
ui_icon_gold.png
ui_icon_food.png
encounter_bandit.png
terrain_grass_tile.png
```

### Directory Structure
```
/assets/
  /sprites/
    /characters/
      wagon_idle.png
      wagon_moving.png
      horse_idle.png
      horse_walk.png
    /guards/
      swordsman_idle.png
      swordsman_attack.png
      archer_idle.png
      ...
    /villages/
      hamlet.png
      village.png
      town.png
      city.png
    /ui/
      icon_gold.png
      icon_food.png
      icon_water.png
      panel_corner.png
    /encounters/
      bandit.png
      merchant.png
      wolf.png
    /terrain/
      grass_tile.png
      path_segment.png
      dirt_tile.png
```

---

## Technical Specifications

### File Format
- **Format**: PNG with alpha channel
- **Color Mode**: Indexed color (256 colors max)
- **Bit Depth**: 8-bit
- **Compression**: PNG compression

### Resolution Guidelines
```
Tiny: 8x8 to 16x16       (UI icons)
Small: 16x16 to 24x24    (Guards, NPCs)
Medium: 24x24 to 48x48   (Wagon, Horse, Buildings)
Large: 48x48 to 96x96    (Cities, Boss enemies)
Huge: 96x96+             (Rare, special sprites)
```

### Pixel Density
```
Base resolution: Native pixel art
Display scaling: 2x, 3x, or 4x (no filtering)
Anti-aliasing: None (crisp pixels)
Filtering: Nearest neighbor only
```

### Sprite Sheets
```
For animations, use sprite sheets:
- Horizontal layout (left to right = frames)
- Consistent frame size
- No padding between frames
- Power-of-2 dimensions when possible
  (128x64, 256x128, etc.)
```

---

## Art Production Pipeline

### 1. Concept
- Sketch rough idea (paper or digital)
- Define key features
- Check readability at target size

### 2. Blockout
- Place basic shapes
- Define proportions
- No details yet

### 3. Lineart
- Add black outlines (1px)
- Define form and structure
- Keep lines consistent

### 4. Base Colors
- Fill with flat colors from palette
- No shading yet
- Check palette compliance

### 5. Shading
- Add 3-tone shading (dark, mid, light)
- Consistent light source (top-left)
- Subtle highlights

### 6. Details
- Add final touches
- Small details and accents
- Eyes, buttons, textures

### 7. Animation
- Duplicate base frame
- Modify for each frame
- Test at game speed
- Adjust timing

### 8. Export
- Check correct size
- Verify palette
- Save as PNG
- Add to asset folder

---

## Style Reference

### Do's ✓
- Use consistent palette
- 1px black outlines
- 3-tone shading
- Readable at small size
- Consistent light source
- Clean, crisp pixels

### Don'ts ✗
- No anti-aliasing
- No gradients (use dithering)
- No off-palette colors
- No muddy/unclear shapes
- No inconsistent outlines
- No automatic scaling (blur)

---

## Mockup Examples

### Game Screen Layout
```
┌─────────────────────────────────────────────────┐
│ 💰145  🍞42  💧38  ⚡67  ❤️85     Day 5  12:30  │ ← HUD
├─────────────────────────────────────────────────┤
│                                                 │
│         [Parallax Background - Forest]         │
│                                                 │
│              ═════════════════                  │ ← Path
│                                                 │
│                  [Horse][Wagon]                 │ ← Player
│                                                 │
│                                                 │
│         [Grass]  [Tree]  [Rock]                │ ← Scenery
│                                                 │
├─────────────────────────────────────────────────┤
│ 🎯 Destination: Millhaven │ 150/250 units      │ ← Info bar
└─────────────────────────────────────────────────┘
```

### Encounter Screen
```
┌─────────────────────────────────────────┐
│         [Encounter Image - Bandit]      │
│                                         │
│    "Bandits block your path!"          │
│                                         │
│    You: [Wagon] [Horse] [2 Guards]    │
│    Them: [3 Bandits]                   │
│                                         │
│  1. Fight (60% chance, may lose HP)   │
│  2. Pay 20g (guaranteed safe)          │
│  3. Flee (50% chance, lose time)       │
│  4. Negotiate (requires charisma)      │
│                                         │
│       [Press 1-4 to choose]            │
└─────────────────────────────────────────┘
```

---

**Version**: 1.0
**Last Updated**: 2025-10-22
**Status**: DRAFT - Ready for asset creation
