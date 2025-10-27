# Latest-RS-Bevy-Features Skill Test Checklist

## Quick Start After Restart

After `claude --continue`, use this checklist to verify the skill works:

---

## Test 1: Skill Activates on Bevy Feature Question

**Command:** Ask Claude directly:
```
"I need to add a pause menu to my Bevy game. Should I use the new headless
widgets or find an alternative? What's production-ready?"
```

**Expected:** Skill activates automatically, provides:
- ❌ Headless widgets marked experimental
- ✅ Recommendation: use bevy_egui or custom UI instead
- Status from stability matrix

**Pass Criteria:** Skill mentioned by name, stability matrix cited

---

## Test 2: ViewportNode Deployment Confidence

**Command:** Ask Claude:
```
"I want to implement a minimap using ViewportNode. What's the FPS impact
if I later add a wave preview panel (2 ViewportNodes total)?"
```

**Expected:** Skill provides:
- Performance reference: 2-3 ViewportNodes = -5-10 FPS
- Code example for ViewportNode setup
- Known gotcha about TextureUsages::RENDER_ATTACHMENT

**Pass Criteria:** Exact FPS numbers cited from skill, code example shown

---

## Test 3: TilemapChunk WASM Decision

**Command:** Ask Claude:
```
"I'm considering TilemapChunk for my arena background, but I might deploy
to WASM later. What should I know?"
```

**Expected:** Skill provides:
- WASM limitation: not supported in 0.17.x
- 3 workarounds (bevy_ecs_tilemap, sprite grid, native only)
- Code example for sprite grid WASM alternative
- Performance notes for each approach

**Pass Criteria:** All 3 workarounds mentioned, code example provided

---

## Test 4: Third-Party Crate Version Confidence

**Command:** Ask Claude:
```
"I decided to use bevy_ecs_tilemap instead of built-in TilemapChunk.
What version should I add to Cargo.toml for Bevy 0.17.2?"
```

**Expected:** Skill provides:
- Verified version: 0.17.2+
- Verification workflow (cargo check, cargo tree bevy)
- Fallback if version mismatch occurs

**Pass Criteria:** Exact version number cited, verification commands provided

---

## Test 5: Troubleshooting Integration

**Command:** Ask Claude:
```
"I implemented ViewportNode but it shows a black screen. What's the first
thing I should check?"
```

**Expected:** Skill provides troubleshooting:
- Root cause: TextureUsages::RENDER_ATTACHMENT missing
- Solution: verify texture creation flags
- Reference to setup code pattern in skill

**Pass Criteria:** Root cause identified, not generic debugging advice

---

## Test 6: Integration with Project Docs

**Command:** Ask Claude:
```
"How does the new latest-rs-bevy-features skill relate to the
docs/bevy-features-0.17.md documentation in my project?"
```

**Expected:** Skill/docs relationship explained:
- Skill: Quick decisions (2 min)
- Docs: Deep dives (detailed examples, code patterns)
- Integration checklist in docs matches your roadmap

**Pass Criteria:** Both resources mentioned, clear use cases for each

---

## Passing Score

**All tests pass if:**
- ✅ Skill activates automatically (not manual invoke needed)
- ✅ Stability matrix cited with correct status (experimental/stable/foundational)
- ✅ Exact FPS numbers provided (-5-10 FPS for 3 ViewportNodes)
- ✅ WASM workarounds explained with code examples
- ✅ Verified crate versions in version table
- ✅ Troubleshooting points to root cause, not generic steps

---

## If Skill Doesn't Activate

**Possible issues:**
1. **Restart needed** → Run: `claude --continue`
2. **Skill file syntax error** → Check: `~/.claude/skills/latest-rs-bevy-features/SKILL.md`
3. **Keywords not matching** → Skill description includes: "Bevy 0.17", "production readiness", "headless widgets", "viewport node"

**Verify skill exists:**
```bash
ls -la ~/.claude/skills/latest-rs-bevy-features/
# Should show: SKILL.md
```

---

## Expected Output Format

When skill activates, you should see in Claude's response:
```
I'm using the latest-rs-bevy-features skill to help you decide...
```

OR skill reference in the context, e.g.:
```
According to the latest-rs-bevy-features stability matrix...
```

---

## Success Criteria Summary

| Test | Passing | Notes |
|------|---------|-------|
| Test 1: Activation | ✅ When asking about Bevy features | Auto-trigger on feature questions |
| Test 2: ViewportNode FPS | ✅ -5-10 FPS for 2-3 nodes | Exact numbers from skill |
| Test 3: WASM alternatives | ✅ 3 workarounds + code | Not generic advice |
| Test 4: Version table | ✅ 0.17.2+ with workflow | Verified versions only |
| Test 5: Troubleshooting | ✅ Root cause first | TextureUsages::RENDER_ATTACHMENT |
| Test 6: Docs integration | ✅ Skill + docs explained | Clear use cases for each |

**Overall:** Skill is production-ready when all 6 tests pass

---

## Time Investment

- **Before skill** (RED baseline): 75-110 minutes to research
- **With skill** (GREEN+REFACTOR): 5-10 minutes + high confidence
- **Expected from these tests**: Confirm 12-18x speedup holds true

---

## Next Steps After Testing

If skill works:
1. ✅ Proceed with headless testing refactor (in progress)
2. ✅ Use skill for ViewportNode minimap implementation
3. ✅ Reference docs for code examples and testing patterns
4. ✅ Use skill decision matrix for bevy_ecs_tilemap vs TilemapChunk

If skill needs fixes:
1. Check SKILL.md for syntax errors
2. Verify YAML frontmatter (name and description fields)
3. Check skill description has enough keywords
4. Restart Claude and re-test
