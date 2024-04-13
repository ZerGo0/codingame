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
    for _ in initial_viable_cells.clone() {
        let mut game_map_copy = game_map.clone();

        let game_result = get_game_result(&mut game_map_copy);
        if game_result.0 {
            return game_result.1;
        }
    }

    panic!("No solution found")
}

fn get_game_result(game_map: &mut Vec<Vec<i32>>) -> (bool, Vec<String>) {
    let mut completion_steps = vec![];

    let viable_cells = game_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell != 0)
                .map(move |(x, &cell)| (cell, x, y))
        })
        .collect::<Vec<_>>();

    if viable_cells.len() == 1 {
        // eprintln!("[get_game_result] only one viable cell left");
        return (false, completion_steps);
    }

    let possible_moves = get_possible_moves(viable_cells);

    if possible_moves.len() == 0 {
        // eprintln!("[get_game_result] no possible moves left");
        return (false, completion_steps);
    }

    // Recursively try to solve the game
    for (cell_value_1, x1, y1, cell_value_2, x2, y2) in possible_moves {
        let move_direction = if x1 == x2 {
            if y1 > y2 {
                "U"
            } else {
                "D"
            }
        } else {
            if x1 > x2 {
                "L"
            } else {
                "R"
            }
        };

        // We need to simulate + and - moves
        for move_sign in vec!["+", "-"] {
            let mut game_map_copy = game_map.clone();
            completion_steps = vec![];
            completion_steps.push(format!("{} {} {} {}", x1, y1, move_direction, move_sign));

            // Perform the move
            let new_cell_value = if move_sign == "+" {
                cell_value_2 + cell_value_1
            } else {
                i32::abs(cell_value_2 - cell_value_1)
            };
            game_map_copy[y2][x2] = new_cell_value;
            game_map_copy[y1][x1] = 0;

            let game_result = get_game_result(&mut game_map_copy);

            if game_result.1.len() > 0 {
                completion_steps.extend(game_result.1);
            }

            if game_result.0 {
                return (true, completion_steps);
            }

            // Check if the game_map is 0
            let is_game_map_empty = game_map_copy
                .iter()
                .all(|row| row.iter().all(|&cell| cell == 0));

            if is_game_map_empty {
                return (true, completion_steps);
            }
        }
    }

    return (false, completion_steps);
}

fn get_possible_moves(
    viable_cells: Vec<(i32, usize, usize)>,
) -> Vec<(i32, usize, usize, i32, usize, usize)> {
    // find the possible moves
    let mut possible_moves = vec![];
    for (cell_value_1, x1, y1) in viable_cells.clone() {
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
    }

    return possible_moves;
}

#[test]
fn test_one() {
    let mut game_map = vec![
        vec![0, 0, 0, 4, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 2, 1],
    ];

    let out = compute_steps(&mut game_map);

    assert_eq!(out, vec!["3 0 D -", "3 4 R -", "6 4 R -"]);
}

#[test]
fn test_two() {
    let mut game_map = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![11, 0, 0, 0, 6, 0, 0, 7],
    ];

    let out = compute_steps(&mut game_map);

    assert_eq!(out, vec!["4 2 D -", "4 4 L -", "0 4 R -"]);
}

#[test]
fn test_three() {
    let mut game_map = vec![
        vec![0, 3, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 3, 0, 0, 0, 0, 0, 0],
        vec![0, 6, 0, 2, 0, 0, 0, 0],
        vec![0, 2, 0, 0, 0, 0, 0, 0],
    ];

    let out = compute_steps(&mut game_map);

    assert_eq!(out, vec!["1 0 D -", "3 3 L -", "1 3 U -", "1 2 D -"]);
}

#[test]
fn test_four() {
    let mut game_map = vec![
        vec![0, 0, 0, 3, 0, 0, 0, 0],
        vec![4, 0, 0, 0, 1, 0, 0, 3],
        vec![0, 0, 0, 4, 0, 1, 3, 0],
        vec![0, 0, 0, 4, 1, 1, 0, 0],
        vec![0, 0, 2, 0, 1, 0, 0, 0],
    ];

    let out = compute_steps(&mut game_map);

    assert_eq!(
        out,
        vec![
            "3 0 D -", "0 1 R +", "7 1 L -", "4 1 D -", "5 2 D +", "6 2 L -", "3 2 D +", "3 3 R -",
            "4 3 D +", "2 4 R -"
        ]
    );
}
