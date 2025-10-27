# Economy (Tech)

## Data
- Currency definitions, reward curves, and drop tables.
- Shop inventories and refresh logic stored in data files.
- Crafting recipes and upgrade costs.

## Systems
- Economy resource tracking income, expenses, and modifiers.
- Encounter payout calculators tied to combat performance metrics.
- Risk mechanics (wagers, sponsorship contracts) and fail-safes.

## Implementation Notes
- Provide deterministic RNG seeds for repeatable economy testing.
- Integrate telemetry to monitor reward pacing.
- Add unit tests for pricing formulas and soft cap enforcement.
