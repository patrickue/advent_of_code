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

#[derive(Debug)]
struct ObjectInSpace{
    id: String,
    orbits: String,
}

fn main() {
    let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: orbit_mapper <orbit_map>", text);
            std::process::exit(1);
        }
    };
    println!("Args: {:?}", filename);
    let mut ois_vec: Vec<ObjectInSpace> = vec![];
    match collect_usize_vec_from_file(filename) {
        Ok(orbit_vec) => {
            println!("Successful: {:?}", orbit_vec);
            for (id1, id2) in orbit_vec {

                ois_vec.push(ObjectInSpace{
                    id: String::from(id2),
                    orbits: String::from(id1),
                });
            }
            //Check if all orbits exist:
            for ois_1 in &ois_vec {
                let mut link_exists: bool = false;
                for ois_2 in &ois_vec {
                    if ois_1.orbits == ois_2.id {
                        //println!("Matching id! {}", o.id);
                        link_exists = true;
                    }
                    else if ois_1.orbits == "COM" {
                        link_exists = true;
                    }
                }
                if !link_exists {
                    println!("For object {:?} a orbit {} does not exist", ois_1, ois_1.orbits);
                    unreachable!();
                }
            }
            // Count all orbits
            let mut orbit_count = 0;
            for ois in &ois_vec {
                println!("CurrOIS: {:?}", ois);
                orbit_count += 1;
                let mut curr_ois = ois;
                let mut local_orbits = 1;
                while curr_ois.orbits != "COM" {
                   for super_ois in &ois_vec {
                       if super_ois.id == curr_ois.orbits {
                           // found the object we're orbiting
                           curr_ois = super_ois;
                           orbit_count += 1;
                           local_orbits += 1;
                           break;
                       }
                   }
                }
                println!("After OIS: {:?}, orbit Nr: {}", curr_ois, local_orbits);
            }
            println!("Orbit count {}", orbit_count);
            println!("All ObjectsInSpace {:?}", ois_vec);
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

fn collect_usize_vec_from_file(inputname: String) -> Result<Vec<(String, String)>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);
//let lines: Result<i32, _> =

    //let lines_iter = buf.lines().map(|l| l.unwrap());
    // We only expect one line:
    //Ok(lines_iter.collect::<Vec<String>>())
    Ok(buf.lines()
        .map(|p| {
            let res = p.unwrap();
            let x: Vec<&str> = res.split(')').collect();
            (x[0].to_string().clone(), x[1].to_string())
        })
        .collect::<Vec<(String, String)>>())
        //Map a possible ParseIntError onto Box Error
        //.map_err(|e| e.into())
}
