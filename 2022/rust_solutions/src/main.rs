use std::cell::RefCell;
use std::cmp::max;
use std::env;
use std::error;
use std::ffi::OsString;
use std::convert::TryFrom;
use std::error::Error;
use std::rc::Rc;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

#[derive(Copy, Clone)]
#[derive(Debug)]
struct Tree {
    height: usize,
    visible: bool,
}

#[derive(Debug)]
struct Forest {
    rows: Vec<Vec<Tree>>,
    height: usize,
    width: usize,
}

impl Forest {
    pub fn new() -> Forest { Forest { rows: make_tree_row(), height:0, width:0 } }
}

// A free standing private function.
fn make_tree_row() -> Vec<Vec<Tree>> { return Vec::new(); }

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: rust_solutions <input.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(terminal_output) => {
            println!("Successful: {:?}", terminal_output);
            let mut forest = parse_forest(terminal_output).unwrap();
            println!("Parsed forest {:?}", forest);
            let count: usize = count_visible_trees(forest);
            println!("There are {:?} visible trees.", count);
        }
        Err(text) => println!("Error occured: {}", text),
    }
}

fn count_visible_trees(mut forest: Forest) -> usize {
    mark_trees_visible(&mut forest);
    println!("Forest after marking {:?} visible trees.", forest);
    let mut tree_count = 0;
    for x in 0..forest.width {
        for y in 0..forest.height{
            if forest.rows[y][x].visible{
                tree_count += 1;
            }
        }
    }
    return tree_count;
}

fn mark_trees_visible(forest: &mut Forest) -> () {
    for x in 0..forest.width {
        for y in 0..forest.height {
            if is_on_edge(x, y, forest.width, forest.height) {
                forest.rows[y][x].visible = true;
            } else if is_tree_visible_inside(x, y, forest)
            {
                forest.rows[y][x].visible = true;
            }
        }
    }
}

fn is_on_edge(x: usize, y: usize, max_x: usize, max_y: usize) -> bool {
    x == 0 ||
        y == 0 ||
        x == max_x - 1 ||
        y == max_y - 1
}

fn is_tree_visible_inside(x_pos: usize, y_pos: usize, forest: &mut Forest) -> bool {
    let tree_height = forest.rows[y_pos][x_pos].height;
    println!("{:?}", forest.rows[0..x_pos].len());
    let mut visible_from_left = true;
    let mut visible_from_right = true;
    let mut visible_from_top = true;
    let mut visible_from_bottom = true;
    for x in 0..x_pos {
        if forest.rows[y_pos][x].height >= tree_height {
            visible_from_left = false;
        }
    }
    for x in x_pos+1..forest.width {
        if forest.rows[y_pos][x].height >= tree_height {
            visible_from_right = false;
        }
    }
    for y in 0..y_pos {
        if forest.rows[y][x_pos].height >= tree_height {
            visible_from_top = false;
        }
    }
    for y in y_pos+1..forest.height {
        if forest.rows[y][x_pos].height >= tree_height {
            visible_from_bottom = false;
        }
    }
    return visible_from_left
        || visible_from_right
        || visible_from_bottom
        || visible_from_top;
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

fn collect_str_vec_from_file(inputname: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);

    Ok(buf.lines()
        .map(|f| f.unwrap())
        .collect::<Vec<String>>())
}

fn parse_forest<'a>(terminal_output: Vec<String>) -> Result<Forest, Box<dyn error::Error>> {
    let mut forest = Forest::new();

    for line in terminal_output.into_iter()
    {
        let mut result_row: Result<Vec<usize>, std::num::ParseIntError> = line.chars().map(|p| p.to_string().parse::<usize>())
            .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
            //Map a possible ParseIntError onto Box Error
            .map_err(|e| e.into());
        let mut row = result_row.unwrap()
            .into_iter()
            .map(|h| Tree { height: h, visible: false })
            .collect::<Vec<Tree>>();
        forest.rows.push(row);
    }
    forest.height = forest.rows.len();
    forest.width = forest.rows[0].len();
    return Ok(forest);
}