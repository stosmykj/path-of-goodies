# Path of Goodies - Resource & Consumption System

**Version**: 0.2
**Last Updated**: 2025-10-21
**Status**: Detailed Design

---

## Philosophy

**Resources consumed at discrete points, NOT during travel.**

This design keeps travel focused on movement and combat while making camping and destination arrival strategic decision points.

---

## Core Resources

```rust
#[derive(Component)]
struct Resources {
    food: f32,          // 0-100 units
    water: f32,         // 0-100 units
    gold: i32,          // Currency

    // Capacity
    max_food: f32,      // Wagon capacity
    max_water: f32,
}
```

---

## Consumption Points

### 1. Rest at Camp

**When player sets up camp and chooses "REST"**:

```
Player clicks [REST] at camp
    ↓
Consumption calculation:
    ├─ Food consumed: 10 units × party size
    ├─ Water consumed: 15 units × party size
    └─ Time passes: 8 hours
    ↓
Benefits received:
    ├─ Player health: Full restore
    ├─ Horse stamina: Full restore
    ├─ Horse exhaustion: Reset to 0
    ├─ Guard health: Partial restore (50%)
    └─ Morale: +20 all
```

**Consumption Formula**:

```rust
fn calculate_camp_consumption(party: &Party) -> (f32, f32) {
    let base_food = 10.0;
    let base_water = 15.0;

    let party_size = 1 // Player
        + party.guards.len()
        + if party.companion.is_some() { 1 } else { 0 }
        + if party.horse.is_some() { 1 } else { 0 };

    let food_consumed = base_food * party_size as f32;
    let water_consumed = base_water * party_size as f32;

    (food_consumed, water_consumed)
}
```

**UI Display**:

```
╔════════════════════════════════════════╗
║ REST AT CAMP?                          ║
╠════════════════════════════════════════╣
║                                        ║
║ Party size: 4 (You, Horse, 2 Guards)  ║
║                                        ║
║ Cost:                                  ║
║ ├─ Food:  40 units                    ║
║ └─ Water: 60 units                    ║
║                                        ║
║ Benefits:                              ║
║ ├─ ❤️ Full health restore             ║
║ ├─ 🐴 Horse fully rested              ║
║ ├─ 🛡️ Guards heal 50%                ║
║ └─ ⏰ 8 hours pass                    ║
║                                        ║
║ Current supplies:                      ║
║ Food:  ▓▓▓▓▓▓░░░░ 65/100             ║
║ Water: ▓▓▓▓▓▓▓░░░ 75/100             ║
║                                        ║
║ After rest:                            ║
║ Food:  ▓▓▓░░░░░░░ 25/100  ⚠️         ║
║ Water: ▓░░░░░░░░░ 15/100  ⚠️         ║
║                                        ║
║   [REST]        [CANCEL]               ║
╚════════════════════════════════════════╝
```

---

### 2. Arrival at Destination

**When player reaches destination (settlement)**:

```
Wagon arrives at settlement
    ↓
Automatic consumption:
    ├─ Food consumed: 5 units × party size
    ├─ Water consumed: 8 units × party size
    └─ Reason: "Settling in for the evening"
    ↓
Player enters settlement
    ↓
Can resupply at market
```

**Arrival Consumption** (smaller than camping):

```rust
fn calculate_arrival_consumption(party: &Party) -> (f32, f32) {
    let base_food = 5.0;   // Half of camp amount
    let base_water = 8.0;

    let party_size = count_party_members(party);

    (base_food * party_size as f32, base_water * party_size as f32)
}
```

---

### 3. No Consumption During Travel

**Important Design Decision**:

✅ **NO resource drain while traveling**
- Focuses gameplay on route planning and combat
- Makes camping a strategic choice, not mandatory
- Simpler to understand and balance
- Travel time matters (longer journey = more likely to camp)

❌ **NOT realistic** (people eat while traveling)
- But realism < fun gameplay
- Consumption at discrete points is clearer
- Less micromanagement

---

## Running Out of Resources

### Out of Food

**Effects when Food = 0**:

