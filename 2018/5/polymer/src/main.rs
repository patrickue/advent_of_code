use std::env;
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
            println!("{:?} Usage: polymer <source>", text);
            std::process::exit(1);
        },
    };
    println!("Found argument: {:?}", filename);
    let polymer = import_polymer(filename);
    //println!("{}", polymer.unwrap());
    let mut polymer_str: String = polymer.unwrap();

    for removed_elem in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'].iter() {

        let mut polymer_str_cpy = polymer_str.clone();

        polymer_str_cpy = remove_problem_unit(polymer_str_cpy, *removed_elem);

        let mut prev_len = polymer_str_cpy.len()+1;
        while prev_len > polymer_str_cpy.len() {  
            //println!("==============");
            prev_len = polymer_str_cpy.len();
            polymer_str_cpy = reduce_polymer(polymer_str_cpy);
        }
        //println!("Result: {}", polymer_str_cpy);
        println!("Removed: {}", removed_elem);
        println!("Len Results: {}", polymer_str_cpy.len());
    }
}

fn remove_problem_unit(original : String, remove_elem : char) -> String {
    let remove_elem_upper = remove_elem.to_ascii_uppercase();
    original.chars().filter(|&c| c != remove_elem_upper).filter(|&c| c != remove_elem).collect()
}

/// Parse the input arguments for the program
fn get_args() -> Result<String, ArgsError>{

    match env::args_os().count() {
        n if n > 2 => return Err(ArgsError::TooManyArgs(n - 1)),
        n if n < 2 => return Err(ArgsError::NotEnoughArgs),
        _ => {}
    }

    env::args_os()
        //get me the first arg
        .skip(1)
        .take(1)
        //map OsString into utf8
        .map( |oss| oss.into_string())
        // collect to get the Results on the outside
        .collect::<Result<Vec<_>, _>>()
        //convert vector into tuple of Strings
        .map(|mut v| (v.remove(0)))
        //wrap conversion error into our Error
        .map_err(|oss| ArgsError::NotUtf8(oss))

}

fn import_polymer(filename: String) -> Result<String, std::io::Error> {

    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let fi = File::open(filename)?;

    let buf = BufReader::new(fi);
    let mut polymer = String::from("");
    for line in buf.lines() {

        let line_as_str = line?;
        polymer.push_str(&line_as_str);
    }
    Ok(polymer)
}

fn reduce_polymer(polymer: String) -> String {
    let mut poly_char: Vec<char> = polymer.chars().collect();
    let mut i = 0;
    while i+1 < poly_char.len() {
        let c = poly_char[i];
        let c_next = poly_char[i+1];
        if c.is_uppercase() {
            // So this is uppercase, let's convert it to lowercase and compare with the following
            // char
            if c.to_ascii_lowercase() == c_next {
                //print!("{}+{}, ", c, c_next);
                poly_char.remove(i+1);
                poly_char.remove(i);
                //do not increase index, because we removed to elements
            }
            else
            {
                i+=1;
            }
        }else
        {
            if c.to_ascii_uppercase() == c_next {
                //print!("{}+{}, ", c, c_next);
                poly_char.remove(i+1);
                poly_char.remove(i);
                //do not increase index, because we removed to elements
            }
            else
            {
                i+=1;
            }
        }
    }
    return poly_char.iter().collect();
}
