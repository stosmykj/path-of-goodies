# Path of Goodies - Encounter & Event Brainstorming

**Version**: 0.2
**Last Updated**: 2025-10-21
**Status**: Creative Design / Brainstorm

---

## Encounter Categories

### 1. Combat Encounters (Covered in COMBAT_SYSTEM_DESIGN.md)

- Bandit ambushes
- Wild animal attacks
- Rival traders (PvP)
- Monster encounters

---

### 2. Social Encounters

#### 2.1 Travelers on the Road

**Friendly Merchant**:
```
Event: Meet traveling merchant
    ↓
Options:
├─ Trade goods (swap cargo items)
├─ Buy/sell supplies (better prices than towns)
├─ Share information (learn about dangers ahead)
└─ Just chat (lore, world-building)
```

**Pilgrims**:
```
Event: Religious pilgrims traveling to shrine
    ↓
They offer:
├─ Blessing (+10% luck for next encounter)
├─ Traveling company (join caravan temporarily, +morale)
└─ Warning about bandits ahead
```

**Noble's Caravan**:
```
Event: Wealthy noble with large escort
    ↓
Options:
├─ Request protection (travel with them, safety)
│   Cost: Must follow their slow pace
│
├─ Offer services (deliver message for payment)
│   Reward: 50 gold + reputation with nobility
│
└─ Ignore (avoid interaction)
```

**Deserter Soldiers**:
```
Event: Former soldiers, looking for work
    ↓
Options:
├─ Hire as guards (cheap but low loyalty)
│   Risk: May abandon if battle goes badly
│
├─ Offer food (humanitarian choice)
│   Cost: 20 food
│   Reward: +reputation with common folk
│
├─ Report to authorities
│   Reward: 30 gold bounty
│   Risk: -reputation with outlaws
│
└─ Ignore
```

---

#### 2.2 Settlements Encounters

**Tavern Events** (at destinations):

```
"The Mysterious Stranger":
- Hooded figure offers secret contract
- High pay, illegal cargo
- Risk: Guards chase if caught
- Reward: 3x normal payment

"The Drunk Guard":
- Guard reveals bandit hideout location
- Can raid for loot OR avoid area
- May be a trap (50% chance)

"The Bard's Tale":
- Listen to bard's story (lore)
- Learn about legendary treasure location
- Side quest opportunity

"The Gambler":
- Card game / dice game
- Risk gold for potential big win
- Skill-based or luck-based mini-game
```

**Town Requests**:

```
"Missing Person":
- Child went missing in forest
- Search and rescue mission
- Reward: Small gold, big reputation
- Time-sensitive (deadline)

"Livestock Escaped":
- Farmer's pigs/chickens escaped
- Catch and return
- Reward: Free food supplies
- Humorous, lighthearted

"Bandit Threat":
- Town asks you to deal with bandits
- Combat mission
- Reward: Gold + Hero status
```

---

### 3. Environmental Encounters

#### 3.1 Natural Hazards

**Storm**:
```
Dark clouds gathering
    ↓
Wagon slowed by heavy rain
    ↓
Options:
├─ Seek shelter (stop, wait it out)
│   Cost: Time (2-3 minutes real-time)
│   Benefit: Avoid damage
│
├─ Push through storm
│   Risk: Wagon takes damage (-20 durability)
│   Risk: Cargo gets wet (ruined goods)
│   Benefit: No time lost
│
└─ Use tarp (if have item)
    Cost: 1 tarp (consumable)
    Benefit: Protect cargo, continue travel
```

**Flooding / River Crossing**:
```
River ahead, bridge washed out
    ↓
Options:
├─ Ford the river (drive through)
│   Risk: Wagon damage, cargo loss
│   Skill check: Based on player skill
│   Horse may panic
│
├─ Find another route (detour)
│   Cost: Extra 5-10 minutes travel
│   Benefit: Safe
│
├─ Build temporary bridge
│   Cost: Time (3 minutes) + materials
│   Requires: Wood (if have)
│   Benefit: Safe crossing, bridge remains for others
│
└─ Wait for water to recede
    Cost: 8 hours (full day-night cycle)
    Benefit: Safe, but lose time
```

