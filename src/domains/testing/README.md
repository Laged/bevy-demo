# Testing Domain

**Maintainer:** Testing Agent

## Responsibilities
- Headless test harness
- Entity spawning helpers
- Simulation utilities
- Performance benchmarks

## Modules
- `harness.rs` - Headless app creation
- `helpers.rs` - Test entity spawners
- `simulation.rs` - Frame simulation and state helpers
- `benchmarks.rs` - Performance benchmark configurations

## Dependencies
**Reads from:**
- All domains (for testing purposes)

**Writes to:**
- Test entities and state

## Contact Points
This domain is independent and used only during testing.
