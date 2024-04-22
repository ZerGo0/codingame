use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    let mut optimal_path: Vec<String> = Vec::new();

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _remaining_turns = parse_input!(input_line, i32);

        if optimal_path.len() == 0 {
            optimal_path = compute_optimal_path(initial_tx, initial_ty, light_x, light_y);
        }

        let next_move = optimal_path.remove(0);
        println!("{}", next_move);
    }
}

fn compute_optimal_path(
    initial_tx: i32,
    initial_ty: i32,
    light_x: i32,
    light_y: i32,
) -> Vec<String> {
    let mut optimal_path: Vec<String> = Vec::new();
    let mut tx = initial_tx;
    let mut ty = initial_ty;

    while tx != light_x || ty != light_y {
        let mut next_move = String::new();
        if ty < light_y {
            next_move.push_str("S");
            ty += 1;
        } else if ty > light_y {
            next_move.push_str("N");
            ty -= 1;
        }

        if tx < light_x {
            next_move.push_str("E");
            tx += 1;
        } else if tx > light_x {
            next_move.push_str("W");
            tx -= 1;
        }

        optimal_path.push(next_move);
    }

    eprintln!("Optimal path: {:?}", optimal_path);

    return optimal_path;
}
