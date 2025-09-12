// Components for visual entities
#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeSegment {
    index: usize, // Which segment this is (0 = first body segment)
}

#[derive(Component)]
struct Food;

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

fn init_display(
    commands: &mut Commands,
    initial_snake: &Snake,
    initial_food: &Position,
) {
    // Spawn snake head
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0), // Green
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position_to_world_coords(initial_snake.head)),
        SnakeHead,
        initial_snake.head, // Position component attached!
    ));

    // Spawn snake body segments
    for (index, &pos) in initial_snake.body.iter().enumerate() {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.0, 0.8, 0.0), // Darker green
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_translation(position_to_world_coords(pos)),
            SnakeSegment { index },
            pos, // Position component attached!
        ));
    }

    // Spawn food
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0), // Red
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position_to_world_coords(*initial_food)),
        Food,
        *initial_food, // Position component attached!
    ));
}

// TODO: Boilerplate removal
// Only change what actually moved
fn update_visual(
    mut commands: Commands,
    state: Res<GameState>,
    mut head_query: Query<(&mut Transform, &mut Position), (With<SnakeHead>, Without<SnakeSegment>, Without<Food>)>,
    mut body_query: Query<(Entity, &mut Transform, &mut Position, &SnakeSegment), (With<SnakeSegment>, Without<SnakeHead>, Without<Food>)>,
    mut food_query: Query<(&mut Transform, &mut Position), (With<Food>, Without<SnakeHead>, Without<SnakeSegment>)>,
) {
    if !state.is_changed() {
        return;
    }

    // Update snake head
    if let Ok((mut head_transform, mut head_pos)) = head_query.single_mut() {
        if *head_pos != state.snake.head {
            *head_pos = state.snake.head;
            head_transform.translation = position_to_world_coords(*head_pos);
        }
    }

    // Get current body segments, sorted by index
    let mut segments: Vec<_> = body_query.iter_mut().collect();
    segments.sort_by_key(|(_, _, _, seg)| seg.index);

    let current_len = segments.len();
    let target_len = state.snake.body.len();

    // Update existing segments
    for (i, &target_pos) in state.snake.body.iter().enumerate() {
        if i < current_len {
            let (_, ref mut transform, ref mut pos, _) = segments[i];
            if **pos != target_pos {
                **pos = target_pos;
                transform.translation = position_to_world_coords(**pos);
            }
        } else {
            // Spawn new segment (snake grew)
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.0, 0.8, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_translation(position_to_world_coords(target_pos)),
                SnakeSegment { index: i },
                target_pos,
            ));
        }
    }

    // Remove excess segments (if snake somehow shrunk)
    for i in target_len..current_len {
        if let Some((entity, _, _, _)) = segments.get(i) {
            commands.entity(*entity).despawn();
        }
    }

    // Update food
    if let Ok((mut food_transform, mut food_pos)) = food_query.single_mut() {
        if *food_pos != state.food {
            *food_pos = state.food;
            food_transform.translation = position_to_world_coords(*food_pos);
        }
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
