#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Action {
    Move,
    Grow,
    GameOver,
}
#[derive(Resource)]
struct GameState {
    score: u32,
    direction: Direction,
    action: Action,
}

#[derive(Resource, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Resource, Clone, Copy, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position { x: self.x, y: self.y + 1 },
            Direction::Down => Position { x: self.x, y: self.y - 1 },
            Direction::Left => Position { x: self.x - 1, y: self.y },
            Direction::Right => Position { x: self.x + 1, y: self.y },
        }
    }
}

impl GameState {
    fn new() -> Self {
        GameState {
            score: 0,
            direction: Direction::Right,
            action: Action::Move,
        }
    }
}

fn new_position(board: &GameBoard) -> Position {
    Position {
        x: (rand::random::<i32>() % board.width).abs(),
        y: (rand::random::<i32>() % board.height).abs()
    }
}

// Helper function to get new apple position with correct query types
fn get_new_apple_position(
    board: &GameBoard,
    head_pos: Position,
    segments: &[&SnakeSegment]
) -> Position {
    let mut pos = new_position(board);

    loop {
        let mut valid = true;

        // Check against head
        if head_pos == pos {
            valid = false;
        }

        // Check against body segments
        if valid {
            for segment in segments {
                if segment.position == pos {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            break;
        }
        pos = new_position(board);
    }
    pos
}

fn step(
    mut head_query: Query<&mut SnakeHead>,
    mut segment_query: Query<&mut SnakeSegment>,
    mut food_query: Query<&mut Food>,
    mut state: ResMut<GameState>,
    board: Res<GameBoard>,
    mut commands: Commands
) {
    if let Ok(mut head) = head_query.single_mut() {
        let new_head_pos = head.position + state.direction;

        // Check for wall collisions
        if new_head_pos.x < 0 || new_head_pos.x >= board.width ||
           new_head_pos.y < 0 || new_head_pos.y >= board.height {
            state.action = Action::GameOver;
            return;
        }

        // Check for self-collisions
        for segment in segment_query.iter() {
            if segment.position == new_head_pos {
                state.action = Action::GameOver;
                return;
            }
        }

        // Check for food collision
        if let Ok(mut food) = food_query.single_mut() {
            if new_head_pos == food.position {
                // Eat food and grow
                state.action = Action::Grow;
                state.score += 1;

                // Move head to food position
                head.update_position(new_head_pos);

                // Add new body segment at old head position
                let new_segment_index = segment_query.iter().count();
                commands.spawn((
                    Sprite {
                        color: Color::srgb(0.0, 0.8, 0.0),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_translation(position_to_world_coords(head.position)),
                    SnakeSegment::new(head.position, new_segment_index),
                ));

                // Generate new food position
                let segments: Vec<&SnakeSegment> = segment_query.iter().collect();
                food.position = get_new_apple_position(&board, head.position, &segments);
            } else {
                // Just move - shift all segments
                state.action = Action::Move;

                // Collect all segments sorted by index
                let mut segments: Vec<_> = segment_query.iter_mut().collect();
                segments.sort_by_key(|segment| segment.index);

                // Move each segment to the position of the one in front of it
                let mut prev_pos = head.position;
                head.update_position(new_head_pos);

                for segment in segments.iter_mut() {
                    let current_pos = segment.position;
                    segment.position = prev_pos;
                    prev_pos = current_pos;
                }
            }
        }
    }
}
