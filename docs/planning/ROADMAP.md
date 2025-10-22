# Development Roadmap

## Project Timeline Overview

**Project Name**: Path of Goodies
**Target Completion**: TBD (estimate 6-12 months part-time)
**Current Phase**: Phase 1 - Planning
**Next Milestone**: Complete Phase 1, begin Phase 2

---

## Development Philosophy

### Iterative Development
- Build in small, testable increments
- Playtest frequently
- Iterate based on feedback
- Don't gold-plate before testing

### Vertical Slices
- Each phase should produce a playable version
- Complete features fully before moving on
- Avoid half-implemented systems

### Documentation First
- Design before coding
- Spec before building
- Think through edge cases early

---

## Phase 1: Foundation & Planning ⏳ (CURRENT)

**Duration**: 2-4 weeks
**Status**: 80% Complete

### Goals
- ✅ Define core game concept and vision
- ✅ Document all major systems
- ✅ Create sprite specifications
- ✅ List all required assets
- 🔄 Create technical architecture plan
- 🔄 Design initial sprites and mockups
- ❌ Create development environment setup guide

### Deliverables

#### Documentation ✅
- [X] Game Concept Document
- [X] Resource System Mechanics
- [X] World Map System Mechanics
- [X] Sprite Specifications
- [X] Complete Asset List
- [ ] Horse System Mechanics
- [ ] Encounter System Mechanics
- [ ] Party System Mechanics
- [ ] Contract System Mechanics
- [ ] Technical Architecture
- [ ] Data Structure Specifications

#### Assets
- [ ] Master color palette (PNG reference)
- [ ] 5-10 sample sprites (proof of concept)
- [ ] UI mockups (static images)
- [ ] World map mockup (concept art)

#### Technical
- [ ] Repository setup with proper structure
- [ ] Development environment guide
- [ ] Tool recommendations
- [ ] Asset pipeline defined

### Success Criteria
- All core systems documented with examples
- Asset specifications are clear and actionable
- Sample sprites demonstrate art style
- Team/solo dev can begin implementation with clarity

### Exit Criteria
- Documentation review complete
- All major questions answered
- Art style validated
- Ready to write first line of code

---

## Phase 2: Minimum Viable Prototype 🎯 (NEXT)

**Duration**: 4-6 weeks
**Status**: Not Started
**Goal**: Playable loop - travel from A to B with basic resource management

### Milestones

#### Milestone 2.1: Project Setup (Week 1)
**Tasks**:
- [ ] Create Rust/Bevy project
- [ ] Set up asset loading
- [ ] Configure build pipeline
- [ ] Test WASM export
- [ ] Create basic project structure

**Deliverable**: Empty project that compiles and runs

#### Milestone 2.2: World Map (Week 2-3)
**Tasks**:
- [ ] Implement graph data structure
- [ ] Generate procedural world
- [ ] Render villages and paths
- [ ] Implement camera controls
- [ ] Add fog of war basic logic
- [ ] Click-to-select villages

**Deliverable**: Interactive world map you can explore

**Required Assets**:
- Village icons (4 types)
- Path texture
- Background texture

#### Milestone 2.3: Travel System (Week 3-4)
**Tasks**:
- [ ] Implement path following
- [ ] Add wagon movement
- [ ] Add horse stamina system
- [ ] Resource consumption (food, water)
- [ ] Arrival at destination
- [ ] Simple HUD display

**Deliverable**: Can travel between two villages

**Required Assets**:
- Wagon sprite (idle)
- Horse sprite (idle)
- UI icons (gold, food, water, stamina)
- Basic terrain background

#### Milestone 2.4: Town Interaction (Week 4-5)
**Tasks**:
- [ ] Town entry/exit
- [ ] Buy food and water
- [ ] Simple UI for market
- [ ] Resource display
- [ ] ESC to leave town

**Deliverable**: Complete A→B→Buy→A loop

**Required Assets**:
- Town UI panels
- Button sprites
- Market interface

#### Milestone 2.5: Basic Contract (Week 5-6)
**Tasks**:
- [ ] Generate simple contracts
- [ ] Accept contract
- [ ] Track contract progress
- [ ] Complete contract for reward
- [ ] Win condition

**Deliverable**: **MVP COMPLETE** - Full gameplay loop

**Required Assets**:
- Contract UI
- Success/completion feedback

