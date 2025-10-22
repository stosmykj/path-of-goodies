# Game Concept: Path of Goodies

## Core Vision

**Path of Goodies** is a strategic resource management game disguised as a medieval trading adventure. The player doesn't directly fight or solve puzzles - instead, they make **meaningful choices** about resource allocation, risk management, and party composition while traveling dangerous roads.

## Design Philosophy

### 1. Choices Over Reflexes
- No twitch-based combat or quick-time events
- Every decision has weight and consequences
- Strategic planning matters more than mechanical skill
- Players should feel smart, not just fast

### 2. Meaningful Risk/Reward
- Every journey is a gamble - risk resources for reward
- Encounters offer choices, not just combat
- Losing should teach, not frustrate
- Success should feel earned, not random

### 3. Resource Tension
- Resources are always scarce but manageable
- Multiple competing priorities (food, gold, equipment)
- Short-term needs vs long-term investments
- No single "correct" solution

### 4. Living World
- The world exists without the player
- Other traders, bandits, travelers populate roads
- Villages have different sizes, services, prices
- Horse is a companion, not just a stat block

## Core Gameplay Pillars

### Pillar 1: Strategic Travel
**What**: Navigate a graph-based world map between villages
**Why**: Planning optimal routes creates strategic depth
**How**:
- View world map showing villages and paths
- Fog of war reveals world gradually
- Choose destinations based on contracts and resources
- Path length affects encounter chance and resource consumption

**Player Questions**:
- Which route is safer vs faster?
- Do I have enough supplies for this journey?
- Should I explore new areas or stick to known paths?

### Pillar 2: Resource Management
**What**: Balance food, water, gold, and horse needs
**Why**: Scarcity creates tension and meaningful choices
**How**:
- Consume resources during travel and at destinations
- Earn gold from contracts
- Spend gold on supplies, equipment, services
- Horse health/stamina affect travel speed and survival

**Player Questions**:
- Can I afford to hire guards this trip?
- Should I rest now or push to the next town?
- Do I buy food or save for equipment?

### Pillar 3: Encounter Decisions
**What**: Face random events with multiple choice outcomes
**Why**: Creates stories and tests player judgment
**How**:
- Distance-based encounter probability
- Each encounter offers 2-4 choices
- Outcomes affect resources, time, party status
- Some encounters have hidden benefits/consequences

**Player Questions**:
- Fight, flee, or negotiate?
- Risk helping strangers or ignore them?
- Trust merchants or suspect scams?

### Pillar 4: Party Building
**What**: Hire, equip, and manage guards for protection
**Why**: Investment creates attachment and strategic options
**How**:
- Hire guards in towns (limited slots, daily salary)
- Equip with weapons/armor from blacksmith
- Guards affect encounter outcomes
- Guards can die or leave if morale is low

**Player Questions**:
- How many guards can I afford?
- Is expensive equipment worth the investment?
- Should I keep guards between contracts?

## Target Experience

### What We Want Players to Feel:

**Tension Without Stress**
- Close calls that make you sweat
- But not so hard you rage quit
- "I made it by the skin of my teeth!"

**Triumph Through Planning**
- Arriving safely with resources to spare
- Successfully completing a risky contract
- "My strategy worked perfectly!"

**Interesting Failure**
- Losing should be a story, not a frustration
- "I almost made it, but the second bandit attack did me in"
- Learn from mistakes and try again

**Emergent Narrative**
- Each run creates unique stories
- "Remember when I hired that expensive guard and he saved my life?"
- Player creates their own legend

## What This Game IS:

✅ Strategic resource management
✅ Risk/reward decision making
✅ Route planning and optimization
✅ Party composition strategy
✅ Emergent storytelling
✅ Replayable with variety

## What This Game IS NOT:

❌ Action/reflex-based gameplay
❌ Story-driven narrative game
❌ Open-world sandbox
❌ Roguelike with permadeath
❌ Complex trading simulator
❌ Idle/passive game

## Unique Selling Points

### 1. The Horse Is Real
Unlike most games where mounts are just speed modifiers, your horse is a living companion:
- Has health, stamina, exhaustion, morale
- Can die from overwork or injury
- Morale affects performance
- Forms emotional connection with player

### 2. Travels Are Journeys
Each trip between towns is an event with:
- Preparation phase (buy supplies, hire guards)
- Journey phase (encounters, resource consumption)
- Arrival phase (delivery, rewards, rest)
- No instant teleportation or fast travel

### 3. Meaningful Non-Combat Encounters
Not every encounter is a fight:
- Merchants offering deals
- Travelers sharing information
- Natural obstacles requiring solutions
- Moral dilemmas with consequences

### 4. Strategic Party Management
Guards aren't just stat boosts:
- Have individual types (archer, swordsman, etc.)
- Level up from experience
- Cost daily salaries
- Can be equipped with gear
- Affect encounter outcomes

## Target Audience

### Primary Audience:
- Players who enjoy strategy over action
- Fans of FTL, Oregon Trail, Darkest Dungeon
- Ages 18-45
- Prefer thoughtful gameplay over twitch reactions

### Secondary Audience:
- Roguelite fans looking for less punishing games
- Resource management enthusiasts
- Pixel art aesthetic lovers
- Players who enjoy emergent storytelling

## Success Metrics

### Core Metric: Time to First Success
Players should feel capable by their 2nd or 3rd run

### Engagement Metrics:
- Average session length: 15-30 minutes
- Completion rate: 60%+ of runs reach destination
- Replay rate: Players attempt 5+ runs

### Emotional Metrics:
- Players share "close call" stories
- Players name their horses
- Players develop favorite strategies
- Players feel clever when winning

## Design Constraints

### Scope Constraints:
- Single-player only (no multiplayer)
- 2D pixel art (no 3D modeling required)
- Turn-based/pause-able (no real-time pressure)
- Modular systems (easy to balance and extend)

### Technical Constraints:
- Runs on web browsers (WASM support required)
- Small download size (<50MB with assets)
- Moddable (RON data files for easy customization)
- Accessible (keyboard only, no controller required)

### Gameplay Constraints:
- No grinding or padding
- Clear feedback for all actions
- Recoverable from mistakes (not permadeath)
- Respects player time (15-30 min sessions)

## Inspirations

### Direct Inspirations:
- **FTL**: Strategic decision-making and encounter variety
- **Oregon Trail**: Resource management during travel
- **Darkest Dungeon**: Party management and stress systems
- **Slay the Spire**: Strategic choice trees and replayability

### Thematic Inspirations:
- Medieval trading routes
- Historical caravan traders
- Fantasy adventure settings
- Western frontier survival

## Open Questions (To Be Resolved)

1. **Permadeath vs Progressive**: Should players lose everything on failure?
2. **Meta-progression**: Should there be unlocks between runs?
3. **Story elements**: How much narrative vs pure mechanics?
4. **Difficulty scaling**: How to challenge veterans without punishing newcomers?
5. **End goal**: Is there a "win condition" or endless play?

## Next Steps

1. Define detailed mechanics for each system
2. Create mathematical models for resource balance
3. Design sprite specifications and mockups
4. Build prototypes to test core gameplay loop
5. Iterate based on playtesting feedback

---

**Version**: 1.0
**Last Updated**: 2025-10-22
**Status**: DRAFT - Open for revision
