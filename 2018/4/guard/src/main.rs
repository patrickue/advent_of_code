use std::env;
use std::fmt;
use std::ffi::OsString;
use regex::Regex;
use std::collections::HashMap;

use std::time::Instant;
fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000.
        + t.subsec_nanos() as f64 / 1e6
}

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Copy, Clone)]
struct SimpleTime {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

#[derive(Debug)]
struct GuardEvent {
    ts: SimpleTime,
    wakeup: bool,
    guard_nr: Option<usize>,
}

#[derive(Copy, Clone)]
struct SleepWakeCounter {
    awake: usize,
    asleep: usize,
}

impl fmt::Debug for SleepWakeCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.awake, self.asleep)
    }
}

impl SimpleTime {
    fn new() -> Self {
        SimpleTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
        }
    }
}

impl fmt::Debug for SimpleTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}, {}:{}", self.day, self.month, self.year, self.hour, self.minute)
    }
}

trait Parse {
    fn parse(&mut self, string: String);
}

impl Parse for SimpleTime {
    fn parse(&mut self, string: String) {
        //parse is expecting the incoming string to be "YYYY-MM-DD hh:mm"
        let time0 = Instant::now();

        /*
        //The REGEX-Version
        //
        let re = Regex::new(r"^(\d{4})-(\d\d)-(\d\d) (\d\d):(\d\d)$").unwrap();
        match re.captures(&string) {
            Some(caps) => {
                self.year = caps[1].parse::<usize>().unwrap();
                self.month = caps[2].parse::<usize>().unwrap();
                self.day = caps[3].parse::<usize>().unwrap();
                self.hour = caps[4].parse::<usize>().unwrap();
                self.minute = caps[5].parse::<usize>().unwrap();
            },
            None => println!("Could not parse")
        }
        */

        //The String decomposition version
        // About 10000x faster then the RegEx
        self.year = string.get(0..4).unwrap().parse::<usize>().unwrap();
        self.month = string.get(5..7).unwrap().parse::<usize>().unwrap();
        self.day = string.get(8..10).unwrap().parse::<usize>().unwrap();
        self.hour = string.get(11..13).unwrap().parse::<usize>().unwrap();
        self.minute = string.get(14..16).unwrap().parse::<usize>().unwrap();

        let time1 = Instant::now();

        //println!("Y{}M{}D{} h{}m{}", self.year, self.month, self.day, self.hour, self.minute);
        //println!("{}", elapsed_ms(time0, time1));
    } 
}


