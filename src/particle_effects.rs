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
    // Colored hit impact variants (palette_color, effect_handle)
    pub impact_variants: Vec<(Color, Handle<EffectAsset>)>,
    // Colored death burst variants (palette_color, effect_handle)
    pub death_burst_variants: Vec<(Color, Handle<EffectAsset>)>,
    // Shared lingering death effect (white/gray)
    pub death_linger: Handle<EffectAsset>,
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

/// Creates a gradient that transitions: base_color (bright) → white (flash) → base_color (transparent)
fn create_color_gradient(base_color: Color) -> bevy_hanabi::Gradient<Vec4> {
    let mut gradient = bevy_hanabi::Gradient::new();

    // Convert color to RGBA f32 values
    let rgba = base_color.to_linear().to_vec4();

    // Start: Base color at full brightness
    gradient.add_key(0.0, Vec4::new(
        rgba.x,
        rgba.y,
        rgba.z,
        1.0
    ));

    // Middle: White flash for pop effect
    gradient.add_key(0.5, Vec4::new(1.0, 1.0, 1.0, 1.0));

    // End: Base color fading to transparent
    gradient.add_key(1.0, Vec4::new(
        rgba.x,
        rgba.y,
        rgba.z,
        0.0
    ));

    gradient
}

/// Finds the closest color variant from the palette using RGB distance
pub fn find_closest_effect_variant(
    enemy_color: Color,
    variants: &Vec<(Color, Handle<EffectAsset>)>,
) -> Handle<EffectAsset> {
    variants
        .iter()
        .min_by_key(|(palette_color, _)| {
            // Simple RGB distance (Euclidean in RGB space)
            let pal_lin = palette_color.to_linear();
            let enemy_lin = enemy_color.to_linear();
            let dr = (pal_lin.red - enemy_lin.red).abs();
            let dg = (pal_lin.green - enemy_lin.green).abs();
            let db = (pal_lin.blue - enemy_lin.blue).abs();
            // Scale by 1000 for integer comparison precision
            ((dr + dg + db) * 1000.0) as u32
        })
        .map(|(_, handle)| handle.clone())
        .expect("Variant palette should not be empty")
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

    // Define 8-color palette evenly spaced on color wheel
    let palette = vec![
        ("Red", Color::srgb(1.0, 0.0, 0.0)),
        ("Orange", Color::srgb(1.0, 0.5, 0.0)),
        ("Yellow", Color::srgb(1.0, 1.0, 0.0)),
        ("Green", Color::srgb(0.0, 1.0, 0.0)),
        ("Cyan", Color::srgb(0.0, 1.0, 1.0)),
        ("Blue", Color::srgb(0.0, 0.0, 1.0)),
        ("Purple", Color::srgb(0.5, 0.0, 1.0)),
        ("Magenta", Color::srgb(1.0, 0.0, 1.0)),
    ];

    // Generate hit impact variants (30 particles, moderate velocity)
    let mut impact_variants = Vec::new();
    for (_name, base_color) in palette.iter() {
        let gradient = create_color_gradient(*base_color);

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
            speed: writer.lit(config.particle_effects.impact_velocity).expr(),
        };

        let effect = effects.add(
            EffectAsset::new(
                2048,
                SpawnerSettings::once((config.particle_effects.impact_particle_count as f32).into()),
                writer.finish()
            )
                .with_name(&format!("impact_{}", _name))
                .init(init_pos)
                .init(init_vel)
                .init(init_age)
                .init(init_lifetime)
                .render(ColorOverLifetimeModifier::new(gradient))
                .render(SizeOverLifetimeModifier {
                    gradient: bevy_hanabi::Gradient::constant(Vec3::splat(config.particle_effects.impact_size)),
                    screen_space_size: false,
                }),
        );

        impact_variants.push((*base_color, effect));
    }

    // Generate death burst variants (60 particles, high velocity)
    let mut death_burst_variants = Vec::new();
    for (_name, base_color) in palette.iter() {
        let gradient = create_color_gradient(*base_color);

        let writer = ExprWriter::new();
        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);
        let lifetime = writer.lit(config.particle_effects.death_burst_lifetime).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(5.0).expr(),
            dimension: ShapeDimension::Surface,
        };

        let init_vel = SetVelocitySphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            speed: writer.lit(config.particle_effects.death_burst_velocity).expr(),
        };

        let effect = effects.add(
            EffectAsset::new(
                4096,
                SpawnerSettings::once((config.particle_effects.death_burst_particle_count as f32).into()),
                writer.finish()
            )
                .with_name(&format!("death_burst_{}", _name))
                .init(init_pos)
                .init(init_vel)
                .init(init_age)
                .init(init_lifetime)
                .render(ColorOverLifetimeModifier::new(gradient))
                .render(SizeOverLifetimeModifier {
                    gradient: bevy_hanabi::Gradient::constant(Vec3::splat(config.particle_effects.death_burst_size)),
                    screen_space_size: false,
                }),
        );

        death_burst_variants.push((*base_color, effect));
    }

    // Create shared lingering death effect (white/gray, slow upward drift)
    let mut linger_gradient = bevy_hanabi::Gradient::new();
    linger_gradient.add_key(0.0, Vec4::new(0.9, 0.9, 0.9, 0.8)); // Light gray
    linger_gradient.add_key(1.0, Vec4::new(0.5, 0.5, 0.5, 0.0)); // Dark gray transparent

    let writer = ExprWriter::new();
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(config.particle_effects.death_linger_lifetime).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::new(0.0, config.particle_effects.death_linger_velocity, 0.0)).expr(),
        speed: writer.lit(config.particle_effects.death_linger_velocity * 0.5).expr(),
    };

    let death_linger = effects.add(
        EffectAsset::new(
            2048,
            SpawnerSettings::once((config.particle_effects.death_linger_particle_count as f32).into()),
            writer.finish()
        )
            .with_name("death_linger")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .render(ColorOverLifetimeModifier::new(linger_gradient))
            .render(SizeOverLifetimeModifier {
                gradient: bevy_hanabi::Gradient::constant(Vec3::splat(config.particle_effects.death_linger_size)),
                screen_space_size: false,
            }),
    );

    commands.insert_resource(ParticleEffectAssets {
        bullet_trail,
        impact_variants,
        death_burst_variants,
        death_linger,
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
