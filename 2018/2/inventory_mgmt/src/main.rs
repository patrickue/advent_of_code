extern crate levenshtein;
use std::collections::btree_map::BTreeMap;
use std::env;
use std::ffi::OsString;
use levenshtein::levenshtein;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}


fn get_args() -> Result<(String), ArgsError>{
    // Prints each argument on a separate line

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

fn testing(filename: String) -> Result<String, std::io::Error> {

    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let mut btreevec: Vec<BTreeMap<char, usize>> = Vec::new();

    let fi = File::open(filename)?;

    let buf = BufReader::new(fi);

    let mut strvec: Vec<String> = Vec::new();

    for line in buf.lines() {

        let mut count = BTreeMap::new();

        let line_as_str = line?;

        for c in line_as_str.clone().chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        // Output results for one line
        //println!("Number of occurences per character");
        //for (ch, count) in &count {
        //    println!("{:?}: {}", ch, count);
        //}

        btreevec.push(count);
        
        // Second task, calculate levenshtein distance
        // to previous lines
        for strelem in &strvec{
            if levenshtein(&strelem, &line_as_str)<=1{
                println!("Found {}{}", strelem, line_as_str);
            }
        }
        strvec.push(line_as_str);
    }
    let mut two_cnt: usize = 0;
    let mut three_cnt: usize = 0;

    for btree in btreevec {
       let mut found_two = false;
       let mut found_three = false;
       for (_key, amount) in &btree {
           if *amount == 2 { found_two = true; }
           if *amount == 3 { found_three = true; }
       }
       if found_two { two_cnt += 1; }
       if found_three { three_cnt += 1; }
    }
    println!("Collecting 2: {}, 3: {}", two_cnt, three_cnt);
    Ok("Something".to_string())

}

fn main() {
     let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: inventory_mgmt <source>", text);
            std::process::exit(1);
        },
    };
    println!("Args: {:?}", filename);
    testing(filename);
    
    println!("{}", levenshtein("kitten", "sitting"));
}
