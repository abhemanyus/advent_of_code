use advent_of_code::load_file;

fn main() {
    let data = load_file("four");
    let list: Result<List, String> = data.as_str().try_into();
    let count = list
        .unwrap()
        .0
        .iter()
        .filter(|f| {
            let overlap = f.overlap();
            if overlap.1 >= overlap.0 {
                return true;
            }
            false
        })
        .count();
    println!("Ding ding, the answer is {count}!");
}

#[derive(Clone, Copy, PartialEq)]
struct Section(u32, u32);

impl TryFrom<&str> for Section {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (start, end) = value
            .split_once("-")
            .ok_or(format!("{value} is not valid Section"))?;
        Ok(Self(
            start.parse().or(Err(format!("{start} is not u32")))?,
            end.parse().or(Err(format!("{end} is not u32")))?,
        ))
    }
}

#[derive(Clone, Copy)]
struct Pair(Section, Section);

impl Pair {
    fn overlap(self) -> Section {
        Section((self.0).0.max((self.1).0), (self.0).1.min((self.1).1))
    }
}

impl TryFrom<&str> for Pair {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (first, second) = value
            .split_once(",")
            .ok_or(format!("{value} is not valid Pair"))?;
        Ok(Self(first.try_into()?, second.try_into()?))
    }
}

struct List(Vec<Pair>);

impl TryFrom<&str> for List {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let list = value
            .split("\n")
            .map(Pair::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self(list))
    }
}
