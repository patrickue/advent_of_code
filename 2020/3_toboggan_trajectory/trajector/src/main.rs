use std::env;
use std::error;
use std::ffi::OsString;

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
            println!("{:?} Usage: trajector <forrestfile>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_string_vec_from_file(filename) {
        Ok(forrest_vec) => {
            let mut tree_product = 1;
            for x in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
                let tree_cnt = calculate_trees(&forrest_vec, x.0, x.1);
                tree_product *= tree_cnt;
                println!("Trees for {}{}: {}", x.0, x.1, tree_cnt);
            }
            println!("Tree product: {:?}", tree_product);
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

fn collect_string_vec_from_file(inputname: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);

    //let lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    //Ok(lines_iter.collect::<Vec<String>>())
    Ok(buf.lines()
        .map(|f| f.unwrap())
        .collect::<Vec<String>>())
    //Map a possible ParseIntError onto Box Error
    //.map_err(|e| e.into())
}

fn calculate_trees(forrest: &Vec<String>, right_shift: usize, down_shift: usize) -> usize {
    let mut count = 0;
    let mut current_x = 0;
    let mut current_y = 0;
    while current_y < forrest.len() {
        let mut tree_line = &forrest[current_y];
        let mut tree_line_char = tree_line.chars();
        if current_x >= tree_line.len() {
            current_x = current_x % tree_line.len();
        }
        match tree_line_char.nth(current_x) {
            Some('#') => count += 1,
            Some('.') => {},
            a => { unreachable!("{:?}", current_x) }
        }
        current_x += right_shift;
        current_y += down_shift;
    }
    return count;
}
