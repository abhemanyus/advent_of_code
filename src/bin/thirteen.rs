use std::str::Chars;

fn main() {
    let sample = "[1,[2,3],4,5]";
    let list = parse_list(&mut sample.chars());
    println!("list: {:?}", list);
}

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Value(i32),
}

fn parse_list(characters: &mut Chars) -> Vec<Item> {
    let mut vec = Vec::new();
    let mut buffer = String::new();
    loop {
        let Some(c) = characters.next() else {
            break;
        };
        match c {
            '[' => {
                vec.push(Item::List(parse_list(characters)));
            }
            ']' => {
                if let Ok(num) = buffer.parse::<i32>() {
                    vec.push(Item::Value(num));
                    buffer.clear();
                }
                return vec;
            }
            ',' => {
                if let Ok(num) = buffer.parse::<i32>() {
                    vec.push(Item::Value(num));
                    buffer.clear();
                }
            }
            digit => {
                buffer.push(digit);
            }
        }
    }
    vec
}
