# UI & UX

## Surfaces
- **Main Menu**: Single centered button labeled `Play`. One click transitions to gameplay.
- **In-Game HUD**: Debug panel in the top-left corner showing FPS, live enemy count, and player health.
- **Cursor**: System cursor doubles as reticle; no custom crosshair.

## Interaction Highlights
- Camera uses smooth follow (10% lerp each frame) to keep the player centered without abrupt jumps.
- Zoom and panning are disabled for players to maintain clarity.
- Mouse aiming drives both gun rotation and player sprite facing direction for readability.

## Feedback
- Weapon discharges are purely visual (no muzzle flash or audio).
- Enemy hits provide no hit-stop or screen shake; damage feedback is only via health readout.
- Death feedback is abrupt: the player sprite despawns and the state jumps to the main menu without messaging.

## UX Gaps
- No tutorial or controls overlay.
- Lack of audio, particle effects, or health UI bar weakens moment-to-moment clarity.
- No pause menu or quit flow.
