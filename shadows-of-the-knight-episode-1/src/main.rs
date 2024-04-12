use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32); // width of the building.
    let h = parse_input!(inputs[1], i32); // height of the building.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32); // maximum number of turns before game over.
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let start_x = parse_input!(inputs[0], i32);
    let start_y = parse_input!(inputs[1], i32);

    let mut current_x = start_x;
    let mut current_y = start_y;

    let mut move_window = (0, w, 0, h);

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string();
        eprintln!("bomb_dir: {}", bomb_dir);

        let optimal_move =
            compute_optimal_move(&bomb_dir, &mut current_x, &mut current_y, &mut move_window);
        eprintln!("move_window: {:?}", move_window);

        println!("{}", optimal_move);
    }
}

fn compute_optimal_move(
    bomb_dir: &String,
    current_x: &mut i32,
    current_y: &mut i32,
    window: &mut (i32, i32, i32, i32),
) -> String {
    for c in bomb_dir.chars() {
        match c {
            'U' => {
                window.3 = *current_y;
                *current_y = (window.2 + window.3) / 2;
            }
            'D' => {
                window.2 = *current_y;
                *current_y = (window.2 + window.3) / 2;
            }
            'L' => {
                window.1 = *current_x;
                *current_x = (window.0 + window.1) / 2;
            }
            'R' => {
                window.0 = *current_x;
                *current_x = (window.0 + window.1) / 2;
            }
            _ => panic!("Invalid bomb direction"),
        }
    }

    format!("{} {}", current_x, current_y)
}
