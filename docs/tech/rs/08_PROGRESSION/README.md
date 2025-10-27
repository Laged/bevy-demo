# Progression (Tech)

## Data
- Tech tree nodes defined in structured files (`assets/progression/tech_tree.ron`).
- Commander level curve and reward tables.
- Sponsor contracts and audience reputation systems.

## Systems
- Progression manager resource tracking XP, currencies, unlocks.
- Respec and build-sharing utilities.
- Event hooks broadcasting progression milestones to UI and analytics.

## Implementation Notes
- Keep growth modifiers centralized to avoid scattered constants.
- Ensure serialization covers all progression-critical data.
- Provide debug commands for granting XP or unlocking nodes during testing.
