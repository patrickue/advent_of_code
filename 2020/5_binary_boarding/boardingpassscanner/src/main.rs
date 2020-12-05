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
            println!("{:?} Usage: boardingpassscanner <boardingpasses.txt>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    match collect_string_vec_from_file(filename) {
        Ok(boardingpass_vec) => {
            let mut id_vec = boardingpass_vec.into_iter()
                .map(|b| parse_boarding_code(b))
                .filter_map(Result::ok)
                .map(|(a, b)| a*8+b)
                .collect::<Vec<usize>>();
            id_vec.sort();
            println!("Sorted Vec: {:?}", id_vec);
            for i in 0..880 {
                if !id_vec.contains(&i) {
                    println!("Missing {}", i);
                }
            }
            //println!("Boarding passes: {:?}", boardingpass_vec);
            //println!("Max ID: {:?}", id_vec.into_iter().max());
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

fn parse_boarding_code(bpc: String) -> Result<(usize, usize), Box<dyn error::Error>> {
    let row_code = bpc[..7].to_string();
    let seat_code = bpc[7..].to_string();
    let mut row_bin = row_code.replace("F", "0");
    row_bin = row_bin.replace("B", "1");
    let mut seat_bin = seat_code.replace("L", "0");
    seat_bin = seat_bin.replace("R", "1");

    let row_nr = isize::from_str_radix(row_bin.as_str(), 2).unwrap();
    let seat_nr = isize::from_str_radix(seat_bin.as_str(), 2).unwrap();
    println!("{}, {}", row_nr, seat_nr);
    Ok((row_nr as usize, seat_nr as usize))
}