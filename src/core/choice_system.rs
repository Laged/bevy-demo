//! Extensible choice system for gameplay pauses
//!
//! Allows gameplay to pause for player decisions (upgrades, loot, etc.)

use bevy::prelude::*;
use std::collections::HashMap;

/// Context for why the game is paused for a choice
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChoiceContext {
    WaveComplete,   // End of wave upgrades
    LevelUp,        // XP threshold reached
    LootFound,      // Random loot drop
    ShopEncounter,  // Merchant encounter
    BossReward,     // Boss defeated
}

/// A single choice option presented to the player
#[derive(Clone, Debug)]
pub struct ChoiceOption {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub choice_type: ChoiceType,
}

/// Type of choice being offered
#[derive(Clone, Debug)]
pub enum ChoiceType {
    Upgrade { stat: StatType, amount: f32 },
    LootItem { item_id: String },
    Skill { skill_id: String },
}

/// Stats that can be upgraded
#[derive(Clone, Debug)]
pub enum StatType {
    Damage,
    FireRate,
    HealthRegen,
    MoveSpeed,
}

/// Metadata about the choice context
#[derive(Clone, Debug, Default)]
pub struct ChoiceMetadata {
    pub wave_number: Option<u32>,
    pub xp_gained: Option<u32>,
    pub loot_rarity: Option<LootRarity>,
}

#[derive(Clone, Debug)]
pub enum LootRarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

/// Trait for handling different choice contexts
pub trait ChoiceHandler: Send + Sync + 'static {
    /// Generate options for this choice context
    fn generate_options(&self, world: &World) -> Vec<ChoiceOption>;

    /// Apply the selected option
    fn apply_choice(&self, choice: &ChoiceOption, world: &mut World);

    /// Determine next state after applying choice
    fn next_state(&self) -> crate::state::GameState;
}

/// Registry of choice handlers
#[derive(Resource, Default)]
pub struct ChoiceHandlerRegistry {
    handlers: HashMap<ChoiceContext, Box<dyn ChoiceHandler>>,
}

impl ChoiceHandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, context: ChoiceContext, handler: Box<dyn ChoiceHandler>) {
        self.handlers.insert(context, handler);
    }

    pub fn get(&self, context: &ChoiceContext) -> Option<&dyn ChoiceHandler> {
        self.handlers.get(context).map(|b| b.as_ref())
    }
}

/// Resource holding pending choice data
#[derive(Resource, Clone)]
pub struct PendingChoice {
    pub context: ChoiceContext,
    pub options: Vec<ChoiceOption>,
    pub metadata: ChoiceMetadata,
}
