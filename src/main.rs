use bevy::prelude::*;

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

    let initial_snake = Snake::new(Position { x: 10, y: 10 });
    let initial_food = Position { x: 5, y: 5 };

    commands.insert_resource(GameState {
        food: initial_food,
        snake: initial_snake.clone(),
        direction: Direction::Right,
        gameover: false,
    });

    init_display(&mut commands, &initial_snake, &initial_food);
}

// TODO: Refactor these two functions
fn new_position(board: &GameBoard) -> Position {
    Position {
        x: rand::random::<i32>() % board.width,
        y: rand::random::<i32>() % board.height
    }
}

fn new_apple(state: &GameState, board: &GameBoard) -> Position {
    let mut pos = new_position(board);
    while state.snake.body.contains(&pos) || pos == state.snake.head {
        pos = new_position(board);
    }
    pos
}

fn step(board: Res<GameBoard>,
        mut state: ResMut<GameState>
) {
    let prev_head = state.snake.head;
    let new_head = prev_head + state.direction;

    // Check for collisions
    if state.snake.body.contains(&new_head) {
        state.gameover = true;
    } // Check for food
    else if new_head == state.food {
        state.snake.extends(new_head);
        state.food = new_apple(&state, &board);
    } // Normal move
    else {
        state.snake.move_to(new_head);
    }
}


#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
enum GameSet {
    CheckInput,
    Step,
    Render
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
                GameSet::Step,
                GameSet::Render
            ).chain()
        )
        .add_systems(Update, direction_input)
        .add_systems(FixedUpdate, (
            check_input.in_set(GameSet::CheckInput),
            step.in_set(GameSet::Step),
            update_visual.in_set(GameSet::Render)
        ))
        // TODO: dynamic speed calculation
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .run();
}
