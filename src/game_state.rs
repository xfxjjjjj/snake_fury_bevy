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
            segment_queue: VecDeque::new(),
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
    segments: Vec<Position>
) -> Position {
    let mut pos = new_position(board);

    while head_pos == pos || segments.contains(&pos) {
        pos = new_position(board);
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

fn insert_new_segment(
    mut commands: Commands,
    state: Res<GameState>,
) {
    let new_segment_pos = state.segment_queue
        .back()
        .cloned()
        .unwrap_or(Position { x: 0, y: 0 });

    // TODO: Refactor these into a bundle
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.5, 0.0), // Dark Green
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position_to_world_coords(new_segment_pos)),
        SnakeSegment::new(new_segment_pos),
    ));
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
