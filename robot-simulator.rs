#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            _ => 3, // West
        }
    }
}
impl From<i32> for Direction {
    fn from(dir: i32) -> Self {
        match dir {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

impl Direction {
    fn rotate_left(self) -> Self {
        let dir: i32 = self.into();
        Direction::from((dir + 3) % 4)
    }

    fn rotate_right(self) -> Self {
        let dir: i32 = self.into();
        Direction::from((dir + 1) % 4)
    }

    fn get_cartesian(self) -> (i32, i32) {
        let dir: i32 = self.into();
        let (x, y) = (dir & 1, !dir & 1);
        if dir > 1 {
            (-x, -y)
        } else {
            (x, y)
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x: x, y: y, d: d }
    }

    pub fn turn_right(self) -> Self {
        let mut this = self;
        this.d = this.d.rotate_right();
        this
    }

    pub fn turn_left(self) -> Self {
        let mut this = self;
        this.d = this.d.rotate_left();
        this
    }

    pub fn advance(self) -> Self {
        let mut this = self;
        let (x, y) = this.d.get_cartesian();
        this.x += x;
        this.y += y;
        this
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |this: Self, command| match command {
                'L' => this.turn_left(),
                'R' => this.turn_right(),
                'A' => this.advance(),
                _ => this,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
