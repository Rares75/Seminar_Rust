fn add_chars_n_p1(mut s: String, n: char, mut k: i64) -> String {
    while k != 0 {
        s.push(n);
        k -= 1;
    }
    s
}
fn add_char_n_p2(s: &mut String, n: char, mut k: i64) {
    while k != 0 {
        s.push(n);
        k -= 1;
    }
}
fn add_spaces(s: &mut String, mut n: i64) {
    while n != 0 {
        s.push(' ');
        n -= 1;
    }
}
fn add_str(s: &mut String, s2: String) {
    s.push_str(&s2);
}
fn add_integer(mut s: String, mut n: i64) -> String {
    let mut position: i32 = 0;
    let mut new_string: String = String::new();
    if n < 0 {
        s.push('-');
        n = -n;
    }
    while n != 0 {
        if position % 3 != 0 || position == 0 {
            new_string.push((b'0' + (n % 10) as u8) as char);
            position += 1;
        } else {
            new_string.push('_');
            new_string.push((b'0' + (n % 10) as u8) as char);
            position += 1;
        }
        n /= 10;
    }

    new_string = new_string.chars().rev().collect();

    s.push_str(&new_string);
    s
}
fn add_float(s: &mut String, mut n: f64) {
    let mut decimals: i32 = 1;
    if n < 0 as f64 {
        n = -n;
        s.push('-');
    }
    let integer_part: i64 = n as i64;
    let mut float_part: f64 = n - integer_part as f64;

    let mut new_string: String = String::from("");
    new_string = add_integer(new_string, integer_part);
    s.push_str(&new_string);
    s.push(',');
    while float_part != 0.0 && decimals <= 10 {
        float_part *= 10.0;
        let figure = (float_part as i64) % 10;
        s.push((b'0' + figure as u8) as char);
        decimals += 1;
    }
}
fn draw() {
    let mut result = String::from("");

    add_spaces(&mut result, 40);
    add_str(&mut result, String::from("I 💚"));
    result.push('\n');

    add_spaces(&mut result, 40);
    add_str(&mut result, String::from("RUST."));
    result.push_str("\n\n");

    add_spaces(&mut result, 4);
    add_str(&mut result, String::from("Most"));
    add_spaces(&mut result, 12);
    add_str(&mut result, String::from("crate"));
    add_spaces(&mut result, 6);
    let num = add_integer(String::new(), 306_437_968);
    add_str(&mut result, num);
    add_spaces(&mut result, 11);
    add_str(&mut result, String::from("and"));
    add_spaces(&mut result, 5);
    add_str(&mut result, String::from("lastest"));
    add_spaces(&mut result, 9);
    add_str(&mut result, String::from("is"));
    result.push('\n');

    add_spaces(&mut result, 9);
    add_str(&mut result, String::from("downloaded"));
    add_spaces(&mut result, 8);
    add_str(&mut result, String::from("has"));
    add_spaces(&mut result, 13);
    add_str(&mut result, String::from("downloads"));
    add_spaces(&mut result, 5);
    add_str(&mut result, String::from("the"));
    add_spaces(&mut result, 9);
    add_str(&mut result, String::from("version"));
    add_spaces(&mut result, 4);
    add_float(&mut result, 2.038);
    result.push('.');
    result.push('\n');
    result.push('\n');

    println!("{}", result);
}
fn main() {
    let mut s = String::from("");
    let mut s2: String = String::from("RUST");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n_p1(s, c, 26 - i);
        add_char_n_p2(&mut s2, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
    println!("{}", s2);
    let mut s3 = String::from("Exemplu 3");
    add_spaces(&mut s3, 12);
    s3.push_str("am adaugat 12 spatii");
    println!("{}", s3);
    let ct: String = String::from("al 2 lea seminar RUST");
    let mut s4: String = String::from("exemplu 4");
    add_str(&mut s4, ct);
    println!("{}", s4);
    let s5: String = String::from("adaugare_numar_intreg:");
    println!("{}", add_integer(s5, -12345));
    let mut s5: String = String::from("adaugare_numar_float:");
    add_float(&mut s5, 123.567);
    println!("{}", s5);
    draw();
}