**Rockslide / Avalanche**:
```
Mountain path blocked
    ↓
Options:
├─ Clear path (manual labor)
│   Cost: Time (2 minutes) + stamina
│   Player exhaustion
│
├─ Climb over (risky)
│   Skill check
│   Success: Continue
│   Failure: Take damage, cargo lost
│
└─ Turn back (find other route)
    Major detour
```

**Fog**:
```
Dense fog reduces visibility
    ↓
Effects:
├─ Movement speed -40%
├─ Cannot see ambushes (surprise attacks)
├─ May get lost (wrong turn risk)
└─ Eerie atmosphere
    ↓
Options:
├─ Slow down and be cautious
│   Extra slow but safe
│
├─ Push forward at normal speed
│   Risk getting lost
│
└─ Light torch (if have)
    Reveals small area, helps
```

---

#### 3.2 Landmarks & Discoveries

**Ancient Ruins**:
```
Discover old ruins off the path
    ↓
Options:
├─ Explore ruins
│   Risk: Traps, monsters
│   Reward: Treasure, artifacts, lore
│   Time: 5-10 minutes
│
├─ Camp at ruins
│   Benefit: Shelter, atmospheric
│   Risk: Haunted? (rare event)
│
└─ Mark on map and continue
    Can return later
```

**Shrine / Temple**:
```
Sacred shrine to ancient god
    ↓
Options:
├─ Pray at shrine
│   Offerings (gold, food, items)
│   Blessings received:
│   ├─ +20% health regen
│   ├─ +luck for next combat
│   └─ Random buff (10 minutes duration)
│
├─ Desecrate shrine (evil choice)
│   Immediate reward: Valuable item
│   Curse: -luck, bad events more frequent
│
├─ Rest at shrine (safe camp spot)
│   Free camping, no hostile spawns
│   Peaceful music
│
└─ Ignore and continue
```

**Scenic Vista**:
```
Beautiful viewpoint (mountain, cliff)
    ↓
Options:
├─ Take a moment (roleplay)
│   Morale +20 all party
│   Screenshot opportunity
│   Relaxing music
│
├─ Search area
│   May find hidden cache
│   5% chance rare item
│
└─ Continue without stopping
```

**Mysterious Monolith**:
```
Strange glowing stone
    ↓
Options:
├─ Touch monolith
│   ??? (Random effect)
│   Could be good, could be bad
│   Magical mystery
│
├─ Study inscriptions
│   Lore dump
│   Learn ancient language
│   Unlock secret knowledge
│
├─ Photograph/sketch (if have item)
│   Sell to scholar later
│
└─ Avoid (seems dangerous)
```

---

### 4. Moral Dilemma Encounters

**Wounded Traveler**:
```
Injured person on roadside
    ↓
Options:
├─ Help (use medical supplies)
│   Cost: Bandage + time
│   Reward: Good karma, reputation
│   May join as companion
│
├─ Give food and leave
│   Cost: 10 food
│   Partial karma
│
├─ Steal their possessions
│   Reward: 20 gold, items
│   Karma: Evil choice, -reputation
│
└─ Ignore completely
    Neutral, no consequences
```

**Starving Village**:
```
Village hit by famine, people desperate
    ↓
Options:
├─ Donate food
│   Cost: 50 food
│   Reward: Hero status, +50 reputation
│   Unlock special contracts later
│
├─ Sell food at high price
│   Profit: 3x normal value
│   Reputation: Neutral/slight negative
│
├─ Donate some, sell some
│   Balanced approach
│
└─ Leave (nothing to give)
    No consequences
```

**Hostage Situation**:
```
Bandits holding civilian hostage
    ↓
Options:
├─ Fight to rescue
│   Combat encounter
│   Save hostage = companion or reward
│
├─ Pay ransom
│   Cost: 100 gold
│   Hostage freed peacefully
│
├─ Negotiate
│   Speech check (skill-based)
│   Success: Free hostage, no fight
│   Failure: Combat with disadvantage
│
└─ Walk away
    Hostage dies
    Guilt (morale penalty)
```

**Thief Caught**:
```
You catch someone stealing from wagon
    ↓
Options:
├─ Turn over to guards (lawful)
│   Thief imprisoned
│   +reputation with law
│
├─ Let them go (merciful)
│   Thief escapes
│   +reputation with outlaws
│   May return to help later
│
├─ Hire them (pragmatic)
│   Thief becomes guard/companion
│   Unique abilities (lockpicking)
│
└─ Execute on spot (evil)
    Harsh justice
    -reputation with most factions
```