```
No food remaining
    ↓
Cannot rest at camp
    ├─ "REST" button disabled
    └─ Tooltip: "Not enough food to rest"
    ↓
Party gets "Hungry" debuff:
    ├─ -10% movement speed
    ├─ -5% combat effectiveness
    └─ Morale drops over time
    ↓
After 3 camps skipped (or long time):
    ├─ "Starving" debuff
    ├─ -25% movement speed
    ├─ -15% combat effectiveness
    └─ Health slowly decreases
```

**Starvation Mechanic**:

```rust
#[derive(Component)]
struct HungerState {
    level: HungerLevel,
    time_without_food: f32,  // Seconds
}

enum HungerLevel {
    Fed,        // Full resources
    Hungry,     // Out of food, < 1 day
    Starving,   // Out of food, > 1 day
    Critical,   // Near death from starvation
}

fn update_hunger(
    mut query: Query<(&Resources, &mut HungerState, &mut Health)>,
    time: Res<Time>,
) {
    for (resources, mut hunger, mut health) in query.iter_mut() {
        if resources.food == 0.0 {
            hunger.time_without_food += time.delta_secs();

            hunger.level = match hunger.time_without_food {
                t if t < 480.0 => HungerLevel::Hungry,      // < 8 min
                t if t < 960.0 => HungerLevel::Starving,    // < 16 min
                _ => HungerLevel::Critical,
            };

            // Damage from starvation
            if hunger.level == HungerLevel::Critical {
                health.current -= 0.5 * time.delta_secs();
            }
        } else {
            hunger.time_without_food = 0.0;
            hunger.level = HungerLevel::Fed;
        }
    }
}
```

### Out of Water

**Effects when Water = 0** (more severe than food):

```
No water remaining
    ↓
Cannot rest at camp
    ↓
"Thirsty" debuff (immediate):
    ├─ -15% movement speed
    ├─ -10% combat effectiveness
    └─ Horse stamina drains 2x faster
    ↓
After short time:
    ├─ "Dehydrated" debuff
    ├─ -35% movement speed
    ├─ -25% combat effectiveness
    ├─ Health decreases faster than starvation
    └─ Horse may collapse
```

**Dehydration is deadlier**:
- Water runs out faster (15 vs 10 per camp)
- Effects happen quicker
- More punishing in desert biomes

---

## Resource Management Strategy

### Planning Before Journey

**At settlement, before accepting contract**:

```
╔══════════════════════════════════════════════════╗
║ PREPARE FOR JOURNEY                              ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║ Contract: Grain to Northport                    ║
║ Distance: 500m                                   ║
║ Estimated travel time: 8-12 minutes             ║
║                                                  ║
║ Recommended supplies:                            ║
║ ├─ Food: 60 units (1-2 camps)                   ║
║ ├─ Water: 90 units (1-2 camps)                  ║
║ └─ Gold: 50 (emergency supplies)                ║
║                                                  ║
║ Current supplies:                                ║
║ Food:  ▓▓░░░░░░░░ 20/100  ⚠️ LOW!              ║
║ Water: ▓▓▓▓▓░░░░░ 45/100                       ║
║ Gold:  145                                       ║
║                                                  ║
║ Purchase supplies?                               ║
║ [MARKET] [DEPART ANYWAY] [CANCEL CONTRACT]      ║
╚══════════════════════════════════════════════════╝
```

### During Journey Decision

**Camp or Push Forward?**:

```
Scenario: Halfway to destination, party tired
    ↓
Options:
    ├─ Camp now:
    │   ├─ Cost: 40 food, 60 water
    │   ├─ Benefit: Full heal, horse rested
    │   ├─ Risk: May not have enough for second camp
    │   └─ Time: +8 hours (may miss deadline)
    │
    └─ Push forward:
        ├─ Cost: Nothing
        ├─ Benefit: Save resources, arrive faster
        ├─ Risk: Arrive exhausted, vulnerable to ambush
        └─ Horse may collapse if whipped
```

**Strategic Questions**:
- How far to destination?
- How much HP left?
- Horse exhaustion level?
- Can I afford to camp?
- Is there a deadline?

