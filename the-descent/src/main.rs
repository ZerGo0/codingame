use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct Mountain {
    index: i32,
    height: i32,
}

fn main() {
    // game loop
    loop {
        let mut max_mountain = Mountain {
            index: 0,
            height: 0,
        };

        for i in 0..8 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let mountain_h = parse_input!(input_line, i32);
            if mountain_h > max_mountain.height {
                max_mountain.height = mountain_h;
                max_mountain.index = i as i32;
            }
        }

        println!("{}", max_mountain.index);
    }
}
