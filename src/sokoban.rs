use array2d::Array2D;
use std::fs;
use std::io;

const BOX: char = '$';
const GOAL: char = '.';
const WALL: char = '#';
const FLOOR: char = ' ';
const PLAYER: char = '@';
const BOX_GOAL: char = '*';
const PLAYER_GOAL: char = '+';
const CONTROLS: &str = "\nWASD to move\nu to undo\nq to exit";

type Coordinate = (usize, usize);

struct State {
    width: usize,
    height: usize,
    level: usize,
    turns: u32,
    board: Array2D<char>,
    history: Vec<Array2D<char>>,
}

pub fn start() {
    println!("SOKOBAN!\n{}\n", CONTROLS);

    let contents = fs::read_to_string("src/assets/sokoban.txt")
        .expect("Something went wrong reading the file");

    let levels: Vec<&str> = contents.split("\n\n").collect();

    let mut state = State {
        board: Array2D::filled_with(' ', 0, 0),
        level: 0,
        width: 0,
        height: 0,
        turns: 0,
        history: Vec::new(),
    };
    parse_level(levels[state.level], &mut state);

    loop {
        let board = &state.board;
        let mut victory = true;
        let mut player_position = (0, 0);

        let spaces = std::iter::repeat(" ")
            .take(state.width / 2)
            .collect::<String>();
        println!("\n{}[TURN {}]", spaces, state.turns);
        for (i, row) in board.rows_iter().enumerate() {
            for (j, col) in row.enumerate() {
                if *col == BOX {
                    victory = false;
                }
                if *col == PLAYER || *col == PLAYER_GOAL {
                    player_position = (i, j);
                }
                print!("{} ", col);
            }
            print!("\n")
        }

        if victory {
            println!("YOU WIN! press n to go to the next level");
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut target = player_position;
        match input.trim() {
            "w" => target.0 -= 1,
            "a" => target.1 -= 1,
            "s" => target.0 += 1,
            "d" => target.1 += 1,
            "u" => {
                if state.history.len() != 0 {
                    let last = state.history.pop().expect("cant undo");
                    state.board = last;
                    state.turns -= 1;
                }
                continue;
            }
            "n" => {
                if victory {
                    state.level += 1;
                    parse_level(levels[state.level], &mut state);
                    state.history = Vec::new()
                }
                continue;
            }
            "q" => break,
            _ => {
                println!("Invalid input\n{}", CONTROLS);
                continue;
            }
        }

        attempt_move(player_position, target, &mut state)
    }
}

fn attempt_move((px, py): Coordinate, (tx, ty): Coordinate, state: &mut State) {
    let player_char = *state.board.get(px, py).expect("Ooops");
    let target_char = *state.board.get(tx, ty).expect("Ooops");
    if target_char == WALL {
        return;
    }

    // let setBoard = |(x, y): Coordinate, c: char| {
    //     state.board.set(x, y, c).expect("error setting character");
    // };

    let current_state = Array2D::clone(&state.board);

    if target_char == BOX || target_char == BOX_GOAL {
        let (bx, by): Coordinate = (2 * tx - px, 2 * ty - py);
        let second_target_char = *state.board.get(bx, by).expect("oof");

        match second_target_char {
            WALL => return,
            BOX => return,
            BOX_GOAL => return,
            GOAL => state.board.set(bx, by, BOX_GOAL).expect("oops"),
            FLOOR => state.board.set(bx, by, BOX).expect("oops"),
            _ => panic!("Unhandled char {}", second_target_char),
        }

        let box_char = if target_char == BOX { FLOOR } else { GOAL };
        state.board.set(tx, ty, box_char).expect("oops");
    }

    match player_char {
        PLAYER_GOAL => state.board.set(px, py, GOAL).expect("oops"),
        PLAYER => state.board.set(px, py, FLOOR).expect("oops"),
        _ => panic!("AHH"),
    }

    match target_char {
        BOX_GOAL => state.board.set(tx, ty, PLAYER_GOAL).expect("oops"),
        GOAL => state.board.set(tx, ty, PLAYER_GOAL).expect("oops"),
        _ => state.board.set(tx, ty, PLAYER).expect("oops"),
    }

    state.turns += 1;
    state.history.push(current_state);
}

fn parse_level(input: &str, state: &mut State) {
    let rows = String::from(input); //input.to_string();
    let arr: Vec<String> = rows.split("\n").map(|s| s.to_string()).collect();
    let rows: Vec<&str> = rows.split("\n").collect();

    let height = arr.len();
    let mut width = 0;
    for row in rows {
        if row.len() > width {
            width = row.len();
        }
    }

    state.board = Array2D::filled_with(FLOOR, height, width);
    state.width = width;
    state.height = height;

    for i in 0..arr.len() {
        let row = &arr[i];
        for (j, c) in row.chars().enumerate() {
            state.board.set(i, j, c).expect("Dang")
        }
    }
}