### Phase 2 Success Criteria
- [ ] Can generate random world
- [ ] Can select destination and travel
- [ ] Resources consumed during travel
- [ ] Can buy supplies in towns
- [ ] Can accept and complete contracts
- [ ] Game loop is satisfying at basic level
- [ ] No critical bugs
- [ ] Runs at 60 FPS

### Phase 2 Known Limitations
- No guards yet
- No encounters
- No horse death
- No equipment
- No save/load
- Very basic UI
- Limited testing

**→ First Playtest Checkpoint**

---

## Phase 3: Core Gameplay Features 🎮

**Duration**: 6-8 weeks
**Status**: Not Started
**Goal**: Add depth - encounters, guards, horse mechanics

### Milestone 3.1: Encounter System (Week 1-2)
**Tasks**:
- [ ] Random encounter probability
- [ ] Encounter UI
- [ ] 5 basic encounter types
- [ ] Choice system
- [ ] Outcome resolution
- [ ] Pause/resume travel

**Required Assets**:
- Encounter backgrounds
- Enemy sprites (3-5 types)
- Encounter UI panels

### Milestone 3.2: Party System (Week 3-4)
**Tasks**:
- [ ] Guard hiring in towns
- [ ] Guard types (4 types)
- [ ] Party member display
- [ ] Daily salary system
- [ ] Guards in combat
- [ ] Guard death

**Required Assets**:
- Guard sprites (4 types, idle + attack)
- Hire UI
- Party display UI

### Milestone 3.3: Horse Mechanics (Week 5-6)
**Tasks**:
- [ ] Horse health system
- [ ] Exhaustion mechanics
- [ ] Morale system
- [ ] Warning system
- [ ] Horse death
- [ ] Speed penalties
- [ ] Whip boost mechanic

**Required Assets**:
- Horse animation states (tired, critical)
- Horse death animation
- Warning UI

### Milestone 3.4: Equipment System (Week 7-8)
**Tasks**:
- [ ] Equipment data structures
- [ ] Blacksmith shop UI
- [ ] Purchase equipment
- [ ] Equip to guards
- [ ] Apply stat bonuses
- [ ] Equipment display

**Required Assets**:
- Equipment icons (10-15)
- Blacksmith UI
- Equipment slots display

### Phase 3 Success Criteria
- [ ] Encounters add variety and risk
- [ ] Guards meaningfully affect gameplay
- [ ] Horse feels like a living companion
- [ ] Equipment provides progression
- [ ] Mid-game loop is engaging
- [ ] Clear sense of growth and power

**→ Second Playtest Checkpoint**

---

## Phase 4: Content & Progression 📈

**Duration**: 4-6 weeks
**Status**: Not Started
**Goal**: More encounters, better balance, progression systems

### Milestone 4.1: Expanded Encounters (Week 1-2)
**Tasks**:
- [ ] 15+ encounter types
- [ ] Rare/legendary encounters
- [ ] Branching encounter chains
- [ ] Encounter consequences
- [ ] Better rewards/penalties

**Required Assets**:
- More NPC sprites
- Special encounter art
- Variety backgrounds

### Milestone 4.2: Town Services (Week 2-3)
**Tasks**:
- [ ] Inn rest mechanics
- [ ] Temple/healing services
- [ ] Blacksmith repairs
- [ ] Multiple service tiers
- [ ] Town reputation system

**Required Assets**:
- Service UI variants
- Town interior backgrounds (optional)

### Milestone 4.3: Contract Variety (Week 3-4)
**Tasks**:
- [ ] Multiple contract types
- [ ] Difficulty tiers
- [ ] Time limits
- [ ] Special conditions
- [ ] Chain contracts
- [ ] Reputation effects

**Required Assets**:
- Contract icons/categories
- Special cargo types (visual indicators)

### Milestone 4.4: Meta-Progression (Week 4-6)
**Tasks**:
- [ ] Unlock system (optional)
- [ ] Achievements
- [ ] Statistics tracking
- [ ] Leaderboard (local)
- [ ] New game+ mode (optional)

**Required Assets**:
- Achievement icons
- Stats display UI

### Phase 4 Success Criteria
- [ ] 50+ hours of unique content
- [ ] Replayability is high
- [ ] Multiple viable strategies
- [ ] Clear progression arc
- [ ] Endgame feels rewarding

**→ Third Playtest Checkpoint**

---

