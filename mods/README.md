# Mods Folder

This folder contains mods for **Path of Goodies**.

## What are Mods?

Mods are community-created content that add or modify game features. You can install mods to customize your gameplay experience with new guards, enemies, items, biomes, and more!

## Installing Mods

### Method 1: In-Game (Recommended)
1. Launch Path of Goodies
2. Go to **Settings → Mods**
3. Click **"Browse Mods"**
4. Find a mod you like and click **"Install"**
5. Enable the mod in the mod list
6. Restart or hot-reload the game

### Method 2: Manual Installation
1. Download a mod (usually a .zip file)
2. Extract the mod folder to this directory (`mods/`)
3. The folder structure should look like:
   ```
   mods/
   ├── example_mod/
   │   ├── mod.ron
   │   ├── data/
   │   └── assets/
   └── your_new_mod/
       ├── mod.ron
       ├── data/
       └── assets/
   ```
4. Launch the game and enable the mod in **Settings → Mods**

## Example Mod

Check out `example_mod/` for a working example that demonstrates:
- How to structure a mod
- How to add new guards
- How to include custom sprites
- Data format reference

Read `example_mod/README.md` for full modding tutorial!

## Creating Your Own Mod

Want to create mods? It's easy!

1. **Read the Documentation**: See `/MODDING_SYSTEM.md` in the game root
2. **Use the Example**: Copy `example_mod/` as a template
3. **Edit Data Files**: Modify `.ron` files to add your content
4. **Add Assets**: Include your sprites, audio, etc.
5. **Test**: Run the game with hot-reload enabled
6. **Share**: Publish your mod on Steam Workshop or itch.io

### Quick Start

```bash
# Copy example mod as template
cp -r example_mod my_awesome_mod

# Edit mod metadata
nano my_awesome_mod/mod.ron

# Add your content
nano my_awesome_mod/data/guards.ron

# Test it
cargo run --features dev
```

## Mod Management

### Enabling/Disabling Mods

In-game:
1. **Settings → Mods**
2. Check/uncheck mods in the list
3. Click **"Apply"** to reload

Manual (`mods_config.ron`):
```ron
(
    enabled_mods: [
        (id: "my_mod", enabled: true, priority: 100),
        (id: "other_mod", enabled: false, priority: 200),
    ],
)
```

### Load Order

Mods load in priority order (higher number = later):
- **Base game** = priority 0
- **Most mods** = priority 100-1000
- **Override mods** = priority 1000+

Later-loading mods can override earlier ones.

### Conflicts

If two mods modify the same thing:
- The mod with **higher priority** wins
- The game will show a warning
- You can adjust priority in **Settings → Mods**

## Finding Mods

- **Steam Workshop** (if on Steam)
- **itch.io** - Tag: "path-of-goodies"
- **GitHub** - Search for "path-of-goodies-mod"
- **Discord** - #mod-showcase channel
- **In-Game Browser** - Built-in mod directory

## Mod Safety

✅ **Safe** - Data-only mods (.ron files, sprites, audio)
⚠️ **Be Careful** - Mods with scripts (Lua/Rhai) - only from trusted sources
❌ **Dangerous** - Never run mods with .exe or suspicious files

The game validates mods before loading and sandboxes scripts for security.

## Troubleshooting

### Mod Won't Load
- Check that `mod.ron` exists and has valid syntax
- Verify game version compatibility
- Look for errors in console/logs
- Try disabling other mods to find conflicts

### Missing Assets
- Ensure sprite paths are correct in data files
- Check file names match (case-sensitive on Linux/Mac)
- Verify sprites are in PNG format with transparency

### Game Crashes
- Disable recently installed mods
- Check mod compatibility with game version
- Report crash with mod list to #modding-help

### Hot-Reload Not Working
- Ensure dev mode is enabled: `cargo run --features dev`
- Check that file watcher is running (console logs)
- Try manual reload: Press F5 (if enabled)

## Getting Help

- **Documentation**: `/MODDING_SYSTEM.md`
- **Discord**: #modding-help channel
- **Example**: `example_mod/README.md`
- **GitHub Issues**: Report bugs
- **Community Wiki**: (coming soon)

## Contributing

Made an awesome mod? Share it!

1. Test thoroughly
2. Write clear README
3. Package with `pog-mod-tool package`
4. Upload to Steam Workshop / itch.io
5. Share in community Discord

**Featured Mods** (coming soon):
- Best mods showcased on official website
- Monthly mod contests with prizes
- "Mod of the Month" recognition

## License

Mods can have their own licenses. Check each mod's LICENSE or README file. The example mod is MIT licensed (free to use and modify).

---

**Happy Modding!** 🎨🎮

Join our modding community on Discord!
