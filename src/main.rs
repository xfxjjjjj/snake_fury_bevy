use bevy::prelude::*;
use rand;

// Constants for rendering
const TILE_SIZE: f32 = 30.0;
const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 20;

include!("game_state.rs");
include!("display.rs");
include!("inputs.rs");

fn initialize_game(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    // Initialize game resources
    commands.insert_resource(GameBoard::new(BOARD_WIDTH, BOARD_HEIGHT));
    commands.insert_resource(GameState::new());

    // Initialize the visual display
    init_display(&mut commands);
}


#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
enum GameSet {
    CheckInput,
    CheckStep,
    Execute
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Set up arbitrary keyboard events
        .add_event::<ChangeDirection>()
        .add_systems(Startup, initialize_game)
        // Configure fixed timestep for game logic
        .configure_sets(
            FixedUpdate,
            (
                GameSet::CheckInput,
                GameSet::CheckStep,
                GameSet::Execute
            ).chain()
        )
        .add_systems(Update, direction_input)
        .add_systems(FixedUpdate, (
            check_input.in_set(GameSet::CheckInput),
            step.in_set(GameSet::CheckStep),
            update_visual.in_set(GameSet::Execute)
        ))
        // TODO: dynamic speed calculation
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .run();
}
