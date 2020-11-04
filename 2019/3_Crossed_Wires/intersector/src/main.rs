/*extern crate term_painter;

use self::term_painter::{ToStyle, Color};
use self::term_painter::Color::{White, Blue, Red, Yellow, Green};

fn main() {
    let str = "...........\n...........\n...........\n....+----+.\n....|....|.\n\
    ....|....|.\n....|....|.\n.........|.\n.o-------+.\n...........";
    for c in str.chars() {
        match c {
            '.' => print!("{}", White.paint(".")),
            (x) => print!("{}", Yellow.paint(x)),
        }
    }
}*/

use std::env;
//use std::fs::File as File;
use std::error;
//use std::io::ErrorKind as ErrorKind;
use std::ffi::OsString;

use std::fmt;

#[derive(Debug, Clone)]
struct DirectionError;

impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction can only be D, U, L or R.!")
    }
}

impl std::error::Error for DirectionError {}

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: intersector <wiredescribefile>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_wirepath_from_file(filename) {
        Ok(wirepath_vec) => {
            for wire in &wirepath_vec {
                let pts = parse_to_lines(wire);
                println!("List of points: {:?}", pts)
            }
            //let res = execute_modified_program(opcode_vec);
            println!("Parsed: {:?}", wirepath_vec);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}

fn get_args() -> Result<String, ArgsError> {
    // Prints each argument on a separate line

    match env::args_os().count() {
        n if n > 2 => return Err(ArgsError::TooManyArgs(n - 1)),
        n if n < 2 => return Err(ArgsError::NotEnoughArgs),
        _ => {}
    }

    env::args_os()
        //get me the first two
        .skip(1)
        .take(1)
        //map OsString into utf8
        .map(|oss| oss.into_string())
        // collect to get the Results on the outside
        .collect::<Result<Vec<_>, _>>()
        //convert vector into tuple of Strings
        .map(|mut v| (v.remove(0)))
        //wrap conversion error into our Error
        .map_err(|oss| ArgsError::NotUtf8(oss))
}

fn collect_wirepath_from_file(inputname: String) -> Result<Vec<Vec<String>>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);

    Ok(buf.lines().map(|l| l.expect("Could not parse line"))
        .map(|l| l.split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>())
}

fn parse_to_lines(wirepath: &Vec<String>) -> Result<Vec<(isize, isize)>, Box<dyn error::Error>> {
    let mut meet_pt: (isize, isize) = (0, 0);
    wirepath.into_iter()
        .map(|s| get_direction_and_dist(s))
        .map(|dir_dist|
            match dir_dist {
                Ok(('D', dist)) => {
                    meet_pt = (meet_pt.0, meet_pt.1 + dist);
                    Ok(meet_pt)
                }
                Ok(('U', dist)) => {
                    meet_pt = (meet_pt.0, meet_pt.1 - dist);
                    Ok(meet_pt)
                }
                Ok(('R', dist)) => {
                    meet_pt = (meet_pt.0 + dist, meet_pt.1);
                    Ok(meet_pt)
                }
                Ok(('L', dist)) => {
                    meet_pt = (meet_pt.0 - dist, meet_pt.1);
                    Ok(meet_pt)
                }
                _ => {
                    Err(DirectionError.into())
                }
            }
        )
        .collect::<Result<Vec<(isize, isize)>, Box<dyn error::Error>>>()
}

///
/// Decompose elements like "R105" into ('R', 105).
///
fn get_direction_and_dist(stretch_of_path: &String) -> Result<(char, isize), Box<dyn error::Error>> {
    let dir = stretch_of_path.chars().nth(0).unwrap();
    let dist = stretch_of_path[1..].parse::<isize>().unwrap();
    Ok((dir, dist))
}