trait HasPosition {
    fn new(position: Position) -> Self;
    fn update_position(&mut self, new_pos: Position);
}

trait HasColor {
    fn default_color() -> Color;
}
// Components for visual entities
#[derive(Component, Debug)]
struct SnakeHead {
    position: Position,
}

impl HasPosition for SnakeHead {
    fn new(position: Position) -> Self {
        SnakeHead { position }
    }

    fn update_position(&mut self, new_pos: Position) {
        self.position = new_pos;
    }
}

impl HasColor for SnakeHead {
    fn default_color() -> Color {
        Color::srgb(0.0, 1.0, 0.0) // Green color for snake head
    }
}

#[derive(Component)]
struct SnakeSegment {
    position: Position
}

impl HasPosition for SnakeSegment {
    fn new(position: Position) -> Self {
        SnakeSegment { position }
    }
    fn update_position(&mut self, new_pos: Position) {
        self.position = new_pos;
    }
}

impl HasColor for SnakeSegment {
    fn default_color() -> Color {
        Color::srgb(0.0, 0.5, 0.0) // Darker green for snake segments
    }
}

#[derive(Component)]
struct Food {
    position: Position,
}

impl HasPosition for Food {
    fn new(position: Position) -> Self {
        Food { position }
    }
    fn update_position(&mut self, new_pos: Position) {
        self.position = new_pos;
    }
}

impl HasColor for Food {
    fn default_color() -> Color {
        Color::srgb(1.0, 0.0, 0.0) // Red color for food
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

// Marker to identify score text entity
#[derive(Component)]
struct ScoreText;

fn init_score(commands: &mut Commands) {
    commands.spawn(Text::new("Score: "))
            .with_child(
                (TextSpan::default(),
                ScoreText)
            );
}

#[derive(Bundle)]
struct VisualComponent<T: Component + HasPosition + HasColor> {
    sprite: Sprite,
    transform: Transform,
    component: T,
}

impl<T: Component + HasPosition + HasColor> VisualComponent<T> {
    fn new(position: Position) -> Self {
        let color = T::default_color();
        VisualComponent {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_translation(position.to_world_coords()),
            component: T::new(position),
        }
    }
}

// Initialize the visual display
fn init_display(commands: &mut Commands) {
    // Spawn initial snake head
    let initial_head_pos = Position { x: 10, y: 10 };
    commands.spawn(VisualComponent::<SnakeHead>::new(initial_head_pos));

    // Spawn initial food
    let initial_food_pos = Position { x: 5, y: 5 };
    commands.spawn(VisualComponent::<Food>::new(initial_food_pos));
}

fn update_head_visual(
    mut head_query: Query<(&mut Transform, &SnakeHead), Changed<SnakeHead>>,
) {
    // Update snake head visual position when component position changes
    for (mut transform, head) in head_query.iter_mut() {
        transform.translation = head.position.to_world_coords();
    }
}

fn update_segment_visual(
    mut segment_query: Query<(&mut Transform, &SnakeSegment), Changed<SnakeSegment>>,
) {
    // Update snake segment visual positions when component positions change
    for (mut transform, segment) in segment_query.iter_mut() {
        transform.translation = segment.position.to_world_coords();
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

    commands.spawn(VisualComponent::<SnakeSegment>::new(new_segment_pos));
}

fn update_food_visual(
    mut food_query: Query<(&mut Transform, &Food), Changed<Food>>,
) {
    // Update food visual position when component position changes
    for (mut transform, food) in food_query.iter_mut() {
        transform.translation = food.position.to_world_coords();
    }
}

fn update_score(
    state: Res<GameState>,
    mut query: Query<&mut TextSpan, With<ScoreText>>,
) {
    for mut span in &mut query {
        **span = format!("Score: {}", state.score);
    }
}
