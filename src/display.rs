// Components for visual entities
#[derive(Component, Debug)]
struct SnakeHead {
    position: Position,
}

impl SnakeHead {
    fn new(position: Position) -> Self {
        SnakeHead { position }
    }

    fn update_position(&mut self, new_pos: Position) {
        self.position = new_pos;
    }
}

#[derive(Component)]
struct SnakeSegment {
    position: Position,
    index: usize, // Which segment this is (0 = first body segment)
}

impl SnakeSegment {
    fn new(position: Position, index: usize) -> Self {
        SnakeSegment { position, index }
    }
}

#[derive(Component)]
struct Food {
    position: Position,
}

impl Food {
    fn new(position: Position) -> Self {
        Food { position }
    }
}

#[derive(Resource)]
struct GameBoard {
    width: i32,
    height: i32,
}

impl GameBoard {
    fn new (width: i32, height: i32) -> Self {
        GameBoard { width, height }
    }
}

// Initialize the visual display
fn init_display(commands: &mut Commands) {
    // Spawn initial snake head
    let initial_head_pos = Position { x: 10, y: 10 };
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // Green
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position_to_world_coords(initial_head_pos)),
        SnakeHead::new(initial_head_pos),
    ));


    // Spawn initial food
    let initial_food_pos = Position { x: 5, y: 5 };
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0), // Red
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position_to_world_coords(initial_food_pos)),
        Food::new(initial_food_pos),
    ));
}

fn update_head_visual(
    mut head_query: Query<(&mut Transform, &SnakeHead), Changed<SnakeHead>>,
) {
    // Update snake head visual position when component position changes
    for (mut transform, head) in head_query.iter_mut() {
        transform.translation = position_to_world_coords(head.position);
        println!("Head moved to: {:?}", head.position);
    }
}

fn update_segment_visual(
    mut segment_query: Query<(&mut Transform, &SnakeSegment), Changed<SnakeSegment>>,
) {
    // Update snake segment visual positions when component positions change
    for (mut transform, segment) in segment_query.iter_mut() {
        transform.translation = position_to_world_coords(segment.position);
    }
}

fn update_food_visual(
    mut food_query: Query<(&mut Transform, &Food), Changed<Food>>,
) {
    // Update food visual position when component position changes
    for (mut transform, food) in food_query.iter_mut() {
        transform.translation = position_to_world_coords(food.position);
    }
}

// Helper function to convert Position to world coordinates
fn position_to_world_coords(pos: Position) -> Vec3 {
    Vec3::new(
        (pos.x as f32 - BOARD_WIDTH as f32 / 2.0) * TILE_SIZE,
        (pos.y as f32 - BOARD_HEIGHT as f32 / 2.0) * TILE_SIZE,
        1.0,
    )
}
