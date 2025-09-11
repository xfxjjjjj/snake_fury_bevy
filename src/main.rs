use bevy::prelude::*;
include!("GameState.rs");
include!("GameBoard.rs");

fn initialize_game(mut commands: Commands) {
    // TODO: enables dynamic board size
    commands.insert_resource(GameBoard::new(20, 20));
    // TODO: randomize initial set up without collisions
    commands.insert_resource(GameState {
        food: Position {
            x: rand::random::<i32>() % 20,
            y: rand::random::<i32>() % 20
        },
        snake: Snake::new(Position { x: 10, y: 10 }),
        direction: Direction::Right,
        gameover: false,
    });
    commands.insert_resource(ButtonInput::<KeyCode>::default());
}

// TODO: bevy UI rendering
fn render(board: Res<GameBoard>,
          state: Res<GameState>) {
    for y in 0..board.height {
        for x in 0..board.width {
            if state.snake.head.x == x && state.snake.head.y == y {
                print!("@ ");
            } else if state.food.x == x && state.food.y == y {
                print!("# ");
            } else if state.snake.body.iter().any(|pos| pos.x == x && pos.y == y) {
                print!("$ ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
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
    if state.snake.body.contains(&new_head) ||
        new_head.x < 0 || new_head.x >= board.width ||
        new_head.y < 0 || new_head.y >= board.height {
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

fn direction_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<ChangeDirection>
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        events.write(ChangeDirection { direction: Direction::Up });
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        events.write(ChangeDirection { direction: Direction::Down });
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        events.write(ChangeDirection { direction: Direction::Left });
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        events.write(ChangeDirection { direction: Direction::Right });
    }
}

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
enum GameSet {
    CheckInput,
    Step,
    Render
}

#[derive(Event)]
struct ChangeDirection {
    direction: Direction
}

fn check_input(
    mut events: EventReader<ChangeDirection>,
    mut state: ResMut<GameState>
) {
    // Only process the latest direction change event
    if let Some(latest_event) = events.read().last() {
        // Only change direction if it's not opposite to current direction
        match (state.direction, latest_event.direction) {
            (Direction::Up, Direction::Down)    |
            (Direction::Down, Direction::Up)    |
            (Direction::Left, Direction::Right) |
            (Direction::Right, Direction::Left) => {}
            _ => {
                state.direction = latest_event.direction;
            }
        }
    }
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
            render.in_set(GameSet::Render)
        ))
        // TODO: dynamic speed calculation
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .run();
}
