use std::fs;
use std::io;

fn problem_1() {
    let s = fs::read_to_string("auxiliar.txt").expect("reading error");
    let mut max_bytes: usize = 0;
    let mut max_length: usize = 0;
    let mut max_line_bytes: usize = 0;
    let mut max_line_size: usize = 0;
    for (index, line) in s.lines().enumerate() {
        if line.len() > max_bytes {
            max_bytes = line.len();
            max_line_bytes = index;
        }
        if line.chars().count() > max_length {
            max_length = line.chars().count();
            max_line_size = index;
        }
    }
    println!(
        "the line with the most size in bytes is: {} , with the value: {} ",
        max_line_bytes, max_bytes
    );
    println!(
        "the longest line is: {} , with the length of: {}",
        max_line_size, max_length
    );
}

fn problem_2(s: String) -> String {
    let mut rot13: String = String::new();
    if !s.is_ascii() {
        panic!("the input string contains non ASCII characters");
    }
    for i in s.chars() {
        if i.is_ascii_lowercase() {
            rot13.push((((i as u8 - b'a') + 13) % 26 + b'a') as char);
        } else if i.is_ascii_uppercase() {
            rot13.push((((i as u8 - b'A') + 13) % 26 + b'A') as char);
        } else {
            rot13.push(i);
        }
    }
    rot13
}
fn problem_3(s: String) -> String {
    let mut return_value: String = String::new();
    if s.is_empty() {
        panic!("you give an empty string as an input");
    }
    for i in s.split_whitespace() {
        if i == "pt" || i == "ptr" {
            return_value.push_str("pentru ");
        } else if i == "dl" {
            return_value.push_str("domnul ");
        } else if i == "dna" {
            return_value.push_str("doamna ");
        } else if i == "dvs" {
            return_value.push_str("dumneavoastra ");
        } else if i == "prof" {
            return_value.push_str("profesor ");
        } else if i == "dr." {
            return_value.push_str("doctor ");
        } else {
            return_value.push_str(i);
            return_value.push(' ');
        }
    }
    return_value
}
fn problem_4() {
    let file = fs::read_to_string("/etc/hosts").expect("can't open /etc/hosts/file");
    let mut index: u32 = 0;

    for line in file.lines() {
        if index < 2
            && !line.starts_with("#")
            && let Some((host, name)) = line.split_once(" ")
        {
            index += 1;
            println!("{}->{}", host, name);
        }
    }
}

fn main() {
    problem_1();
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("cannot read user input");
    let s1: String = String::from("Am fost la dl Matei pt că m-a invitat cu o zi înainte");
    println!("{}", problem_2(s));
    println!("{}", problem_3(s1));
    problem_4();
}