## Phase 5: Polish & Quality 💎

**Duration**: 4-6 weeks
**Status**: Not Started
**Goal**: Make it feel professional and complete

### Milestone 5.1: Visual Polish (Week 1-2)
**Tasks**:
- [ ] Particle effects
- [ ] Screen shake on events
- [ ] Smooth transitions
- [ ] Parallax backgrounds
- [ ] Weather effects
- [ ] Day/night cycle visuals

**Required Assets**:
- Particle sprites
- Effect animations
- Time-of-day backgrounds

### Milestone 5.2: Audio Implementation (Week 2-3)
**Tasks**:
- [ ] Background music system
- [ ] Music for each game state
- [ ] Sound effects for all actions
- [ ] Volume controls
- [ ] Audio mixing/balance

**Required Assets**:
- 5-6 music tracks
- 20-30 sound effects

### Milestone 5.3: UI/UX Polish (Week 3-4)
**Tasks**:
- [ ] Smooth UI animations
- [ ] Tooltip system
- [ ] Better feedback messages
- [ ] Tutorial/help system
- [ ] Keyboard shortcuts
- [ ] Accessibility features

**Required Assets**:
- Icon refinements
- Tutorial graphics
- Help text

### Milestone 5.4: Save/Load & Settings (Week 4-5)
**Tasks**:
- [ ] Save game system
- [ ] Load game system
- [ ] Auto-save
- [ ] Multiple save slots
- [ ] Settings menu (graphics, audio, controls)

### Milestone 5.5: Balance Pass (Week 5-6)
**Tasks**:
- [ ] Playtest extensively
- [ ] Balance resources
- [ ] Balance rewards
- [ ] Balance difficulty
- [ ] Fix exploits
- [ ] Tune timings

### Phase 5 Success Criteria
- [ ] Game feels polished and complete
- [ ] No major bugs
- [ ] Runs smoothly on target hardware
- [ ] Audio enhances experience
- [ ] UI is intuitive and responsive
- [ ] Balance feels fair

**→ Beta Testing Phase**

---

## Phase 6: Release Preparation 🚀

**Duration**: 2-4 weeks
**Status**: Not Started
**Goal**: Package and release the game

### Milestone 6.1: Documentation (Week 1)
**Tasks**:
- [ ] Player manual/guide
- [ ] Controls reference
- [ ] Strategy tips
- [ ] FAQ
- [ ] Modding guide (if applicable)
- [ ] Credits

### Milestone 6.2: Marketing Assets (Week 1-2)
**Tasks**:
- [ ] Game trailer (1-2 min)
- [ ] Screenshots (10+)
- [ ] Animated GIFs
- [ ] Key art / cover image
- [ ] Store page description
- [ ] Presskit

### Milestone 6.3: Platform Deployment (Week 2-3)
**Tasks**:
- [ ] Build for Windows
- [ ] Build for Linux
- [ ] Build for Mac (if possible)
- [ ] WASM build for web
- [ ] Test on all platforms
- [ ] Package installers

### Milestone 6.4: Store Setup (Week 3)
**Tasks**:
- [ ] Create itch.io page
- [ ] (Optional) Steam store page
- [ ] Set pricing (if not free)
- [ ] Configure downloads
- [ ] Set up analytics

### Milestone 6.5: Launch (Week 4)
**Tasks**:
- [ ] Final QA pass
- [ ] Launch on itch.io
- [ ] Social media announcement
- [ ] Monitor for critical bugs
- [ ] Day-1 patch if needed

### Phase 6 Success Criteria
- [ ] Game is publicly available
- [ ] All platforms working
- [ ] Store page looks professional
- [ ] Documentation is complete
- [ ] No critical launch bugs

**→ LAUNCH! 🎉**

---

## Post-Launch Support

### Immediate (Week 1-2)
- Monitor player feedback
- Fix critical bugs quickly
- Respond to community
- Gather metrics

### Short-term (Month 1-2)
- Balance patches
- QOL improvements
- Minor content additions
- Bug fixes

### Long-term (Month 3+)
- Consider major content updates
- Evaluate modding support expansion
- Plan potential sequel/spinoff
- Maintain community

---

## Estimated Timeline