---

### 5. Economic Encounters

**Merchant Convoy**:
```
Large merchant caravan
    ↓
Options:
├─ Trade cargo items
│   Swap goods for profit
│   Market arbitrage
│
├─ Join convoy for safety
│   Travel together
│   Share guard costs
│
├─ Sell contract cargo (illegal)
│   Immediate profit
│   Fail delivery mission
│   Massive reputation loss
│
└─ Continue separately
```

**Black Market**:
```
Secret dealer in shadows
    ↓
Offers:
├─ Illegal goods (high profit delivery)
├─ Stolen equipment (cheap, risk)
├─ Fake documents (evade guards)
└─ Information (enemy locations)
    ↓
Risk: If caught by guards, serious consequences
```

**Toll Bridge**:
```
Bridge guarded, toll required
    ↓
Options:
├─ Pay toll
│   Cost: 10-20 gold (fair)
│
├─ Bribe less
│   Speech check
│   Success: 5 gold
│   Failure: 30 gold (offended)
│
├─ Fight guards (evil)
│   Combat
│   Win: Free passage
│   Consequences: Wanted criminal
│
└─ Find ford (go around)
    Detour, slower
```

---

### 6. Supernatural / Mystery Encounters

**Ghost Wagon**:
```
Encounter spectral wagon on road
    ↓
Atmospheric, spooky music
    ↓
Options:
├─ Investigate
│   Lore about previous trader who died
│   Find their lost cargo (loot)
│   Potentially cursed
│
├─ Follow ghost
│   Leads to hidden treasure
│   OR leads to trap
│
└─ Flee area
    Safe but lose opportunity
```

**Witch in the Woods**:
```
Old woman at cauldron
    ↓
Options:
├─ Request potion
│   Pay 20 gold
│   Get powerful buff potion
│
├─ Trade ingredients
│   Give herbs/items
│   Receive rare consumables
│
├─ Attack (evil)
│   Difficult fight
│   Cursed if you win
│
└─ Politely decline
```

**Fairy Ring / Mushroom Circle**:
```
Glowing mushrooms in circle
    ↓
Fey magic zone
    ↓
Options:
├─ Step inside circle
│   Teleport to random location
│   Could be helpful (skip ahead)
│   Could be harmful (go backwards)
│
├─ Pick mushrooms
│   Alchemical ingredients
│   Risk: Fairy curse
│
├─ Leave offering
│   Fey boon (buff)
│
└─ Avoid entirely
```

**Time Anomaly**:
```
Reality warps, time distorts
    ↓
Effects:
├─ Time speeds up (travel faster)
├─ Time slows down (delayed)
├─ Time loops (repeat section)
└─ Time skip (suddenly at destination)
    ↓
Rare, mysterious, unexplained
Players speculate about lore
```

---

### 7. Random Fun Encounters

**Traveling Circus**:
```
Circus performers on road
    ↓
Options:
├─ Watch performance
│   Entertainment, morale boost
│   Mini-game (juggling, etc.)
│
├─ Hire performer
│   Bard joins as companion
│   Buffs party with music
│
├─ Trade exotic goods
│
└─ Continue journey
```

**Wild Horse Herd**:
```
Beautiful wild horses galloping
    ↓
Opportunity to tame
Multiple horses available
Easier than solo wild horse
```

**Meteor Shower**:
```
Falling stars in night sky
    ↓
Beautiful visual
Party stops to watch
Morale boost
    ↓
Meteors hit ground nearby
Can collect meteorite fragments
Sell for high price
```

**Festival/Fair**:
```
Arrive at settlement during festival
    ↓
Activities:
├─ Games (dice, cards, archery)
├─ Contest (wagon race)
├─ Special market (rare goods)
└─ Quest opportunities
```

**Pack of Puppies**:
```
Abandoned puppies on road
    ↓
Options:
├─ Adopt puppy
│   Pet companion (cosmetic)
│   Morale boost
│   Eats food but worth it
│
├─ Take to nearest town
│   Good deed
│   Reputation +
│
└─ Leave them (sad choice)
```

