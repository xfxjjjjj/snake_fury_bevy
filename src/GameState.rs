
#[derive(Resource)]
struct GameState {
    food: Position,
    snake: Snake,
    direction: Direction,
    gameover: bool,
}

#[derive(Resource, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource, Clone)]
struct Snake {
    head: Position,
    body: Vec<Position>,
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

impl Snake {
    fn new(head: Position) -> Self {
        Snake { head, body: Vec::new() }
    }
    fn extends(self: &mut Self, new_head: Position) {
        self.body.insert(0, self.head);
        self.head = new_head;
    }
    fn move_to(self: &mut Self, new_head: Position) {
        self.body.insert(0, self.head);
        self.body.pop();
        self.head = new_head;
    }
}