fn get_args() -> Result<String, ArgsError>{
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

fn import_guardlist(filename: String) -> Result<Vec<GuardEvent>, std::io::Error> {

    use std::fs::File;
    use std::io::{BufReader, BufRead};

    let fi = File::open(filename)?;

    let buf = BufReader::new(fi);

    let mut guard_evt_list: Vec<GuardEvent> = Vec::new();

    // lines typically look like this: [1518-11-07 00:21] falls asleep
    // So let's capture the ints out of this
    let re = Regex::new(r"^\[(.+)\] (.+)$").unwrap();
    let re_guard_nr = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    for line in buf.lines() {

        let line_as_str = line?;
        let guard_evt: Option<GuardEvent> = match re.captures(&line_as_str) {
            Some(caps) => {
                //println!("{}", &line_as_str);
                
                //Let's extract the time first
                let mut ts: SimpleTime = SimpleTime::new();
                ts.parse(caps[1].to_string());
                //println!("{}", date_time);

                let wakeup: bool;
                let mut guard_nr: Option<usize> = None;

                //Now we check the possible content/event:
                let substr: String = caps[2].chars().take(5).collect();
                match substr.as_str() {
                    "falls" => {wakeup = false;},
                    "wakes" => {wakeup = true;},
                    "Guard" => {
                        guard_nr = match re_guard_nr.captures(&caps[2]) {
                            Some(caps) => {
                                //println!("Guard#: {}", &caps[1]);
                                wakeup = true;
                                Some(caps[1].parse::<usize>().unwrap())
                            },
                            None => {
                                    println!("No guard#");
                                    panic!();
                            } 
                        };
                    },
                    _ => {panic!();}
                }

                //Fill the captures values into a Struct
                Some(GuardEvent {ts, wakeup, guard_nr})
            },
            None => { 
                    println!("Could not parse");
                    panic!()
            }
        };
        match guard_evt {
            Some(real_evt) => {
                //println!("{:?}", real_evt);
                guard_evt_list.push(real_evt);
            },
            None => {}
        }
    }
    Ok(guard_evt_list)
}

fn sort_and_fill_guardlist(mut guard_evt_list: Vec<GuardEvent>) -> () {
    guard_evt_list.sort_by(|a, b| a.ts.cmp(&b.ts));

    // Example of a typical guardlist after sorting:
    /*
    [1518-02-26 23:59] Guard #571 begins shift
    [1518-02-27 00:17] falls asleep
    [1518-02-27 00:51] wakes up
    [1518-02-27 00:56] falls asleep
    [1518-02-27 00:59] wakes up
    [1518-02-28 00:00] Guard #263 begins shift
    ...
    */

    let mut guard_map = HashMap::new();
    let mut curr_guard_active: Option<usize> = None;
    let mut prev_event_timestamp: Option<SimpleTime> = None;
    for mut evt in &mut guard_evt_list {

        evt.guard_nr = match evt.guard_nr {
            Some(guard_nr) => {
                //An event with a guard number indicates the start of a shift
                if !guard_map.contains_key(&guard_nr) {
                    let sleepwake_array: Vec<SleepWakeCounter> = vec![SleepWakeCounter{awake: 0, asleep: 0}; 120];
                    guard_map.insert(
                        guard_nr,
                        sleepwake_array
                    );
                }
                else {}
                match curr_guard_active {
                    None => {
                        //Seems to be the first event ever
                    }
                    Some(guard_nr) => {
                        for i in simpletime_to_arridx(evt.ts)..119 {
                            let sleepmap = guard_map.get_mut(&guard_nr).unwrap();
                            sleepmap[i].awake+=1;
                        }
                    }
                }
                curr_guard_active = Some(guard_nr);
                prev_event_timestamp = Some(evt.ts);
                Some(guard_nr)
            },
            None => {
                //An event without a guard number is either wake up or falling asleep of the last
                //active guard
                match prev_event_timestamp {
                    Some(ts) => {
                        //There was an event previous to this, so fill up time between previous and now
                        //with wake/sleep
                        let begin_idx = simpletime_to_arridx(ts);
                        let end_idx = simpletime_to_arridx(evt.ts);

                        let sleepmap = guard_map.get_mut(&curr_guard_active.unwrap()).unwrap();

                        if evt.wakeup {
                            println!("Guard {} asleep between {:?} and {:?}", curr_guard_active.unwrap(), begin_idx, end_idx);
                            for i in begin_idx..(end_idx) {
                                sleepmap[i].asleep+=1;
                            }
                        }else {
                            println!("Guard {} awake between {:?} and {:?}", curr_guard_active.unwrap(), begin_idx, end_idx);
                            for i in begin_idx..(end_idx) {
                                sleepmap[i].awake+=1;
                            }
                        }
                        prev_event_timestamp = Some(evt.ts);
                    },
                    None => {panic!()}
                }

                curr_guard_active
            }
        }
    }
    //println!("List: {:?}", guard_evt_list);
    //println!("Map of Guards: {:?}", guard_map);
    println!(" ===== Output =====");
    print!("     ");
    for i in 0..120 {print!("  {}", i/10);}
    print!("\n     ");
    for i in 0..120 {print!("  {}", i%10);}
    print!("\n");

    for (key, value) in guard_map {
        print!("{:04}:", key);
        let mut sum = 0;
        for i in 0..120 {
            let elem = value[i];
            print!(" {:02}", elem.asleep);
            sum += elem.asleep;
        }
        print!(" sum:{} \n", sum);
    }
}

//Map hours between 23:00 and 23:59 to 0..59, and
// minutes between 00:00 and 00:59 to 60..119
fn simpletime_to_arridx(ts: SimpleTime) -> usize
{
    match ts.hour {
        0 => {ts.minute+60},
        23 => {ts.minute},
        _ => {panic!();}
    }
}

fn main() {
     let filename = match get_args() {
        Ok(a) => a,
        Err(text) => {
            println!("{:?} Usage: guard <source>", text);
            std::process::exit(1);
        },
    };
    println!("Args: {:?}", filename);
    let guard_evt_list_res = import_guardlist(filename); 
    match guard_evt_list_res {
        Ok(guard_evt_list) => {sort_and_fill_guardlist(guard_evt_list);},
        Err(e) => {println!("This Error happened: {}", e);}
    }
}