---

## Resupply System

### At Settlements

**Market UI**:

```
╔═══════════════════════════════════════════╗
║ MARKET - Northport Village                ║
╠═══════════════════════════════════════════╣
║                                           ║
║ Your Gold: 💰 145                        ║
║                                           ║
║ ┌─────────────────────────────────────┐  ║
║ │ SUPPLIES                            │  ║
║ ├─────────────────────────────────────┤  ║
║ │ 🍞 Food (10 units)      5 gold     │  ║
║ │    [BUY 1] [BUY 10] [MAX]          │  ║
║ │                                     │  ║
║ │ 💧 Water (10 units)     3 gold     │  ║
║ │    [BUY 1] [BUY 10] [MAX]          │  ║
║ │                                     │  ║
║ │ 🥕 Carrot (horse treat)  2 gold    │  ║
║ │    Restores 20 horse stamina       │  ║
║ │    [BUY 1] [BUY 5]                 │  ║
║ │                                     │  ║
║ │ 🍎 Apple (morale boost)  1 gold    │  ║
║ │    +10 morale to all               │  ║
║ │    [BUY 1] [BUY 5]                 │  ║
║ └─────────────────────────────────────┘  ║
║                                           ║
║ Current capacity:                         ║
║ Food:  25/100 (75 units available)       ║
║ Water: 15/100 (85 units available)       ║
║                                           ║
║ Quick Buy:                                ║
║ [FILL FOOD] (38 gold)                    ║
║ [FILL WATER] (26 gold)                   ║
║ [FILL BOTH] (64 gold)                    ║
║                                           ║
║              [DONE SHOPPING]              ║
╚═══════════════════════════════════════════╝
```

### Variable Pricing

**Prices vary by location**:

```ron
(
    settlements: [
        (
            id: "capital_city",
            name: "Capital City",

            market_prices: (
                food_price: 5,      // Standard
                water_price: 3,
                supply_abundance: High,  // Always stocked
            ),
        ),

        (
            id: "desert_outpost",
            name: "Desert Outpost",

            market_prices: (
                food_price: 7,      // Expensive (imported)
                water_price: 10,    // VERY expensive (scarce)
                supply_abundance: Low,
            ),
        ),

        (
            id: "riverside_village",
            name: "Riverside Village",

            market_prices: (
                food_price: 4,      // Cheap (farming)
                water_price: 1,     // Very cheap (river)
                supply_abundance: High,
            ),
        ),
    ],
)
```

---

## Emergency Resources

### Finding Resources

**Random events can provide resources**:

```ron
(
    events: [
        (
            id: "abandoned_wagon",
            name: "Abandoned Wagon",
            description: "You find an abandoned wagon with supplies!",

            rewards: [
                (item: "food", amount: (10, 30), chance: 0.8),
                (item: "water", amount: (15, 40), chance: 0.8),
                (item: "gold", amount: (5, 15), chance: 0.5),
            ],

            risks: [
                "May be a trap (10% chance ambush)",
                "Supplies may be spoiled (20% chance)",
            ],
        ),

        (
            id: "friendly_trader",
            name: "Traveling Merchant",
            description: "A friendly merchant offers to trade.",

            options: [
                (
                    choice: "Buy supplies",
                    cost: Gold(20),
                    reward: (food: 30, water: 40),
                ),
                (
                    choice: "Trade goods",
                    cost: CargoItem,
                    reward: (food: 50, water: 60),
                ),
                (
                    choice: "Decline",
                    cost: None,
                    reward: None,
                ),
            ],
        ),

        (
            id: "natural_spring",
            name: "Natural Spring",
            description: "You discover a fresh water spring!",

            biomes: [Forest, Grassland],

            rewards: [
                (item: "water", amount: (40, 80), chance: 1.0),
            ],

            bonus: "Can refill for free (fills to max)",
        ),

        (
            id: "berry_bush",
            name: "Wild Berry Bush",
            description: "Edible berries grow here.",

            biomes: [Forest],

            rewards: [
                (item: "food", amount: (5, 15), chance: 1.0),
            ],

            time_cost: 60.0,  // Takes 1 minute to gather
        ),
    ],
)
```

