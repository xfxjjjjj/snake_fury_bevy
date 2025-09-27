use std::collections::VecDeque;

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
    segment_queue: VecDeque<Position>,
}

#[derive(Resource, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Resource, Clone, Copy, Eq, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {

    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn to_world_coords(&self) -> Vec3 {
        Vec3::new(
            (self.x as f32 - BOARD_WIDTH as f32 / 2.0) * TILE_SIZE,
            (self.y as f32 - BOARD_HEIGHT as f32 / 2.0) * TILE_SIZE,
            1.0,
        )
    }

    fn move_towards(self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position::new(self.x, self.y + 1),
            Direction::Down => Position::new(self.x, self.y - 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

impl GameState {
    fn new() -> Self {
        GameState {
            score: 0,
            direction: Direction::Right,
            action: Action::Move,
            segment_queue: VecDeque::new(),
        }
    }
}

fn random_pos(board: &GameBoard) -> Position {
    Position::new(
        (rand::random::<i32>() % board.width).abs(),
        (rand::random::<i32>() % board.height).abs()
    )
}

// Helper function to get new apple position, avoiding collisions
fn get_new_apple_position(
    board: &GameBoard,
    head_pos: Position,
    segments: Vec<Position>
) -> Position {
    let mut pos = random_pos(board);

    while head_pos == pos || segments.contains(&pos) {
        pos = random_pos(board);
    }
    pos
}

fn step(
    mut head_query: Query<&mut SnakeHead>,
    mut segment_query: Query<&mut SnakeSegment>,
    mut food_query: Query<&mut Food>,
    mut state: ResMut<GameState>,
    board: Res<GameBoard>
) {
    if let Ok(mut head) = head_query.single_mut() {
        let new_head_pos = head.position.move_towards(state.direction);

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

        let tail_position = state.segment_queue
                                           .pop_back()
                                           .unwrap_or(head.position);
        if !segment_query.is_empty() {
            let mut tail_query = segment_query.iter_mut()
                .find(|s| s.position == tail_position)
                .unwrap();

            tail_query.update_position(head.position);
            state.segment_queue.push_front(head.position);
        }

        // Check for food collision
        if let Ok(mut food) = food_query.single_mut() &&
           new_head_pos == food.position {
                // Eat food and grow
                state.action = Action::Grow;
                state.score += 1;

                // Generate new food position
                let segments: Vec<Position> = segment_query.iter()
                    .map(|s| s.position).collect();
                food.position =
                    get_new_apple_position(&board, head.position, segments);

                state.segment_queue.push_back(tail_position);
        } else {
            state.action = Action::Move;
        }

        head.update_position(new_head_pos);
    }
}

fn clean_up<T: Component>(
    mut commands: Commands,
    head_query: Query<Entity, With<T>>
) {
    for entity in head_query.iter() {
        commands.entity(entity).despawn();
    }
}


fn snake_growing(state: Res<GameState>) -> bool {
    state.action == Action::Grow
}
