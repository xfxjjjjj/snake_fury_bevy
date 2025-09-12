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
