# Color Palette Reference

## Master Palette

This is the definitive color palette for Path of Goodies. All sprites should use colors from this palette only. Special exceptions must be documented.

## Color Codes

### Terrain & Environment

```
Ground_Dark     #3e2731    RGB(62, 39, 49)    Dark soil, shadows
Ground_Mid      #5a4a52    RGB(90, 74, 82)    Main ground color
Ground_Light    #8b7665    RGB(139, 118, 101) Highlights, dry earth
Grass_Dark      #2d5016    RGB(45, 80, 22)    Deep grass, shadows
Grass_Mid       #4a7c2e    RGB(74, 124, 46)   Main grass color
Grass_Light     #6db043    RGB(109, 176, 67)  Bright grass, highlights
```

### Wood & Structure

```
Wood_Dark       #3e2731    RGB(62, 39, 49)    Deep wood grain, shadows
Wood_Mid        #5a4a52    RGB(90, 74, 82)    Main wood color
Wood_Light      #a0826d    RGB(160, 130, 109) Light wood, highlights
```

### Stone & Buildings

```
Stone_Dark      #4a4e69    RGB(74, 78, 105)   Dark stone, shadows
Stone_Mid       #6b7398    RGB(107, 115, 152) Main stone color
Stone_Light     #9badb7    RGB(155, 173, 183) Light stone, highlights
```

### Metals

```
Metal_Dark      #333333    RGB(51, 51, 51)    Dark metal, iron
Metal_Mid       #6b6b6b    RGB(107, 107, 107) Steel, weapons
Metal_Light     #b8b8b8    RGB(184, 184, 184) Polished metal, silver
```

### Fabrics & Cloth

```
Cloth_Red       #8b2e2e    RGB(139, 46, 46)   Red fabric, banners
Cloth_Blue      #2e4a8b    RGB(46, 74, 139)   Blue fabric, clothes
Cloth_Green     #2e8b4a    RGB(46, 139, 74)   Green fabric, cloaks
Cloth_Brown     #6b4a2e    RGB(107, 74, 46)   Brown fabric, common
```

### Accents

```
Gold            #d4a03d    RGB(212, 160, 61)  Gold coins, treasure
Silver          #c0c0c0    RGB(192, 192, 192) Silver items
Leather_Dark    #5a3a2e    RGB(90, 58, 46)    Dark leather, boots
Leather_Light   #8b6a5a    RGB(139, 106, 90)  Light leather, armor
```

### UI Elements

```
UI_Dark         #1a1a2e    RGB(26, 26, 46)    Dark UI backgrounds
UI_Mid          #2d3561    RGB(45, 53, 97)    Main UI color
UI_Light        #ffffff    RGB(255, 255, 255) Text, highlights
UI_Highlight    #ffd700    RGB(255, 215, 0)   Selected, attention
UI_Warning      #ff6b6b    RGB(255, 107, 107) Danger, low health
UI_Success      #6bff6b    RGB(107, 255, 107) Success, healing
```

### Special

```
Outline         #000000    RGB(0, 0, 0)       All sprite outlines
Transparent     #ff00ff    RGB(255, 0, 255)   Magic pink (alpha)
```

---

## Usage Guidelines

### Three-Tone Shading Rule

Every object should use exactly 3 shades for depth:
- **Dark**: Shadows and recesses
- **Mid**: Main body color
- **Light**: Highlights and edges

Example - Wooden Wagon:
```
Dark:  Wood_Dark   #3e2731  (Wheel shadows, under-carriage)
Mid:   Wood_Mid    #5a4a52  (Main wagon body, planks)
Light: Wood_Light  #a0826d  (Top edges, sunlit areas)
```

### Consistent Light Source

All sprites should assume **top-left lighting**:
- Highlights on top and left edges
- Shadows on bottom and right edges
- Consistent across all assets

```
Good:                Bad:
  ╱▓▓╲              ╱░░╲
 ▓▓▓▓▓▓            ▓▓▓▓▓▓
▓▓▓▓▓▓▓▓          ░░░░░░░░
 ░░░░░░             ╲▓▓╱
(Light from top)  (Inconsistent)
```

---

## Color Combinations

### Recommended Pairings

**For Characters**:
- Skin: Leather_Light
- Armor: Metal_Mid + Metal_Dark
- Clothes: Any Cloth color + darker shade
- Hair: Wood_Dark, Wood_Mid, Leather_Dark

**For Environments**:
- Grass: Grass_Dark (base) + Grass_Mid (detail) + Grass_Light (highlights)
- Dirt: Ground_Dark + Ground_Mid + Ground_Light
- Sky: UI_Mid (base) + UI_Light (clouds)

**For UI**:
- Panels: UI_Dark (background) + Wood_Mid (borders)
- Text: UI_Light on UI_Dark
- Buttons: Wood_Mid (normal) + Wood_Light (hover)
- Icons: Full color on UI_Dark background

### Avoid These Combinations

❌ Grass_Light + UI_Warning (too bright)
❌ Metal_Dark + Outline (no contrast)
❌ UI_Dark + Ground_Dark (muddy)
❌ Multiple bright colors together (visual noise)

---

