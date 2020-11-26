mod tests;

fn main() {
    let lower_limit = 248345;
    let upper_limit = 746315;
    let mut count = 0;
    for i in lower_limit .. upper_limit {
        if two_digits_adjacent(i) {
            if never_decreases(i) {
                count+=1;
            }
        }
    }
    println!("Passwd amount {}", count);
}

fn two_digits_adjacent(num: isize) -> bool {
    let mut x = num;
    let mut last_digit = x%10;
    let mut was_double = false;
    let mut res_val = false;
    x=x/10;
    while x > 10 {
        if x%10 == last_digit
        {
            if was_double {
                res_val = false;
            } else {
                was_double = true;
                res_val = true;
            }
        } else
        {
            if was_double && res_val { return true; }
            was_double = false;
        }
        last_digit = x%10;
        x=x/10;
    }
    //last digit check
    if last_digit == x%10 {
        if !was_double {
            res_val = true;
        }
        else
        {
            res_val = false;
        }
    }
    return res_val;
}

fn never_decreases(num: isize) -> bool {

    let mut ret_val = true;

    let mut x = num;
    let mut last_digit = x%10;
    x=x/10;
    while x > 10 {
        if x%10 > last_digit {
            ret_val = false;
        }
        last_digit = x%10;
        x=x/10;
    }
    //last digit check
    if x%10 > last_digit {
        ret_val  = false;
    }
    ret_val
}