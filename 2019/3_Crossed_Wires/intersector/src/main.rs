/*extern crate term_painter;

use self::term_painter::{ToStyle, Color};
use self::term_painter::Color::{White, Blue, Red, Yellow, Green};

fn main() {
    let str = "...........\n...........\n...........\n....+----+.\n....|....|.\n\
    ....|....|.\n....|....|.\n.........|.\n.o-------+.\n...........";
    for c in str.chars() {
        match c {
            '.' => print!("{}", White.paint(".")),
            (x) => print!("{}", Yellow.paint(x)),
        }
    }
}*/

use std::env;
//use std::fs::File as File;
use std::error;
//use std::io::ErrorKind as ErrorKind;
use std::ffi::OsString;

use std::fmt;
use geo::line_string;
use geo::{Coordinate, Point, Line};
use geo::algorithm::intersects::Intersects;
use geo::algorithm::euclidean_length::EuclideanLength;
use line_intersection::{LineRelation, LineInterval};

#[derive(Debug, Clone)]
struct DirectionError;




impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction can only be D, U, L or R.!")
    }
}

impl std::error::Error for DirectionError {}

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
            println!("{:?} Usage: intersector <wiredescribefile>", text);
            std::process::exit(1);
        }
    };
    let mut line_list: Vec<Vec<Line<isize>>> = Vec::new();
    println!("Args: {:?}", filename);
    match collect_wirepath_from_file(filename) {
        Ok(wirepath_vec) => {
            for wire in &wirepath_vec {
                let pts = parse_to_endpoints(wire);
                println!("List of points: {:?}", pts);
                let lines = convert_endpoints_to_lines(pts.unwrap());
                //println!("List of lines: {:?}", lines);
                line_list.push(lines);
            }
            let intersec = intersect_all_lines(line_list.get(0).unwrap().to_vec(),
                                               line_list.get(1).unwrap().to_vec());
            println!("Intersections: {:?}", intersec);
            /*println!("Manhattan distances: {:?}", (intersec.into_iter()
                .map(|p| p.x().abs() + p.y().abs())
                .collect::<Vec<isize>>()));*/

            let steps1 = calc_steps_to_intersection(&intersec, line_list.get(0).unwrap().to_vec());
            println!("Steps Line1: {:?}", steps1);
            let steps2 = calc_steps_to_intersection(&intersec, line_list.get(1).unwrap().to_vec());
            println!("Steps Line2: {:?}", steps2);
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

fn collect_wirepath_from_file(inputname: String) -> Result<Vec<Vec<String>>, Box<dyn error::Error>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let fi = File::open(inputname)?;

    let buf = BufReader::new(fi);

    Ok(buf.lines().map(|l| l.expect("Could not parse line"))
        .map(|l| l.split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>())
}

fn parse_to_endpoints(wirepath: &Vec<String>) -> Result<Vec<Point<isize>>, Box<dyn error::Error>> {
    let mut meet_pt: Point<isize> = Coordinate { x: 0, y: 0 }.into();
    wirepath.into_iter()
        .map(|s| get_direction_and_dist(s))
        .map(|dir_dist|
            match dir_dist {
                Ok(('D', dist)) => {
                    meet_pt.set_y(meet_pt.y() + dist);
                    Ok(meet_pt.clone())
                }
                Ok(('U', dist)) => {
                    meet_pt.set_y(meet_pt.y() - dist);
                    Ok(meet_pt.clone())
                }
                Ok(('R', dist)) => {
                    meet_pt.set_x(meet_pt.x() + dist);
                    Ok(meet_pt.clone())
                }
                Ok(('L', dist)) => {
                    meet_pt.set_x(meet_pt.x() - dist);
                    Ok(meet_pt.clone())
                }
                _ => {
                    Err(DirectionError.into())
                }
            }
        )
        .collect::<Result<Vec<Point<isize>>, Box<dyn error::Error>>>()
}

///
/// Decompose elements like "R105" into ('R', 105).
///
fn get_direction_and_dist(stretch_of_path: &String) -> Result<(char, isize), Box<dyn error::Error>> {
    let dir = stretch_of_path.chars().nth(0).unwrap();
    let dist = stretch_of_path[1..].parse::<isize>().unwrap();
    Ok((dir, dist))
}


fn convert_endpoints_to_lines(test_vec: Vec<Point<isize>>) -> Vec<Line<isize>>
{
    let mut res_vec: Vec<Line<isize>> = Vec::new();
    let mut peekable_iter = test_vec.into_iter().peekable();
    let mut prev = Coordinate{ x: 0, y: 0};
    while peekable_iter.peek() != None {
        let start_pt = prev;
        let end_pt = peekable_iter.next().unwrap();
        res_vec.push(
            Line {
                start: start_pt,
                end: end_pt.into()
            }
                );
        prev = end_pt.into();
    }
    res_vec
}


fn intersect_all_lines(linevec1: Vec<Line<isize>>, linevec2: Vec<Line<isize>>)
                       -> Vec<Point<isize>> {
    let mut res: Vec<Point<isize>> = Vec::new();
    for a_line in linevec1 {
        for b_line in &linevec2
        {
            let a_line_f32: Line<f32> = Line{
                start: (a_line.start.x as f32, a_line.start.y as f32).into(),
                end: (a_line.end.x as f32, a_line.end.y as f32).into(),
            };
            let line_interv: LineInterval<f32> = LineInterval::line_segment(a_line_f32);
            let b_line_f32: Line<f32> = Line{
                start: (b_line.start.x as f32, b_line.start.y as f32).into(),
                end: (b_line.end.x as f32, b_line.end.y as f32).into(),
            };
            let b_line_interv: LineInterval<f32> = LineInterval::line_segment(b_line_f32);
            let intersec = line_interv.relate(&b_line_interv).unique_intersection();
            match intersec {
                Some(p) => {
                    let point_isize = Point::new(
                        p.x() as isize,
                        p.y() as isize);
                    res.push(point_isize.into());
                }
                None => {}
            }
        }
    }
    return res;
}

fn calc_steps_to_intersection(intersections: &Vec<Point<isize>>, lines: Vec<Line<isize>>)
    -> Vec<(Point<isize>, isize)>
{
    let mut step_cnt = 0;
    let mut steps_to_intersect = Vec::new();
    for line in lines {
        for p in intersections.into_iter() {
            if p.intersects(&line) {
                let partial_line_start_to_intersect: Line<f64> = Line::new(
                    (line.start.x as f64, line.start.y as f64), (p.x() as f64, p.y() as f64));
                let distance_from_linestart_to_intersection =
                    partial_line_start_to_intersect.euclidean_length() as isize;
                let mut something = (*p, step_cnt + distance_from_linestart_to_intersection);
                steps_to_intersect.push(something);
            }
            else {
            }
        }
        let tmp_line = convert_line_isize_to_f64(line);
        let line_len = tmp_line.euclidean_length() as isize;
        step_cnt += line_len;
    }
    return steps_to_intersect;
}

fn convert_line_isize_to_f64(line: Line<isize>) -> Line<f64> {
    Line::new(
        Point::new(line.start.x as f64, line.start.y as f64),
        Point::new(line.end.x as f64, line.end.y as f64)
    )
}