# Resource System

## Overview

The resource system is the foundation of Path of Goodies' strategic gameplay. Players must constantly balance four primary resources (Food, Water, Gold, Horse Stamina) while traveling and making decisions.

## Core Design Principles

1. **Scarcity Creates Tension** - Resources should always feel limited
2. **Multiple Uses** - Each resource has several competing uses
3. **Trade-offs** - Gaining one resource often costs another
4. **Recoverable** - Running out shouldn't mean instant death
5. **Visible Consequences** - Clear feedback when resources run low

## Primary Resources

### 1. Food 🍞

**Purpose**: Keep the party alive during travel

**Starting Amount**: 30 units
**Maximum Capacity**: 100 units
**Weight**: Affects wagon speed if over 80 units

#### Consumption Rates:
```
Base consumption per destination: 5 food
+ 3 per guard in party
+ 2 per extra day of travel
Camping: 10 food (one full meal)
```

#### Acquisition:
- **Market/Inn**: 1.0-2.0 gold per unit (varies by town)
- **Encounters**: Find abandoned supplies (5-15 units)
- **Contracts**: Sometimes rewarded (rare)
- **Hunting** (future): Random encounter choice

#### Consequences of Low Food:
```
< 10 units: Warning message "Food supplies running low"
< 5 units: Party morale decreases (-5 per day)
= 0 units: Health decreases for all (-10 HP per day)
```

#### Design Notes:
- Food should run out in 5-6 trips without restocking
- Forces regular town visits
- Creates tension without being punishing
- Buffering allows some mistakes

---

### 2. Water 💧

**Purpose**: Essential survival resource, more critical than food

**Starting Amount**: 40 units
**Maximum Capacity**: 100 units
**Weight**: Heavier than food

#### Consumption Rates:
```
Base consumption per destination: 8 water
+ 4 per guard in party
+ 3 per extra day of travel
Camping: 15 water (full party needs)
Horse drinks: 5 water per rest
```

#### Acquisition:
- **Market/Inn**: 0.5-1.5 gold per unit
- **Wells**: Free 20 units (rare encounter)
- **Rivers**: Free 30 units (risky - disease chance)
- **Towns**: Cheaper than food usually

#### Consequences of Low Water:
```
< 15 units: Warning "Water is critical!"
< 8 units: Forced rest periods (slower travel)
= 0 units: Severe penalties:
  - Party health -20 HP per day
  - Horse stamina recovery -50%
  - Movement speed -30%
```

#### Design Notes:
- Water is more critical than food (dies faster)
- Should create urgency every 4-5 trips
- Higher consumption than food
- Dehydration effects are immediate

---

### 3. Gold 💰

**Purpose**: Universal currency for all purchases and services

**Starting Amount**: 100 gold
**Maximum Capacity**: Unlimited (but affects wagon weight if > 1000)

#### Income Sources:
```
Contract Completion:
- Easy contract: 30-60 gold
- Medium contract: 60-120 gold
- Hard contract: 120-250 gold
- Epic contract: 250-500 gold

Other Sources:
- Encounter rewards: 10-50 gold
- Merchant trading: Variable profit
- Treasure finds: 20-100 gold (rare)
- Town services (selling items): TBD
```

#### Expenditures:
```
Essential:
- Food: 1.0-2.0 per unit
- Water: 0.5-1.5 per unit
- Inn rest: 15-30 gold per party member

Optional:
- Guard salary: 5-20 per day per guard
- Equipment: 20-500 gold per item
- Temple services: 20-50 gold
- Horse care: 10-30 gold
- Repairs: 15-40 gold (if wagon damaged)
```

#### Consequences of No Gold:
```
Cannot:
- Buy supplies (forced to scavenge)
- Hire guards (travel alone)
- Rest at inns (must camp)
- Buy equipment (weak party)
- Use town services

Can still:
- Accept contracts
- Travel between towns
- Handle encounters
- Camp for free
```