---

### 8. Chase Sequences

**Being Pursued**:

```
Bandits spotted chasing you!
    ↓
Real-time chase mini-game
    ↓
Options:
├─ Whip horse (speed boost)
│   Escape if fast enough
│   Horse exhaustion risk
│
├─ Drop cargo
│   Lighten load, speed up
│   Bandits stop to loot
│   Lose cargo, fail delivery
│
├─ Fight (stand and defend)
│   Turn around, combat
│
└─ Negotiate while fleeing
    Throw gold, they may stop
```

**Chasing Someone**:

```
Thief stole cargo!
    ↓
Chase mini-game (reverse)
    ↓
Catch up and recover goods
OR lose them forever
```

---

### 9. Weather Event Encounters

**Rainbow After Storm**:
```
Beautiful rainbow appears
    ↓
Follow to end?
    ├─ Find treasure (50% chance)
    └─ Nothing (50% chance)
    ↓
Whimsical, lighthearted
```

**Aurora Borealis**:
```
Northern lights visible
    ↓
Magical atmosphere
Rare random event
Morale boost
Photo opportunity
```

**Eclipse**:
```
Sun darkens (solar eclipse)
    ↓
Supernatural creatures appear
Special encounters available only now
Rare loot
```

---

### 10. Story/Lore Encounters

**Old Veteran**:
```
Retired soldier tells war stories
    ↓
Lore about kingdom history
Hints about future events
May recruit as strong companion
```

**Historian**:
```
Scholar studying ancient texts
    ↓
Teaches about world lore
Quests to find artifacts
Unlock hidden locations
```

**Prophecy**:
```
Oracle/seer predicts your future
    ↓
Cryptic message
Hints at upcoming events
Player speculation
```

---

## Encounter Frequency & Balancing

### Encounter Rate Formula

```rust
fn calculate_encounter_rate(
    path_danger: u8,      // 1-10
    biome: Biome,
    time_of_day: TimeOfDay,
    player_reputation: i32,
) -> f32 {
    let base_rate = 0.1; // 10% per minute

    let danger_mult = path_danger as f32 / 10.0;
    let biome_mult = match biome {
        Biome::Road => 0.5,
        Biome::Grassland => 1.0,
        Biome::Forest => 1.5,
        Biome::Mountain => 1.3,
        Biome::Desert => 1.2,
    };

    let time_mult = match time_of_day {
        TimeOfDay::Day => 1.0,
        TimeOfDay::Dusk => 1.2,
        TimeOfDay::Night => 1.5,
        TimeOfDay::Dawn => 1.1,
    };

    // High reputation = fewer bandit attacks
    let reputation_mult = if player_reputation > 50 {
        0.8
    } else if player_reputation < -50 {
        1.3 // Outlaws seek you out
    } else {
        1.0
    };

    base_rate * danger_mult * biome_mult * time_mult * reputation_mult
}
```

### Encounter Distribution

**Suggested mix** (per hour of gameplay):

```
Combat: 40%
├─ Bandits: 20%
├─ Wildlife: 15%
└─ Other enemies: 5%

Social: 25%
├─ Friendly travelers: 15%
└─ Moral dilemmas: 10%

Environmental: 20%
├─ Natural hazards: 10%
└─ Discoveries: 10%

Economic: 10%

Supernatural: 5% (rare)
```

---

## Summary

**Encounter Variety**:
✅ **Combat** - Action and danger
✅ **Social** - Choices and roleplay
✅ **Environmental** - Natural challenges
✅ **Moral** - Meaningful decisions
✅ **Economic** - Trade opportunities
✅ **Supernatural** - Mystery and wonder
✅ **Fun** - Lighthearted moments
✅ **Story** - Lore and world-building

**Design Goals**:
- Never boring (something every 2-3 minutes)
- Varied (not repetitive)
- Meaningful choices (consequences matter)
- Moddable (all data-driven)

**Implementation Priority** (MVP):
1. Basic combat encounters
2. 2-3 social encounters
3. 1-2 environmental hazards
4. 1 moral dilemma
5. Expand in later phases

---

**Status**: Creative brainstorm - many ideas
**Next**: Prioritize for MVP, design data structures
**All moddable**: Community can add infinite encounters!
