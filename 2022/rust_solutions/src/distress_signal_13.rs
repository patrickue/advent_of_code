use core::cmp::Ordering;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Result, Value};
use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Right, Left};

#[derive(Debug)]
pub struct IndexedSignalPair {
    pub idx: usize,
    pub pair: RawSignalPair,
}

#[derive(Debug)]
pub struct RawSignalPair {
    pub left: Value,
    pub right: Value,
}

#[derive(Serialize, Deserialize, Debug)]
enum SignalStructure {
    Null,
    Number(Number),
    Array(Vec<SignalStructure>),
}


fn parse_pair(idx: usize, left_str: &String, right_str: &String) -> Result<IndexedSignalPair> {
    let left = serde_json::from_str(left_str)?;
    let right = serde_json::from_str(right_str)?;
    Ok(IndexedSignalPair {
        idx: idx,
        pair: RawSignalPair {
            left,
            right,
        },
    })
}

pub fn split_into_pairs(raw_signal_vec: Vec<String>) -> Result<Vec<IndexedSignalPair>> {
    // let ts: Result<Vec<u8>> = serde_json::from_str(r#"[1, 2, 3]"#);
    // println!("TestStruct: {:?}", ts);

    raw_signal_vec.chunks(3).enumerate().map(
        |(idx, x)| parse_pair(idx, &x[0], &x[1]),
    ).collect::<Result<Vec<IndexedSignalPair>>>().map_err(|e| e.into())
}

pub fn is_pair_in_right_order(left: Value, right: Value) -> Option<bool> {
    return match (left, right) {
        (Value::Array(l), Value::Array(r)) => {
            for pair in l.iter().zip_longest(r.iter()) {
                return match pair {
                    Both(inner_l, inner_r) => is_pair_in_right_order(inner_l.clone(), inner_r.clone()),
                    Left(_) => Some(false),
                    Right(_) => Some(true),
                };
            }
            return None;
        }
        (Value::Array(l), Value::Number(r)) =>
            return is_pair_in_right_order(Value::Array(l), Value::Array(vec!(Value::Number(r)))),
        (Value::Array(l), Value::Null) =>
            return is_pair_in_right_order(Value::Array(l), Value::Array(vec!())),

        (Value::Number(l), Value::Array(r)) =>
            return is_pair_in_right_order(Value::Array(vec!(Value::Number(l))), Value::Array(r)),

        (Value::Number(l), Value::Number(r)) => {
            return match l.as_u64().unwrap().cmp(&r.as_u64().unwrap()) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None
            };
        }
        (Value::Number(_), Value::Null) =>
            return Some(false),
        (Value::Null, Value::Array(r)) =>
            return is_pair_in_right_order(Value::Array(vec!()), Value::Array(r)),
        (Value::Null, Value::Number(_)) =>
            return Some(true),
        (Value::Null, Value::Null) => return None,
        _ => return None,
    };
}