#### Design Notes:
- Gold enables options, lack limits them
- Never instantly fail for lack of gold
- Early game: barely profitable
- Mid game: comfortable surplus
- Late game: strategic investments

---

### 4. Horse Stamina ⚡

**Purpose**: Determines travel speed and efficiency

**Starting Amount**: 100 stamina
**Maximum Capacity**: 100 stamina
**Regeneration**: Passive (see Horse System doc)

#### Consumption Rates:
```
Normal travel: -1 stamina per 10 distance units
Sprint/Whip: -5 stamina per second
Carrying heavy loads: +20% consumption
Low morale: +10% consumption
```

#### Regeneration:
```
Passive: +2 stamina per 10 seconds (standing still)
Camping: Full restore
Inn rest: Full restore
Horse blessing: +20 max stamina
```

#### Consequences of Low Stamina:
```
< 30: "Horse is tired" warning
< 10: Movement speed reduced to 50%
= 0: Forced stop, cannot move until regen
     Horse loses morale (-5)
     Risk of exhaustion damage
```

#### Design Notes:
- Creates pacing (can't rush forever)
- Rest points become strategic
- Whip is powerful but has cost
- Ties into horse health system

---

## Secondary Resources

### 5. Horse Health ❤️

**Starting Amount**: 100 HP
**Maximum Capacity**: 100 HP (can be buffed)

See [Horse System](HORSE.md) for detailed mechanics.

**Quick Reference**:
- Damaged in encounters
- Lost from overwork (stamina exhaustion)
- Lost from whipping too much
- Restored at inns, temples, camping
- **Death**: Horse can die (major setback)

---

### 6. Party Member Health 💚

**Starting Amount**: Varies by guard type (50-100 HP)
**Maximum Capacity**: Varies (50-150 HP)

See [Party System](PARTY.md) for detailed mechanics.

**Quick Reference**:
- Damaged in combat encounters
- Lost from starvation/dehydration
- Restored at inns, temples
- **Death**: Guards can die permanently

---

## Resource Economics

### Early Game (Contracts 1-3)

**Typical Income**: 40-80 gold per contract
**Typical Expenses**: 30-50 gold (food, water)
**Net Profit**: 10-30 gold per run

**Player State**:
- Living contract-to-contract
- Can't afford guards or equipment
- Must be efficient with resources
- Learning optimal routes

### Mid Game (Contracts 4-10)

**Typical Income**: 100-150 gold per contract
**Typical Expenses**: 60-100 gold (supplies + 1-2 guards)
**Net Profit**: 40-80 gold per run

**Player State**:
- Can afford 1-2 guards
- Building equipment collection
- Some margin for error
- Exploring harder contracts

### Late Game (Contracts 11+)

**Typical Income**: 200-400 gold per contract
**Typical Expenses**: 100-200 gold (full party + equipment)
**Net Profit**: 100-200 gold per run

**Player State**:
- Full party with equipment
- Comfortable resource buffer
- Taking risky contracts
- Optimizing for max profit

---

## Resource Management Strategies

### Conservative Strategy
**Focus**: Survival and consistency
```
- Always buy extra supplies
- Rest at inns (don't camp)
- Hire guards early
- Take easy contracts
- Avoid risky encounters
```

**Pros**: Very safe, consistent income
**Cons**: Slow progression, low profits

### Aggressive Strategy
**Focus**: Maximum profit
```
- Buy minimal supplies
- Camp instead of inns
- No guards (save salary)
- Take hard contracts
- Gamble on encounters
```

**Pros**: High profits when successful
**Cons**: High failure rate, risky

### Balanced Strategy
**Focus**: Sustainable growth
```
- Buy enough supplies + buffer
- Mix inns and camping
- 1-2 guards for safety
- Medium contracts
- Safe encounter choices
```

**Pros**: Good balance of risk/reward
**Cons**: Slower than aggressive, riskier than conservative

---

## Mathematical Models

### Travel Cost Formula
```rust
// Per destination
food_cost = 5 + (num_guards * 3) + (extra_days * 2)
water_cost = 8 + (num_guards * 4) + (extra_days * 3)
guard_salary = sum(guard.daily_salary) * days_traveled

total_cost = food_cost + water_cost + guard_salary
```

### Break-Even Analysis
```
Minimum contract reward for profit:
= food_cost + water_cost + guard_salary + supplies_buffer

Example (medium contract, 2 guards, 2 days):
Food: 5 + 6 + 4 = 15 units = 22g @ 1.5g/unit
Water: 8 + 8 + 6 = 22 units = 22g @ 1.0g/unit
Guards: 2 * 10g * 2 days = 40g
Buffer: 20g (safety margin)
------------------
Total Cost: 104g
Minimum Reward: 110g for profitability
```

### Optimization Goals
```
Target margins:
- Early game: 30-50% profit margin
- Mid game: 50-70% profit margin
- Late game: 60-80% profit margin

This allows:
- Equipment purchases without bankruptcy
- Margin for errors/bad encounters
- Sense of progression and wealth
```

---

## Resource UI Display

### HUD Elements

```
┌─────────────────────────────────────┐
│ 💰 Gold: 145                        │
│ 🍞 Food: 42 / 100                   │
│ 💧 Water: 38 / 100                  │
│ ⚡ Stamina: 67 / 100 [■■■■■■░░░░]   │
│ ❤️  Horse: 85 / 100  [■■■■■■■■░░]   │
└─────────────────────────────────────┘
```

### Warning States

```
When < 20% of max:
🍞 → 🍞❗ (yellow text)
💧 → 💧❗ (yellow text)
⚡ → ⚡❗ (yellow text)
❤️  → ❤️❗  (yellow text)

When < 10% of max:
🍞❗ → 🍞‼️  (red text + flashing)
```

### Tooltips
```
Hover over resource:
"Food: 42/100
 Consumption: ~5 per destination
 Trips remaining: ~8"
```

---

## Testing & Balance

### Playtesting Metrics

1. **Resource Depletion Rate**
   - Target: Run out in 5-7 trips without restocking
   - Too fast: Players frustrated
   - Too slow: No tension

2. **Profit Margins**
   - Target: 40-60% profit on average contract
   - Too high: Game too easy
   - Too low: Feels like grinding

3. **Close Calls**
   - Target: 20-30% of trips end with < 15% resources
   - Creates tension without punishment
   - "Made it by the skin of my teeth" feeling

4. **Resource Bottlenecks**
   - Identify which resource runs out first
   - Should vary based on strategy
   - All resources should matter

### Balance Knobs

```rust
// Easy to adjust these for balance
const BASE_FOOD_CONSUMPTION: f32 = 5.0;
const BASE_WATER_CONSUMPTION: f32 = 8.0;
const GUARD_FOOD_MULTIPLIER: f32 = 3.0;
const GUARD_WATER_MULTIPLIER: f32 = 4.0;
const STARTING_GOLD: f32 = 100.0;
const FOOD_PRICE_RANGE: (f32, f32) = (1.0, 2.0);
const WATER_PRICE_RANGE: (f32, f32) = (0.5, 1.5);
```

---

## Future Considerations

### Potential Additions:
1. **Repair Resources** - Wagon durability system
2. **Medicine** - Healing items for emergencies
3. **Trade Goods** - Buy low, sell high mechanics
4. **Reputation** - Affects prices and opportunities
5. **Seasonal Pricing** - Supply/demand fluctuations

### Questions to Resolve:
1. Should resources spoil/expire?
2. Should there be inventory weight limits?
3. Should gold have any physical weight?
4. Should water be more expensive in deserts?
5. Should food prices vary by town size?

---

**Version**: 1.0
**Last Updated**: 2025-10-22
**Status**: DRAFT - Ready for implementation
