use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn part1() -> io::Result<()> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let path = Path::new("data/input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(msg) => panic!("Couldn't open {}: {}", display, msg),
        Ok(f) => f,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let res = line?;
        let mut parts = res.split_whitespace();

        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            if let (Ok(li), Ok(ri)) = (l.parse::<i32>(), r.parse::<i32>()) {
                // println!("Left: {}, Right: {}", li, ri);
                left.push(li);
                right.push(ri);
            } else {
                println!("Failed to parse {}, {} to int", l, r);
            }
        } else {
            println!("Failed to split {}", res);
        }
    }

    left.sort();
    right.sort();

    let mut ans: i32 = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        ans += (r - l).abs();
    }
    println!("{}", ans);

    Ok(())
}

fn part2() -> io::Result<()> {
    let mut left: Vec<i32> = Vec::new();
    let mut counts: HashMap<i32, i32> = HashMap::new();

    let path = Path::new("data/input.txt");
    let file = File::open(&path)?;

    // Why no error checker on this?
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let res = line?;
        let mut parts = res.split_whitespace();

        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            if let (Ok(li), Ok(ri)) = (l.parse::<i32>(), r.parse::<i32>()) {
                let k = match counts.get(&ri) {
                    Some(v) => v + 1,
                    None => 1i32,
                };
                counts.insert(ri, k);
                left.push(li);
            } else {
                println!("Failed to parse {}, {} to int", l, r);
            }
        } else {
            println!("Failed to split {}", res);
        }
    }

    let mut ans: i32 = 0;
    for l in left.iter() {
        let mul = match counts.get(&l) {
            Some(v) => v,
            None => &0i32,
        };
        ans += l * mul;
    }
    println!("{}", ans);

    Ok(())
}

fn main() {
    let _ = part1();
    let _ = part2();
    ()
}
