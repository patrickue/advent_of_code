use std::cell::RefCell;
use std::env;
use std::error;
use std::ffi::OsString;
use std::convert::TryFrom;
use std::rc::Rc;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

struct Folder {
    file_size: Option<u32>,
    children: Vec<Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
}

impl Folder {
    pub fn new() -> Folder {
        return Folder {
            file_size: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Folder>>) {
        self.children.push(new_node);
    }

    pub fn print(&self) -> String {
        if let Some(value) = self.value {
            return value.to_string();
        } else {
            return String::from("[")
                + &self
                .children
                .iter()
                .map(|tn| tn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
                + "]";
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Instruction {
    op: Operation,
    arg: isize,
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: aoc07_no_space_left_on_device <input.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(terminal_output) => {
            println!("Successful: {:?}", terminal_output);
            tree_structure = parse_terminal_output(terminal_output);
            println!("Parsed instructions {:?}", instructions);
            let loop_acc = check_if_recombination(instructions[0..4].to_vec(), instructions[5]);
            println!("Trying to recombined resulted in: {:?}.", loop_acc);
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

fn collect_str_vec_from_file(inputname: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);

    Ok(buf.lines()
        .map(|f| f.unwrap())
        .collect::<Vec<String>>())
}

fn parse_terminal_output<'a>(terminal_output: Vec<String>) ->  TreeStructure{

    for line in terminal_output.into_iter()
    {
        match line[]
    }
}