### Hunting/Foraging

**Optional mechanic** (not in MVP, but designed for):

```
Player near forest
    ↓
"Hunt Wildlife" action available
    ↓
Mini-game or automatic (based on skill)
    ↓
Success: Gain food (meat)
Failure: Waste time, scare away animals
    ↓
Requires: Bow, arrows, time
```

---

## Cargo Management

### Cargo vs Supplies

**Wagon capacity is shared**:

```rust
struct WagonCargo {
    max_capacity: f32,      // 500 kg total

    // Space allocation
    contract_cargo: f32,    // Must deliver (untouchable)
    food_weight: f32,       // 0.5 kg per unit
    water_weight: f32,      // 1.0 kg per unit
    items_weight: f32,      // Equipment, loot

    // Current weight
    total_weight: f32,
}

fn calculate_total_weight(cargo: &WagonCargo) -> f32 {
    cargo.contract_cargo
        + (cargo.food * 0.5)
        + (cargo.water * 1.0)
        + cargo.items_weight
}

fn can_add_supplies(cargo: &WagonCargo, food: f32, water: f32) -> bool {
    let new_weight = calculate_total_weight(cargo)
        + (food * 0.5)
        + (water * 1.0);

    new_weight <= cargo.max_capacity
}
```

**Trade-off**:
- More cargo = more money, but less supply capacity
- Less cargo = more supplies, safer journey
- Player must balance greed vs safety

---

## Weather & Biome Effects

### Biome-Specific Consumption

**Desert** (future expansion):

```
In desert biome:
    ├─ Water consumption: 2x at camps
    ├─ Horse thirst: Increases faster
    └─ Dehydration: Happens 2x faster
```

**Mountain** (future expansion):

```
In mountains:
    ├─ Food consumption: 1.5x (cold weather)
    ├─ Camping takes longer
    └─ May need firewood resource
```

---

## Tutorial & UI Communication

### First Camp Tutorial

```
Player sets up first camp
    ↓
Tutorial overlay:

╔══════════════════════════════════════════╗
║ 💡 CAMPING TUTORIAL                      ║
╠══════════════════════════════════════════╣
║                                          ║
║ When you REST at camp, you consume:     ║
║ • Food and water based on party size    ║
║ • Larger party = more consumption       ║
║                                          ║
║ Benefits of resting:                     ║
║ • Full health restore                    ║
║ • Horse fully rested                     ║
║ • Guards heal                            ║
║                                          ║
║ 💡 TIP: Plan your supplies wisely!      ║
║ Running out forces you to push forward  ║
║ without resting, which is dangerous.    ║
║                                          ║
║ 💡 TIP: You can resupply at settlements ║
║ but prices vary by location.            ║
║                                          ║
║          [GOT IT!]                       ║
╚══════════════════════════════════════════╝
```

### Warning System

**Low supplies warning**:

```
When food or water < 25%:

⚠️ "Food supplies running low!"
⚠️ "Water supplies critical!"

Yellow warning icon in HUD

When food or water = 0:

🔴 "OUT OF FOOD - Cannot rest at camp!"
🔴 "OUT OF WATER - Party dehydrated!"

Red critical icon, flashing
```

---

## Summary

**Simple but Strategic System**:

✅ **Discrete consumption** (camps & destinations only)
✅ **Strategic decisions** (camp now or push forward?)
✅ **Clear consequences** (run out = debuffs, eventually death)
✅ **Planning matters** (buy enough supplies before journey)
✅ **Trade-offs** (cargo space vs supplies)
✅ **Resupply opportunities** (markets, events)

**Not included in MVP** (future):
- Hunting/foraging mini-games
- Food spoilage over time
- Cooking system
- Weather effects on consumption
- Special rations (military rations, elven bread, etc.)

---

**Status**: Ready for implementation
**Dependencies**: Camping system, market UI
**Priority**: Phase 1 (Week 4 - Resource Management)
