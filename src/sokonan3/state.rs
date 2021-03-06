use array2d::Array2D;
use std::fs;

const BOX: char = '$';
const GOAL: char = '.';
const WALL: char = '#';
const FLOOR: char = ' ';
const PLAYER: char = '@';
const BOX_GOAL: char = '*';
const PLAYER_GOAL: char = '+';

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub type Coordinate = (usize, usize);

pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub level: usize,
    pub turns: u32,
    pub levels: Vec<String>,
    pub history: Vec<Array2D<char>>,
    pub victory: bool,
}

impl GameState {
    pub fn new(path: &str) -> GameState {
        let contents = fs::read_to_string(path).unwrap();
        let levels: Vec<String> = contents.split("\n\n").map(|s| s.to_string()).collect();

        let mut state = GameState {
            levels,
            level: 0,
            width: 0,
            height: 0,
            turns: 0,
            history: Vec::new(),
            victory: false,
        };
        state.load();

        state
    }

    fn load(&mut self) {
        let rows = &self.levels[self.level];
        let arr: Vec<String> = rows.split("\n").map(|s| s.to_string()).collect();

        self.height = arr.len();
        self.width = 0;
        for i in 0..self.height {
            let length = arr[i].len();
            if length > self.width {
                self.width = length;
            }
        }

        let mut board = Array2D::filled_with(FLOOR, self.height, self.width);
        for (i, row) in arr.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                board.set(i, j, c).expect("Error setting up board")
            }
        }
        self.history = Vec::new();
        self.history.push(board);
        self.validate();
    }

    pub fn next(&mut self) {
        self.level += 1;
        self.load();
    }

    pub fn undo(&mut self) {
        if self.history.len() > 1 {
            self.history.pop().expect("cant undo");
            self.turns -= 1;
        }
        self.validate();
    }

    fn validate(&mut self) {
        let mut victory = true;
        for row in self.history.last().unwrap().rows_iter() {
            for col in row {
                if *col == BOX {
                    victory = false;
                }
            }
        }
        self.victory = victory;
    }

    pub fn attempt_move(&mut self, dir: Direction) {
        let mut board = self.history.last().expect("Oof").clone();
        let mut player_position: Coordinate = (0, 0);

        for (i, row) in board.rows_iter().enumerate() {
            for (j, col) in row.enumerate() {
                if *col == PLAYER || *col == PLAYER_GOAL {
                    player_position = (i, j);
                }
            }
        }
        let mut target: Coordinate = player_position.clone();
        match dir {
            Direction::Up => target.0 -= 1,
            Direction::Down => target.0 += 1,
            Direction::Left => target.1 -= 1,
            Direction::Right => target.1 += 1,
        };

        let player_char = *board
            .get(player_position.0, player_position.1)
            .expect("Error getting player character");
        let target_char = *board
            .get(target.0, target.1)
            .expect("Error getting target character");

        let target_char_update = match target_char {
            WALL => return,
            BOX | BOX_GOAL => {
                let (bx, by): Coordinate = (
                    2 * target.0 - player_position.0,
                    2 * target.1 - player_position.1,
                );
                let second_target_char =
                    *board.get(bx, by).expect("Error getting additional target");

                let box_char = match second_target_char {
                    WALL | BOX | BOX_GOAL => return,
                    GOAL => {
                        board.set(bx, by, BOX_GOAL).expect("Error setting Box");
                        BOX_GOAL
                    }
                    FLOOR => {
                        board.set(bx, by, BOX).expect("Error setting Box");
                        BOX
                    }
                    _ => panic!("Unhandled char {}", second_target_char),
                };

                board
                    .set(bx, by, box_char)
                    .expect("Error setting floor after box move");
                if target_char == BOX {
                    PLAYER
                } else {
                    PLAYER_GOAL
                }
            }
            FLOOR => PLAYER,
            GOAL => PLAYER_GOAL,
            _ => panic!("Unhandled character {}", target_char),
        };

        board
            .set(target.0, target.1, target_char_update)
            .expect("Error setting new target ");

        board
            .set(
                player_position.0,
                player_position.1,
                match player_char {
                    PLAYER_GOAL => GOAL,
                    PLAYER => FLOOR,
                    _ => panic!("AHH"),
                },
            )
            .expect("Error resetting player's prev position");

        self.turns += 1;
        self.history.push(board);
        self.validate();
    }
}
