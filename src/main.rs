use std::io;
mod guessing;
mod sdltest;
mod sokoban;
mod sokoban2;
mod sokonan3;

static OPTIONS: [(&str, fn()); 5] = [
    ("guessing", guessing::guessing),
    ("sokoban", sokoban::start),
    ("sokoban v2", sokoban2::start),
    ("SDL test", sdltest::start),
    ("SDL Sokoban", sokonan3::start),
];

fn main() {
    println!("Choose a game");
    let mut iter = 1;
    for game in &OPTIONS {
        println!("{}. {}", iter, game.0);
        iter += 1
    }
    println!("{}. quit", iter);

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: usize = choice.trim().parse().expect("Not a number");
    let choice = choice - 1;

    if choice < OPTIONS.len() {
        let (name, game) = OPTIONS[choice];
        println!("you chose {}. {}", choice, name);
        game()
    }

    println!("Goodbye")
}
