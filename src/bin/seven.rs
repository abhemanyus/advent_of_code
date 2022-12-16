use std::collections::HashMap;

use advent_of_code::load_file;

fn main() {
    let data = load_file("seven");
    let commands = data
        .split("$ ")
        .filter(|l| !l.is_empty())
        .map(Command::try_from)
        .collect::<Result<Vec<_>, String>>()
        .unwrap();
    let sizes = directory_sizes(commands);
    let clear_size = 30000000 - (70000000 - sizes["//"]);
    let total_size = sizes.values().filter(|s| **s > clear_size).min().unwrap();
    println!("Ding ding, the answer is {total_size}");
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
            .split_once(' ')
            .ok_or(format!("{value} is a single word"))?;
        let entry = if first == "dir" {
            Self::Directory(Directory {
                name: second.to_owned(),
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
#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}
#[derive(Debug)]
struct Directory {
    name: String,
}

#[derive(Debug)]
enum Command {
    List(List),
    ChangeDir(ChangeDir),
}
impl TryFrom<&str> for Command {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (command, other) = value.split_once('\n').ok_or(format!("{value} is empty"))?;
        if let Some(("cd", second)) = command.split_once(' ') {
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
            .split('\n')
            .filter(|l| !l.is_empty())
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

fn directory_sizes(commands: Vec<Command>) -> HashMap<String, usize> {
    let mut stack: Vec<String> = Vec::new();
    let mut sizes: HashMap<String, usize> = HashMap::new();
    let mut cd_count = 0;
    let mut ls_count = 0;
    for command in commands {
        match command {
            Command::List(ls) => {
                ls_count += 1;
                for entry in ls.0 {
                    match entry {
                        Entry::File(file) => {
                            for back in stack.iter() {
                                sizes
                                    .entry(back.to_string())
                                    .and_modify(|v| *v += file.size)
                                    .or_insert(file.size);
                            }
                        }
                        Entry::Directory(dir) => {
                            let dir_size = *sizes.get(&dir.name).unwrap_or(&0);
                            for back in stack.iter() {
                                sizes
                                    .entry(back.to_string())
                                    .and_modify(|v| *v += dir_size)
                                    .or_insert(dir_size);
                            }
                        }
                    }
                }
            }
            Command::ChangeDir(cd) => {
                cd_count += 1;
                match cd {
                    ChangeDir::Up => {
                        stack.pop().unwrap();
                    }
                    ChangeDir::Down(dir) => {
                        let name = stack.join("/") + "/" + &dir;
                        stack.push(name);
                    }
                }
            }
        }
    }
    dbg!(cd_count, ls_count);
    sizes
}
