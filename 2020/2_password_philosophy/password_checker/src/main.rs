use std::env;
use std::error;
use std::io::{Error, ErrorKind};
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
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: password_checker <passwordlist>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_str_vec_from_file(filename) {
        Ok(passwd_vec) => {
            println!("Successful: {:?}", passwd_vec);
            let count = passwd_vec.into_iter()
                .map(|line| extract_policy_and_password_from_str(line).unwrap())
                .map(|(pp, passwd)| check_password_to_policy(&pp, &passwd))
                .filter(|res| *res).count();

            println!("Counted: {}", count);
            /*for line in passwd_vec {
                let ppp: (PasswordPolicy, String) =
                    extract_policy_and_password_from_str(line).unwrap();
                let ppp_ok = check_password_to_policy(&ppp.0, &ppp.1);
                println!("PP {:?} and password \"{}\", OK? {}", ppp.0, ppp.1, ppp_ok);
            }*/
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
//let lines: Result<i32, _> =

    //let lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    //Ok(lines_iter.collect::<Vec<String>>())
    Ok(buf.lines()
        .map(|p| p.unwrap())
        .collect::<Vec<String>>())
    //Map a possible ParseIntError onto Box Error
    //.map_err(|e| e.into())
}

/// Typical format is "3-4 t: dttt"
fn extract_policy_and_password_from_str(passwd_line: String)
                                        -> Result<(PasswordPolicy, String), Box<dyn error::Error>>
{
    // lines typically look like this: "3-10 g: gggxwxggggkgglklhhgg"
    // So let's capture the ints out of this
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    match re.captures(&passwd_line) {
        Some(caps) => {
            //println!("{}", &line_as_str);
            let pp = PasswordPolicy {
                min: caps[1].parse::<usize>().unwrap(),
                max: caps[2].parse::<usize>().unwrap(),
                letter: caps[3].to_string().chars().next().unwrap(),
            };
            let password = caps[4].to_string();

            //println!("{:?}", pp);
            return Ok((pp, password));
        }
        None => println!("Could not parse")
    }
    Err(Box::new(Error::new(ErrorKind::Other, "oh no!")))
}

fn check_password_to_policy(pp: &PasswordPolicy, passwd: &String) -> bool {
    let first_pos: bool = match passwd.chars().nth(pp.min-1)
    {
        Some(l) => {
            if l == pp.letter {
                true
            } else {
                false
            }
        }
        None => { false }
    };

    let second_pos = match passwd.chars().nth(pp.max-1)
    {
        Some(l) => { l == pp.letter }
        None => {false}
    };
    first_pos ^ second_pos

    //let c = passwd.matches(pp.letter).count();
    //c >= pp.min && c <= pp.max
}