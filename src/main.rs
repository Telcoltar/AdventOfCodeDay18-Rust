mod test_main;

use std::fs::File;
use log::{debug, info};
use std::io::{BufReader, BufRead};
use regex::{Regex, Captures};

fn get_input_data(file_name: &str) -> Vec<String> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut exercise: Vec<String> = Vec::new();

    for line in f.lines() {
        exercise.push(line.unwrap().trim().replace(" ",""));

    }
    return exercise;
}


fn compute_one(first: &str, op: &str, second: &str) -> String {
    let result: i64;
    if op == "+" {
        result = first.parse::<i64>().unwrap() + second.parse::<i64>().unwrap();
    } else {
        result = first.parse::<i64>().unwrap() * second.parse::<i64>().unwrap();
    }
    return result.to_string();
}

fn loop_parts(re: Regex, part: &str) -> String{
    let mut result = part.to_owned();
    while re.find(&result).is_some() {
        result = re.replace(&result,
                            |cap: &Captures|
                                compute_one(&cap[1],&cap[2],&cap[3])).
            parse().unwrap();
        debug!("{:?}", result);
    }
    return result;
}

fn compute_simple_exercise_part_1(part: &str) -> String {
    let re = Regex::new(r"(\d+)([+*])(\d+)").unwrap();
    return loop_parts(re, part);
}

fn solve_exercise_part_1(exercise: &str) -> i64 {
    let re = Regex::new(r"\(([0-9+*]+)\)").unwrap();
    let mut result = exercise.to_owned();
    while re.find(&result).is_some() {
        result = re.replace(&result,
                            |caps: &Captures|
                                compute_simple_exercise_part_1(&caps[1])).parse().unwrap();
        debug!("{:?}", result)
    }
    result = compute_simple_exercise_part_1(&result);
    return result.parse().unwrap();
}

fn compute_simple_exercise_part_2(part: &str) -> String {
    let re = Regex::new(r"(\d+)(\+)(\d+)").unwrap();
    let mut result = loop_parts(re, part);
    let re = Regex::new(r"(\d+)(\*)(\d+)").unwrap();
    result = loop_parts(re, &result);
    return result;
}

fn solve_exercise_part_2(exercise: &str) -> i64 {
    let re = Regex::new(r"\(([0-9+*]+)\)").unwrap();
    let mut result = exercise.to_owned();
    while re.find(&result).is_some() {
        result = re.replace(&result,
                            |caps: &Captures|
                                compute_simple_exercise_part_2(&caps[1])).parse().unwrap();
        debug!("{:?}", result)
    }
    result = compute_simple_exercise_part_2(&result);
    return result.parse().unwrap();
}


fn solution_part_1(file_name: &str) -> i64 {
    let exercises = get_input_data(file_name);
    let mut sum: i64 = 0;
    for exercise in exercises {
        sum += solve_exercise_part_1(&exercise);
    }
    return sum;
}

fn solution_part_2(file_name: &str) -> i64 {
    let exercises = get_input_data(file_name);
    let mut sum: i64 = 0;
    for exercise in exercises {
        sum += solve_exercise_part_2(&exercise);
    }
    return sum;
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
