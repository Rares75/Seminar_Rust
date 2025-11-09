use std::cmp;
fn problem1(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    if x % 2 == 0 && x != 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    let mut d = 3;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d = d + 2;
    }
    true
}
fn problem2(x: i32, y: i32) -> bool {
    let k = cmp::min(x, y);
    for i in 2..k {
        if x % i == 0 && y % i == 0 {
            return false;
        }
    }
    true
}
fn problem3(x: i32) {
    println!(
        "{} bottles of beer on the wall,
              {} bottles of beer.
              Take one down, pass it around,
            {} bottles of beer on the wall.",
        x,
        x,
        x - 1
    );
}
fn main() {
    if false {
        for i in 0..100 {
            if problem1(i) {
                println!("{} number is prime", i);
            } else {
                println!("{} number is not prime", i);
            }
        }
    }
    if true {
        for i in 1..100 {
            for j in 1..100 {
                if problem2(i, j) {
                    println!("{} and {} are coprime", i, j);
                } else {
                    println!("{}and {} are not coprime", i, j);
                }
            }
        }
    }
    if false {
        for i in (1..100).rev() {
            problem3(i);
        }
    }
}
