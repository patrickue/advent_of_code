
use std::env;
use std::ffi::OsString;
use regex::Regex;
use std::fmt;


struct Patch {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl fmt::Display for Patch { 
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "#{}[x: {}, y: {}, w: {}, h: {}]", self.id, self.x, self.y, self.width, self.height) 
        //write!(f, "[x: {}, y: {}, w: {}, h: {}]", self.x, self.y, self.width, self.height) 
    }
}

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

fn testing(filename: String) -> Result<Vec<Patch>, std::io::Error> {

    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let fi = File::open(filename)?;

    let buf = BufReader::new(fi);

    let mut patchvec: Vec<Patch> = Vec::new();

    // lines typically look like this: #462 @ 5,820: 20x24
    // So let's capture the ints out of this
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in buf.lines() {

        let line_as_str = line?;
        match re.captures(&line_as_str) {
            Some(caps) => {
                //println!("{}", &line_as_str);
                let patch = Patch {
                    id: caps[1].parse::<usize>().unwrap(),
                    x: caps[2].parse::<usize>().unwrap(),
                    y: caps[3].parse::<usize>().unwrap(),
                    width: caps[4].parse::<usize>().unwrap(),
                    height: caps[5].parse::<usize>().unwrap(),
                };
                println!("{}", patch);
                patchvec.push(patch);
            },
            None => println!("Could not parse")
        }
    }
    return Ok(patchvec);
}

/*fn parse_patch(line_as_str: String) -> Patch {

        match re.captures(&line_as_str) {
            Some(caps) => {
                //println!("{}", &line_as_str);
                let patch = Patch {
                    id: 0, //caps[1].parse::<usize>().unwrap(),
                    x: caps[2].parse::<usize>().unwrap(),
                    y: caps[3].parse::<usize>().unwrap(),
                    width: caps[4].parse::<usize>().unwrap(),
                    height: caps[5].parse::<usize>().unwrap(),
                };
                println!("{}", patch);
                if patch.id < 100 {
                    patchvec.push(patch);
                }
            },
            None => println!("Could not parse")
}*/

fn mark_patch(grid: &mut Box<[[usize; 1000]; 1000]>, patch: &Patch) {

    let start_x = patch.x as usize;
    let end_x = (patch.x+patch.width) as usize;
    let start_y = patch.y as usize;
    let end_y = (patch.y+patch.height) as usize;

    for x in start_x..end_x {
        for y in start_y..end_y {
            grid[x][y] += 1;
        }
    }
}

fn marking_patches(patchvec: Vec<Patch>) {
    let mut grid = Box::new([[0; 1000]; 1000]);
    //let mut grid: [[usize; 1000]; 1000] = box [[0; 1000]; 1000];

    let mut counter = 0;

    for patch in patchvec.iter() {
        mark_patch(&mut grid, patch.clone());
    }

    for line in grid.iter() {
        for elem in line.iter()
        {
            print!("{}", elem);
            if *elem > 1 { counter += 1; }
        }
        print!("\n");
    }

    println!("Found {} patches with multiple usage.", counter);
    //println!("{:?}", grid);
    //
    for patch in patchvec.iter() {
        check_patch(&grid, patch.clone());
    }
}

fn check_patch(grid: &Box<[[usize; 1000]; 1000]>, patch: &Patch) {
    let mut patch_overlapped = false;
    let start_x = patch.x as usize;
    let end_x = (patch.x+patch.width) as usize;
    let start_y = patch.y as usize;
    let end_y = (patch.y+patch.height) as usize;

    for x in start_x..end_x {
        for y in start_y..end_y {
            if grid[x][y] > 1 {
                patch_overlapped = true;
            }
        }
    }

    if !patch_overlapped {
        println!("Patch #{} did not overlap!", patch.id);
    }

}

fn main() {
     let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: slicing <source>", text);
            std::process::exit(1);
        },
    };
    println!("Args: {:?}", filename);
    let patchvec = testing(filename); 
    marking_patches(patchvec.unwrap());
}
