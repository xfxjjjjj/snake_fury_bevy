#[derive(Component, Resource)]
struct GameBoard {
    width: i32,
    height: i32,
}

impl GameBoard {
    fn new (width: i32, height: i32) -> Self {
        GameBoard { width, height }
    }
}