## Palette Limitations

### Why Only 32 Colors?

1. **Consistency**: Limited palette forces cohesive art style
2. **Performance**: Smaller file sizes, faster rendering
3. **Artistic Unity**: All sprites feel part of same world
4. **Moddability**: Easy to understand and modify

### When to Add New Colors

New colors should only be added when:
1. Absolutely necessary for clarity
2. Can't achieve effect with existing colors
3. Approved by art lead (you!)
4. Documented in this file

**Process**:
1. Try using existing colors first
2. If impossible, document why
3. Add new color to palette
4. Update this document
5. Inform team/modders

---

## Color Psychology

### Emotional Associations

**Warm Colors** (Reds, Browns, Golds):
- Used for: Towns, inns, safety, rewards
- Feeling: Comfort, wealth, warmth

**Cool Colors** (Blues, Grays):
- Used for: Night, water, stone, metal
- Feeling: Cold, danger, distance

**Earth Tones** (Greens, Browns):
- Used for: Nature, travel, common items
- Feeling: Natural, grounded, realistic

**Bright Colors** (UI_Highlight, UI_Warning):
- Used for: UI feedback, important info
- Feeling: Attention, urgency, success

---

## Accessibility Considerations

### Color Blindness

The palette has been designed with color blindness in mind:

**Protanopia** (Red-Blind):
- Use brightness contrast, not just color
- Avoid red/green for critical info
- UI_Warning and UI_Success differ in brightness

**Deuteranopia** (Green-Blind):
- Similar considerations as Protanopia
- Gold stands out even without green perception

**Tritanopia** (Blue-Blind):
- Blues may appear gray
- Stone and UI colors have brightness variation

### High Contrast Mode

For accessibility, consider offering:
- Outline thickness option
- Text size scaling
- Color-blind friendly UI icons
- Screen reader support (text-based feedback)

---

## Palette Testing

### How to Verify

1. **Create test sprite** using all palette colors
2. **Export as indexed PNG** (8-bit)
3. **Check in image editor** - should show exactly 32 colors
4. **View at target size** - colors should be distinct
5. **Test on different monitors** - colors should remain readable

### Common Mistakes

❌ **Anti-aliasing**: Creates in-between colors
```
Bad:  ░▒▓█  (4+ colors from gradient)
Good: ░░▓▓  (2 colors, clean edge)
```

❌ **Opacity/Alpha**: Creates new colors when overlapping
```
Bad:  Red (50% opacity) over Blue = Purple (new color!)
Good: Use Purple from palette directly
```

❌ **Resizing**: Auto-scaling creates new colors
```
Bad:  Auto-scale with bilinear filtering
Good: Scale with nearest-neighbor only
```

---

## Palette Export Formats

### For Artists

**Aseprite Palette** (.pal):
```
Create palette file with exact hex codes
Import into Aseprite for consistent colors
Lock palette to prevent accidental colors
```

**Photoshop Swatch** (.aco):
```
Create swatch library
Share with team
Use color picker from swatches only
```

**GIMP Palette** (.gpl):
```
Create GIMP Palette (GPL) format
32 colors with names
Import for easy selection
```

### For Developers

**JSON Palette**:
```json
{
  "terrain": {
    "ground_dark": "#3e2731",
    "ground_mid": "#5a4a52",
    "ground_light": "#8b7665"
  },
  "grass": {
    "grass_dark": "#2d5016",
    "grass_mid": "#4a7c2e",
    "grass_light": "#6db043"
  }
}
```

**Rust Constants**:
```rust
pub const GROUND_DARK: Color = Color::rgb(0.243, 0.153, 0.192);
pub const GROUND_MID: Color = Color::rgb(0.353, 0.290, 0.322);
pub const GROUND_LIGHT: Color = Color::rgb(0.545, 0.463, 0.396);
```

---

## Version History

### v1.0 (2025-10-22)
- Initial 32-color palette
- Organized by category
- Documented usage guidelines

### Future Considerations

Potential additions (not confirmed):
- Sky blue variant for day/night
- Fire orange/yellow for flames
- Purple for magic/special effects
- Additional skin tones for diversity

---

## Quick Reference Card

```
TERRAIN       WOOD          STONE         METAL
#3e2731 ████  #3e2731 ████  #4a4e69 ████  #333333 ████
#5a4a52 ████  #5a4a52 ████  #6b7398 ████  #6b6b6b ████
#8b7665 ████  #a0826d ████  #9badb7 ████  #b8b8b8 ████
#2d5016 ████
#4a7c2e ████  CLOTH         ACCENTS       UI
#6db043 ████  #8b2e2e ████  #d4a03d ████  #1a1a2e ████
              #2e4a8b ████  #c0c0c0 ████  #2d3561 ████
              #2e8b4a ████  #5a3a2e ████  #ffffff ████
              #6b4a2e ████  #8b6a5a ████  #ffd700 ████
                                          #ff6b6b ████
SPECIAL                                   #6bff6b ████
#000000 ████ (Outline)
#ff00ff ████ (Transparent)
```

---

**Version**: 1.0
**Last Updated**: 2025-10-22
**Status**: LOCKED - Do not modify without documentation
