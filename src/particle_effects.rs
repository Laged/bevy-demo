use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::state::GameState;

pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_particle_assets);
    }
}

#[derive(Resource)]
pub struct ParticleEffectAssets {
    pub bullet_trail: Handle<EffectAsset>,
    pub impact_burst: Handle<EffectAsset>,
}

#[derive(Component)]
pub struct BulletTrailEmitter;

#[derive(Component)]
pub struct ImpactEffect {
    pub lifetime: Timer,
}

fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Placeholder - we'll implement effect creation in next task
    let bullet_trail = effects.add(EffectAsset::default());
    let impact_burst = effects.add(EffectAsset::default());

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}
