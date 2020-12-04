use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::result::Result;

pub struct Config {
    mode: u8,
    filename: String,
    num_queens: u32,
    width: u128,
    height: u128,
}

struct Queen {
    col: u128,
    row: u128,
}

struct Board {
    width: u128,
    height: u128,
    queens: Vec<Queen>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let len = args.len() as u32;

        if len != 3 && len != 5 {
            return Err("Usage: cargo run check <filename>\n
                        OR:\tcargo run place <num_queens> <width> <height>"
                .into());
        }

        let mode = match args[1].clone().as_str() {
            "check" => 0,
            "place" => 1,
            _ => 2,
        };

        if mode == 2 || (mode == 0 && len != 3) || (mode == 1 && len != 5) {
            return Err("Usage: cargo run check <filename>
                        OR: cargo run place <num_queens> <width> <height>"
                .into());
        }

        let mut filename = String::from("");
        let mut num_queens: u32 = 0;
        let mut width: u128 = 0;
        let mut height: u128 = 0;

        if mode == 0 {
            filename = args[2].clone();
        } else {
            num_queens = args[2].parse::<u32>()?;
            width = args[3].parse::<u128>()?;
            height = args[4].parse::<u128>()?;
        }

        Ok(Config {
            mode,
            filename,
            num_queens,
            width,
            height,
        })
    }
}

impl Queen {
    pub fn new(coord: String) -> Result<Queen, Box<dyn Error>> {
        let coord = coord.as_str();

        let mut col: u128 = 0;
        let mut index = 0;

        for i in coord.chars() {
            if i.is_ascii_digit() {
                break;
            }

            col = col * 26 + i.to_ascii_lowercase() as u128 - 'a' as u128 + 1;
            index += 1;
        }

        let split = coord.split_at(index);

        let row = split.1.parse::<u128>()?;

        Ok(Queen { col, row })
    }
}

impl Board {
    pub fn new(filename: String) -> Result<Board, Box<dyn Error>> {
        let mut lines = read_lines(filename)?;

        let mut queens: Vec<Queen> = Vec::new();

        let top_line = lines.next().expect("Failure reading width/height")?;
        let mut split = top_line.trim().split_whitespace();

        let width = split
            .next()
            .expect("Failure parsing width")
            .parse::<u128>()?;
        let height = split
            .next()
            .expect("Failure parsing height")
            .parse::<u128>()?;

        for line in lines {
            queens.push(Queen::new(line?)?);
        }

        Ok(Board {
            width,
            height,
            queens,
        })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// This function returns nonsense at the moment
fn all_the_queens_are_safe(board: Board) -> Result<bool, Box<dyn Error>> {
    if board.height == 1 {
        if board.queens[0].col != 1 && board.queens[0].row != 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    } else if board.width == 1 {
        if board.queens[0].col != 1 && board.queens[0].row != 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        if board.queens[0].col == 1 && board.queens[0].row == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// This function returns nonsense at the moment
fn n_queens(num_queens: u32, width: u128, height: u128) -> Result<u32, Box<dyn Error>> {
    let a = width > height;

    if a {
        Ok(num_queens)
    } else {
        Ok(num_queens + 1)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.mode == 0 {
        let board = Board::new(config.filename)?;
        let safe = all_the_queens_are_safe(board)?;

        if safe {
            println!("Queens are safe");
        } else {
            println!("Queens are not safe");
        }
    } else {
        let n = n_queens(config.num_queens, config.width, config.height)?;

        println!("There are {} solutions", n);
    }

    Ok(())
}
