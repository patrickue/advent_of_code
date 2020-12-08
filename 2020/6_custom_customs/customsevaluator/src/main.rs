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
            println!("{:?} Usage: customsevaluator <customforms>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(answer_vec) => {
            println!("Successful: {:?}", answer_vec);

            let total_count = count_all_positive_group_answers(answer_vec);

            println!("Sum answers of all groups: {}", total_count);
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

fn count_all_positive_group_answers(sheet_lines: Vec<String>) -> usize {
    let mut found_answers_vec: Vec<char> = vec![];
    let mut group_id = 0;
    let mut total_answers = 0;
    let mut new_group = true;

    for line in sheet_lines {

        // On empty line, a new group starts
        if line == "" {
            //new group, new answers
            let group_answer_count = found_answers_vec.len();
            total_answers += group_answer_count;

            println!("Group ID: {}, has {} answers, total: {}. {:?}",
                     group_id, group_answer_count, total_answers, found_answers_vec);

            found_answers_vec = vec![];
            group_id += 1;
            new_group = true;
        }
        else
        {

            if new_group {
                //let's fill the first line
                line.chars().into_iter()
                    .map(|c| found_answers_vec.push(c))
                    .for_each(drop);
                new_group = false;
            }
            else
            {
                println!("Vec before: {:?}", found_answers_vec);
                found_answers_vec = found_answers_vec
                    .into_iter()
                    .filter(|c| line.contains(*c))
                    .collect();
                println!("Vec after: {:?}", found_answers_vec);
                /*for c in found_answers_vec.chars() {
                    if !found_answers_vec.contains(&c) {
                       found_answers_vec.push(c);
                       group_answer_count += 1;
                    }
                }*/
            }
        }
    }
    total_answers
}