```
Phase 1 (Planning):         2-4 weeks     ████░░░░░░░░░░░░░░░░
Phase 2 (MVP):              4-6 weeks     ░░░░████████░░░░░░░░
Phase 3 (Core Gameplay):    6-8 weeks     ░░░░░░░░░░░░████████████░░░░
Phase 4 (Content):          4-6 weeks     ░░░░░░░░░░░░░░░░████████░░
Phase 5 (Polish):           4-6 weeks     ░░░░░░░░░░░░░░░░░░░░████████
Phase 6 (Release):          2-4 weeks     ░░░░░░░░░░░░░░░░░░░░░░░░████

Total Estimated Time:       22-34 weeks (5-8 months)
```

**Realistic Timeline**: 6-12 months part-time development

---

## Risk Management

### High Risk Items

**1. Scope Creep**
- *Risk*: Adding features beyond plan
- *Mitigation*: Stick to documented phases, use "ideas.txt" for future
- *Contingency*: Cut Phase 4 features if needed

**2. Art Asset Production**
- *Risk*: Creating 100+ sprites takes time
- *Mitigation*: Use placeholder art, commission if budget allows
- *Contingency*: Simplify art style, reuse sprites

**3. Balance Issues**
- *Risk*: Game too easy/hard
- *Mitigation*: Playtest early and often
- *Contingency*: Adjustable difficulty, extensive tuning knobs

**4. Technical Challenges**
- *Risk*: Bevy issues, WASM problems
- *Mitigation*: Spike technical risks early
- *Contingency*: Simplify technical requirements

**5. Motivation/Burnout**
- *Risk*: Losing steam mid-project
- *Mitigation*: Celebrate milestones, take breaks
- *Contingency*: Release "Early Access" at Phase 3

---

## Success Metrics

### Development Metrics
- [ ] Complete each phase on schedule (±1 week)
- [ ] Maintain consistent commit frequency
- [ ] No phase has >3 critical bugs at completion

### Quality Metrics
- [ ] 60 FPS on target hardware
- [ ] <2 second load times
- [ ] Zero crash bugs in final release
- [ ] 90%+ positive playtest feedback

### Launch Metrics
- Target: 100 downloads in first week
- Target: 50+ itch.io ratings
- Target: 4.0+ star average
- Target: 10+ YouTube playthroughs

---

## Decision Points

### At End of Phase 2
**Question**: Is the core loop fun?
- **Yes** → Continue to Phase 3
- **No** → Iterate on core mechanics
- **Uncertain** → Extensive playtesting

### At End of Phase 3
**Question**: Is there enough content for release?
- **Yes** → Skip Phase 4, go to Phase 5
- **No** → Continue as planned
- **More needed** → Extend Phase 4

### At End of Phase 5
**Question**: Release or more polish?
- **Ready** → Proceed to launch
- **Almost** → 2-week polish extension
- **Not ready** → Re-evaluate scope

---

## Dependencies

### Critical Path
```
Phase 1 → Phase 2 → Phase 3 → Phase 5 → Phase 6
         (MVP)     (Core)     (Polish)  (Release)
```

Phase 4 (Content) can be done in parallel with Phase 5 or skipped.

### Blockers
- Phase 2 blocked by: Asset creation (P0 assets)
- Phase 3 blocked by: Phase 2 completion
- Phase 5 blocked by: Audio asset creation
- Phase 6 blocked by: Complete game

---

## Version Milestones

- **v0.1**: Phase 2 complete (MVP)
- **v0.2**: Phase 3 complete (Core gameplay)
- **v0.3**: Phase 4 complete (Full content)
- **v0.9**: Phase 5 complete (Beta)
- **v1.0**: Phase 6 complete (Release)

---

## Current Action Items

### Immediate (This Week)
1. ✅ Complete Resource System doc
2. ✅ Complete World Map doc
3. ✅ Complete Sprite Specifications
4. ✅ Complete Asset List
5. [ ] Write Horse System doc
6. [ ] Write Encounter System doc
7. [ ] Write Party System doc

### This Month
1. [ ] Complete all mechanics documentation
2. [ ] Create technical architecture doc
3. [ ] Create 10 sample sprites
4. [ ] Create UI mockups
5. [ ] Set up development environment
6. [ ] Begin Phase 2

### This Quarter
1. [ ] Complete Phase 2 (MVP)
2. [ ] Begin Phase 3
3. [ ] First playtest session

---

**Version**: 1.0
**Last Updated**: 2025-10-22
**Status**: LIVING DOCUMENT - Update as project progresses
