use advent_of_code::load_file;

fn main() {
    let data = load_file("seven");
    let list = data
        .split("$ ")
        .filter(|l| l.len() > 0)
        .map(Command::try_from)
        .collect::<Result<Vec<_>, String>>()
        .unwrap();
}

#[derive(Debug)]
enum Entry {
    File(File),
    Directory(Directory),
}
impl TryFrom<&str> for Entry {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (first, second) = value
            .split_once(" ")
            .ok_or(format!("{value} is a single word"))?;
        let entry = if first == "dir" {
            Self::Directory(Directory {
                name: second.to_owned(),
                enteries: Vec::new(),
            })
        } else {
            Self::File(File {
                name: second.to_owned(),
                size: first
                    .parse()
                    .or(Err(format!("{second} is not valid file size")))?,
            })
        };
        Ok(entry)
    }
}
impl Entry {
    fn get_size(&self) -> usize {
        match self {
            Entry::File(file) => file.size,
            Entry::Directory(dir) => dir.get_size(),
        }
    }
}
#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}
#[derive(Debug)]
struct Directory {
    name: String,
    enteries: Vec<Entry>,
}
impl Directory {
    fn get_size(&self) -> usize {
        if self.enteries.len() == 0 {
            0
        } else {
            self.enteries.iter().map(|e| e.get_size()).sum()
        }
    }
}
#[derive(Debug)]
enum Command {
    List(List),
    ChangeDir(ChangeDir),
}
impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (command, other) = value.split_once("\n").ok_or(format!("{value} is empty"))?;
        if let Some(("cd", second)) = command.split_once(" ") {
            Ok(Self::ChangeDir(second.into()))
        } else if command == "ls" {
            Ok(Self::List(other.try_into()?))
        } else {
            Err(format!("{command} is not a valid command"))
        }
    }
}
#[derive(Debug)]
struct List(Vec<Entry>);
impl TryFrom<&str> for List {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let entries = value
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(Entry::try_from)
            .collect::<Result<Vec<Entry>, String>>()?;
        Ok(Self(entries))
    }
}
#[derive(Debug)]
enum ChangeDir {
    Up,
    Down(String),
}
impl From<&str> for ChangeDir {
    fn from(value: &str) -> Self {
        match value {
            ".." => Self::Up,
            dir => Self::Down(dir.to_owned()),
        }
    }
}
