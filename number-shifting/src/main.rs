use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    println!("first_level");
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        eprintln!("intial inputs: {:?}", inputs);
        let width = parse_input!(inputs[0], i32);
        let height = parse_input!(inputs[1], i32);

        let mut game_map = vec![vec![0; width as usize]; height as usize];

        for i in 0..height as usize {
            let mut inputs = String::new();
            io::stdin().read_line(&mut inputs).unwrap();
            eprintln!("inputs: {:?}", inputs);
            let cell_values = inputs.split_whitespace().collect::<Vec<_>>();
            for j in 0..width as usize {
                let x = cell_values[j];
                game_map[i][j] = x.parse::<i32>().unwrap();
            }
        }

        eprintln!("game_map: {:?}", game_map);

        let solve_steps = compute_steps(&mut game_map);
        eprintln!("solve_steps: {:?}", solve_steps);
        for step in solve_steps {
            println!("{}", step);
        }

        // println!("7 4 L +");
        // println!("3 0 D -");
        // println!("6 4 L -");
    }
}

/*
You are given a grid, filled with numbers. You can move a number horizontally or vertically by exactly as many cells as the value of the number. The number has to be pushed on another non-zero number. The moved number will then be added to the other number or subtracted. The absolute value will be taken on subtraction. The goal is to clear the board and not have any numbers remaining.
The top left corner has the coordinate (0,0). X increases to the right, y to the bottom.

Then the program has to output the actions to solve the game, each in a new line:
x y dir +/-, the position of the number that shall be moved, the direction (U,D,R,L) and a + to add the numbers or a - to subtract them.
*/
fn compute_steps(game_map: &mut Vec<Vec<i32>>) -> Vec<String> {
    let mut completion_steps = vec![];

    let initial_viable_cells = game_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell != 0)
                .map(move |(x, &cell)| (cell, x, y))
        })
        .collect::<Vec<_>>();

    // We need to compute every possible game state
    for viable_cell in initial_viable_cells.clone() {
        let mut game_map_copy = game_map.clone();
        let (cell_value, x, y) = viable_cell;

        let game_result = get_game_result(&mut game_map_copy, cell_value, x, y);
    }

    return completion_steps;
}

fn get_game_result(game_map: &mut Vec<Vec<i32>>, start_cell: (i32, usize, usize)) -> Vec<String> {
    let mut completion_steps = vec![];

    let initial_viable_cells = game_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell != 0)
                .map(move |(x, &cell)| (cell, x, y))
        })
        .collect::<Vec<_>>();

    let possible_moves = get_possible_moves(start_cell, initial_viable_cells);

    // Recursively try to solve the game
    for (cell_value_1, x1, y1, cell_value_2, x2, y2) in possible_moves {
        let mut game_map_copy = game_map.clone();
        let game_result = get_game_result(&mut game_map_copy, (cell_value_1, x1, y1));

        if game_result.len() > 0 {
            completion_steps.extend(game_result);
        }
    }

    return completion_steps;
}

fn get_possible_moves(
    current_cell: (i32, usize, usize),
    viable_cells: Vec<(i32, usize, usize)>,
) -> Vec<(i32, usize, usize, i32, usize, usize)> {
    let mut possible_moves = vec![];
    let (cell_value_1, x1, y1) = current_cell;

    // find the possible moves
    for (cell_value_2, x2, y2) in viable_cells.clone() {
        // Skip the current cell
        if x1 == x2 && y1 == y2 {
            continue;
        }

        // Make sure that the cell is in reach
        let x_diff = usize::abs_diff(x1, x2);
        let y_diff = usize::abs_diff(y1, y2);
        if x_diff != cell_value_1 as usize && y_diff != cell_value_1 as usize {
            continue;
        }

        // Make sure that the cell is in the same row or column
        if x_diff > 0 && y_diff > 0 {
            continue;
        }

        // Make sure that we don't already have this combination, but in reverse
        let mut already_exists = false;
        for (_, x1_, y1_, _, x2_, y2_) in possible_moves.clone() {
            if x1 == x2_ && y1 == y2_ && x2 == x1_ && y2 == y1_ {
                already_exists = true;
                break;
            }
        }

        if already_exists {
            continue;
        }

        possible_moves.push((cell_value_1, x1, y1, cell_value_2, x2, y2));
    }

    return possible_moves;
}
