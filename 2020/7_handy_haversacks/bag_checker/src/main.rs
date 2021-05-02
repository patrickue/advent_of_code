use std::env;
use std::error;
use std::ffi::OsString;
use regex::Regex;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

#[derive(Debug)]
struct BagRule {
    outside: String,
    inside: Vec<(usize, String)>
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: bag_checker <bag_rules.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(answer_vec) => {
            println!("Successful: {:?}", answer_vec);
            let bagrules = answer_vec.into_iter()
                .map(|l| parse_rules(l))
                .collect::<Vec<BagRule>>();
            let outside_bags = find_bag_amount_inside(&bagrules, "shiny gold".to_string());
            println!("Inside bag amount {:?}", outside_bags);
            //println!("Outside bag-len {:?}", outside_bags.len());
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
    //Map a possible ParseIntError onto Box Error
}

fn parse_rules<'a>(line: String) -> BagRule {
    let line_parts: Vec<&str> = line.split("contain ").collect();
    // Match first part
    let re = Regex::new(r"^(\w+\s\w+)\sbags?\s").unwrap();
    println!("Line: {}#{}", line_parts[0], line_parts[1]);
    let mut outside_color: String = "".to_string();
    for outside in re.captures_iter(line_parts[0]) {
        println!("Outside bag: {}", &outside[1]);
        outside_color = String::from(&outside[1]);
    }

    //Split second part & match
    let mut inside_colors: Vec<(usize, String)> = vec![];
    let second_parts: Vec<&str> = line_parts[1].split(", ").collect();
    for inside in second_parts {
        let re2 = Regex::new(r"^\s*(\d)\s(\w+\s\w+)\sbags?[\s.,]{0,3}").unwrap();
        for inside_part in re2.captures_iter(inside) {
            let amount = &inside_part[1].parse::<usize>().unwrap();
            println!("Inside bag: {}x{}", amount, &inside_part[2]);
            inside_colors.push((*amount, String::from(&inside_part[2])));
        }
    }
    if outside_color == "" {
        unreachable!();
    }

    let returnbagrule = BagRule {
        outside: outside_color.to_string(),
        inside: inside_colors
    };
    return returnbagrule;
}

//fn find_outside_bags

fn find_bag_amount_inside (bag_rules: &Vec<BagRule>, search: String)
    -> usize {

    let mut bag_amount: usize = 0;
    for rule in bag_rules {
        if rule.outside == search {
            if rule.inside.is_empty() {
                //bag_amount = 1;
            }
            else {
                for (amount, inside_bag) in &rule.inside {
                    bag_amount += amount * (1 + find_bag_amount_inside(&bag_rules,
                                                                  inside_bag.to_string()));
                }
            }
        }
    }
    bag_amount
}