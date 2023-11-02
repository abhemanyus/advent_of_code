fn main() {
    let input = advent_of_code::load_file!("thirteen");
    let first = input.split('\n').next().unwrap();
    println!("{}", first);
    let list = Item::from_str(first);
    dbg!(&list);
    println!("{}", Item::to_string(&list));
}

#[derive(Debug)]
enum Item {
    Number(u32),
    List(Vec<Item>),
}

impl Item {
    fn from_str(s: &str) -> Vec<Item> {
        let mut list = Vec::new();
        let tokens: Vec<String> = s
            .to_string()
            .split(",")
            .map(|s| s.to_string())
            .collect();
        Self::from_str_self(&tokens, &mut list);
        list
    }
    fn from_str_self(s: &[String], list: &mut Vec<Item>) {
        for token in s {
            match token.as_str() {
                "[" => {
                    let mut sub_list = Vec::new();
                    Self::from_str_self(&s[1..], &mut sub_list);
                    list.push(Item::List(sub_list));
                }
                "]" => return,
                digit => {
                    list.push(Item::Number(digit.parse::<u32>().unwrap()));
                }
            }
        }
    }
    fn to_string(list: &Vec<Item>) -> String {
        let mut string = String::from("[");
        Self::to_string_self(list, &mut string);
        string.push_str("]");
        string
    }
    fn to_string_self(list: &Vec<Item>, string: &mut String) {
        for item in list {
            match item {
                Item::Number(n) => {
                    string.push_str(&n.to_string());
                }
                Item::List(l) => {
                    string.push_str("[");
                    Item::to_string_self(l, string);
                }
            }
        }
        string.push_str("]")
    }
}
