use advent_of_code::load_file;

fn main() {
    let list = list_parser::list(&load_file("one")).unwrap();
    let mut sum: Vec<u32> = list.iter().map(|elf| elf.iter().sum::<u32>()).collect();
    sum.sort();
    let maxes: u32 = sum[sum.len() - 3..].iter().sum();
    println!("Ding ding, the answer is {maxes}!");
}

peg::parser! {
    grammar list_parser() for str {
        rule food() -> u32 = n:$(['0'..='9']+) {? n.parse().or(Err("u32"))}

        rule elf() -> Vec<u32> = e:(food() ** "\n") { e }

        pub rule list() -> Vec<Vec<u32>> = l:(elf() ** "\n\n") { l }
    }
}
