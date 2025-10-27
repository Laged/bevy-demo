//! Configuration optimized for performance benchmarking
//!
//! Provides benchmark-tuned settings for stress-testing the game engine

use crate::config_loader::GameConfig;

impl GameConfig {
    /// Returns a config optimized for performance benchmarking
    ///
    /// This config maximizes entity count and load while keeping gameplay logic intact:
    /// - Overpowered player (instant kill, rapid fire)
    /// - No enemy spawn rate limiting
    /// - Maximum possible enemy count
    pub fn benchmark_mode() -> Self {
        let mut config = Self::default();

        // Overpowered player - kills enemies instantly
        config.gun.bullet_damage = 999999.0;
        config.gun.num_bullets_per_shot = 50;
        config.gun.bullet_spawn_interval = 0.0; // Fire every frame
        config.gun.bullet_speed = 30.0;
        config.gun.bullet_time_secs = 2.0;

        // No spawn rate limiting - flood with enemies
        config.enemy.spawn_interval = 0.0;
        config.enemy.max_num_enemies = 100_000;

        config
    }
}
