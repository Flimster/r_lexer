use std::fs::File;
use std::io::Read;
use std::time;
use std::time::SystemTime;

#[derive(Debug)]
enum Token {
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    String,
    Digit,
    ArrayBegin,
    ArrayEnd,
    Colon,
    Comma,
}


fn main() {
    let mut file = File::open("test/test.json").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let mut line = 0;
    let mut column = 0;

    let mut chars = s.chars();

    let now = SystemTime::now();
    loop {
        column += 1;
        let mut c = chars.next();
        match c {
            Some('{') => println!("Matched OpeningCurlyBracket token"),
            Some('}') => println!("Matched ClosingCurlyBracket token"),
            Some('[') => println!("Matched ArrayBegin"),
            Some(']') => println!("Matched ArrayEnd"),
            Some(':') => println!("Matched Colon"),
            Some('"') => {
                let mut a = String::new();
                a.push(c.unwrap());
                c = chars.next();
                while c != Some('"') {
                    a.push(c.unwrap());
                    c = chars.next();
                }
                a.push(c.unwrap());
                println!("Got string '{}'", a);
            }
            Some(',') => println!("Matched comma"),
            Some('\n') => {line += 1},
            Some(' ') => println!("Matched space"),
            None => break,
            _ => {
                let d = c.unwrap();
                if d.is_digit(10) {
                    let mut dd = d.to_digit(10).unwrap();
                    c = chars.next();
                    while c != Some(',') {
                        dd = c.unwrap().to_digit(10).unwrap() + dd * 10;
                        c = chars.next();
                    }
                    println!("Matched digit {}", dd);
                } else {
                    eprintln!("Error, unrecognized token '{:#?}'", c)
                }
            }
        };
    }
    let after = SystemTime::now();

    println!("Time is {:#?}", now.elapsed());
}
