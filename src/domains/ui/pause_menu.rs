//! Pause menu system using Bevy 0.17 headless widgets - Owned by UI Agent
//!
//! Implements pause menu with Button widgets that work in both headless and visual modes

use bevy::prelude::*;
use crate::core::state::GameState;
use crate::core::events::PauseMenuToggled;
use crate::plugin_mode::PluginMode;

pub struct PauseMenuPlugin {
    mode: PluginMode,
}

impl PauseMenuPlugin {
    pub fn new(mode: PluginMode) -> Self {
        Self { mode }
    }
}

impl Default for PauseMenuPlugin {
    fn default() -> Self {
        Self::new(PluginMode::default())
    }
}

#[derive(Component)]
pub struct PauseMenuRoot;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct QuitButton;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // Register pause menu toggle event
        app.add_event::<PauseMenuToggled>();

        // Visual mode: spawn UI elements
        if self.mode == PluginMode::Standard {
            app.add_systems(OnEnter(GameState::MainMenu), setup_pause_menu)
                .add_systems(Update, handle_pause_button_press.run_if(in_state(GameState::MainMenu)));
        }

        // Headless mode: just handle events
        app.add_systems(Update, process_pause_toggle);
    }
}

fn setup_pause_menu(mut commands: Commands) {
    // Spawn pause menu UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            PauseMenuRoot,
        ))
        .with_children(|parent| {
            // Resume button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                    ResumeButton,
                ))
                .with_child((
                    Text::new("Resume"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

            // Quit button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                    QuitButton,
                ))
                .with_child((
                    Text::new("Quit"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}

fn handle_pause_button_press(
    mut next_state: ResMut<NextState<GameState>>,
    resume_button_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    quit_button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    // Check resume button
    for interaction in resume_button_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }

    // Check quit button
    for interaction in quit_button_query.iter() {
        if *interaction == Interaction::Pressed {
            // In real implementation, would exit game
            info!("Quit button pressed");
        }
    }
}

fn process_pause_toggle(
    mut events: EventReader<PauseMenuToggled>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        match current_state.get() {
            GameState::InGame => next_state.set(GameState::MainMenu),
            GameState::MainMenu => next_state.set(GameState::InGame),
            _ => {},
        }
    }
}
