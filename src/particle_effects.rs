use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::state::GameState;

pub struct ParticleEffectsPlugin;

impl Plugin for ParticleEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_particle_assets)
            .add_systems(
                Update,
                despawn_finished_impacts.run_if(in_state(GameState::InGame)),
            );
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

#[derive(Component)]
pub struct DeathLingerEffect {
    pub lifetime: Timer,
}

fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    config: Res<crate::config_loader::GameConfig>,
) {
    // Bullet trail effect - continuous emission
    let mut gradient = bevy_hanabi::Gradient::new();
    gradient.add_key(0.0, Vec4::new(
        config.particle_effects.bullet_trail_color_r,
        config.particle_effects.bullet_trail_color_g,
        config.particle_effects.bullet_trail_color_b,
        1.0
    )); // Bright color at spawn
    gradient.add_key(1.0, Vec4::new(
        config.particle_effects.bullet_trail_color_r,
        config.particle_effects.bullet_trail_color_g,
        config.particle_effects.bullet_trail_color_b,
        0.0
    )); // Fade to transparent

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(config.particle_effects.bullet_trail_lifetime).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(10.0).expr(),
    };

    let bullet_trail = effects.add(
        EffectAsset::new(
            32768,
            SpawnerSettings::rate(config.particle_effects.bullet_trail_emission_rate.into()),
            writer.finish()
        )
            .with_name("bullet_trail")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier::new(gradient))
            .render(SizeOverLifetimeModifier {
                gradient: bevy_hanabi::Gradient::constant(Vec3::splat(config.particle_effects.bullet_trail_size)),
                screen_space_size: false,
            }),
    );

    // Impact burst effect - one-shot radial explosion
    let mut impact_gradient = bevy_hanabi::Gradient::new();
    impact_gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.3, 1.0)); // Bright yellow-white
    impact_gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.0, 0.8)); // Orange
    impact_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0)); // Fade to red transparent

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(config.particle_effects.impact_lifetime).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(5.0).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(50.0).expr(),
    };

    let impact_burst = effects.add(
        EffectAsset::new(
            2048,
            SpawnerSettings::once((config.particle_effects.impact_particle_count as f32).into()),
            writer.finish()
        )
            .with_name("impact_burst")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier::new(impact_gradient))
            .render(SizeOverLifetimeModifier {
                gradient: bevy_hanabi::Gradient::constant(Vec3::splat(config.particle_effects.impact_size)),
                screen_space_size: false,
            }),
    );

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_burst,
    });
}

fn despawn_finished_impacts(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ImpactEffect)>,
) {
    for (entity, mut impact) in query.iter_mut() {
        impact.lifetime.tick(time.delta());
        if impact.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
