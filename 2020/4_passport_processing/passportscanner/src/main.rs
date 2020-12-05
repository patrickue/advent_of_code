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
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new_empty() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn validate(&self) -> bool {
        //byr (Birth Year) - four digits; at least 1920 and at most 2002.
        match &self.byr {
            Some(b) => {
                if !Passport::validate_str2usize_min_max(b, 1920, 2002)
                {
                    return false;
                }
            }
            None => { return false; }
        }
        //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        match &self.iyr {
            Some(i) => {
                if !Passport::validate_str2usize_min_max(i, 2010, 2020)
                {
                    return false;
                }
            }
            None => { return false; }
        }

        //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        match &self.eyr {
            Some(e) => {
                if !Passport::validate_str2usize_min_max(e, 2020, 2030)
                {
                    return false;
                }
            }
            None => { return false; }
        }

        //hgt (Height) - a number followed by either cm or in:
        //If cm, the number must be at least 150 and at most 193.
        //If in, the number must be at least 59 and at most 76.
        match &self.hgt {
            Some(h) =>
                {
                    if h.contains("in")
                    {
                        let height_str = &h[..2];
                        if !Passport::validate_str2usize_min_max(&height_str, 59, 76)
                        {
                            return false;
                        }
                    } else if h.contains("cm") {
                        let height_str = &h[..3];
                        if !Passport::validate_str2usize_min_max(&height_str, 150, 193)
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            None => { return false; }
        }

        //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        match &self.hcl {
            Some(h) => {
                let re_hcl = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                if !re_hcl.is_match(h) { return false; }
            }
            None => { return false; }
        }

        //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let poss_eye_color = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match &self.ecl {
            Some(e) => {
                if !poss_eye_color.contains(&e.as_str()) { return false; }
            }
            None => { return false; }
        }

        //pid (Passport ID) - a nine-digit number, including leading zeroes.
        match &self.pid {
            Some(p) => {
                let re_hcl = Regex::new(r"^\d{9}$").unwrap();
                if !re_hcl.is_match(p) { return false; }
            }
            None => { return false; }
        }

        //cid (Country ID) - ignored, missing or not.

        // if no invalidation took place, it has to be valid ;)
        true
    }

    fn validate_str2usize_min_max(s: &str, min: usize, max: usize) -> bool {
        match s.parse::<usize>() {
            Ok(i) => {
                if i < min || i > max
                {
                    return false;
                }
            }
            Err(_) => { return false; }
        }
        return true;
    }
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: passportscanner <passportlist.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_string_vec_from_file(filename) {
        Ok(passport_vec) => {
            let valid_pp_count = parse_passport(&passport_vec);
            //println!("Passports: {:?}", passport_vec);
            println!("Valid passports: {}", valid_pp_count);
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

fn parse_passport(passports: &Vec<String>) -> usize {
    let mut valid_passport_count = 0;
    for passport_str in passports {
        let mut passport = Passport::new_empty();
        let fields: Vec<&str> = passport_str.split(' ').collect();

        for field in fields {
            let mut name_val: Vec<&str> = field.split(':').collect();
            match name_val[0] {
                "byr" => { passport.byr = Some(name_val[1].to_string()) }
                "iyr" => { passport.iyr = Some(name_val[1].to_string()) }
                "eyr" => { passport.eyr = Some(name_val[1].to_string()) }
                "hgt" => { passport.hgt = Some(name_val[1].to_string()) }
                "hcl" => { passport.hcl = Some(name_val[1].to_string()) }
                "ecl" => { passport.ecl = Some(name_val[1].to_string()) }
                "pid" => { passport.pid = Some(name_val[1].to_string()) }
                "cid" => { passport.cid = Some(name_val[1].to_string()) }
                a => { unreachable!(a) }
            }
        }
        let valid = passport.validate();
        if valid {
            valid_passport_count += 1;
        }
        println!("{:?}, valid: {}", passport, valid);
        /*
        let mut pp_contains_all_fields = true;
        if pp_contains_all_fields {

            valid_passport_count += 1;
        }*/
    }
    return valid_passport_count;
}