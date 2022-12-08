use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq)]
struct Directory {
    name: String,
    parents: Vec<String>,
    entries: Vec<Entry>,
}

impl Directory {
    fn path(&self) -> String {
        if self.parents.is_empty() {
            if self.name == "/" {
                self.name.clone()
            } else {
                format!("/{}", self.name)
            }
        } else {
            format!("/{}/{}", self.parents.join("/"), self.name)
        }
    }

    fn parent_path(&self) -> String {
        if self.parents.is_empty() {
            "/".into()
        } else {
            format!("/{}", self.parents.join("/"))
        }
    }
}

#[derive(Debug, PartialEq)]
enum Entry {
    File(File),
    Directory(Directory),
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let root = parse(input);
    let mut directory_totals: HashMap<String, usize> = HashMap::new();
    let mut total_at_most_100k: usize = 0;

    compute_totals(&root, &mut directory_totals);

    for (_name, total) in directory_totals {
        if total <= 100_000 {
            total_at_most_100k += total;
        }
    }

    total_at_most_100k
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let root = parse(input);
    let mut directory_totals: HashMap<String, usize> = HashMap::new();

    compute_totals(&root, &mut directory_totals);

    let mut sorted: Vec<(String, usize)> =
        directory_totals.into_iter().collect();
    sorted.sort_unstable_by(|(_name1, value1), (_name2, value2)| {
        value1.partial_cmp(value2).unwrap()
    });

    let (_name, size) =
        sorted.iter().find(|(_name, size)| *size > 8381165).unwrap();

    *size
}

fn compute_totals(directory: &Directory, totals: &mut HashMap<String, usize>) {
    for entry in &directory.entries {
        match entry {
            Entry::File(file) => {
                totals
                    .entry(directory.path())
                    .and_modify(|total| *total += file.size)
                    .or_insert(file.size);
            }
            Entry::Directory(child_directory) => {
                compute_totals(&child_directory, totals);
                let child_directory_size: usize =
                    *totals.get(&child_directory.path()).unwrap();

                totals
                    .entry(directory.path())
                    .and_modify(|total| *total += child_directory_size)
                    .or_insert(child_directory_size);
            }
        }
    }
}

fn parse(input: &str) -> Directory {
    let mut root = Directory {
        name: "/".into(),
        parents: vec![],
        entries: Vec::new(),
    };

    let mut current_directory = &mut root;

    for line in input.lines() {
        if line.starts_with("$ ") {
            let command: Vec<&str> = line
                .trim()
                .strip_prefix("$ ")
                .unwrap()
                .split_whitespace()
                .collect();

            if command[0] == "cd" {
                match command[1] {
                    "/" => {
                        current_directory = &mut root;
                    }
                    ".." => {
                        let parent_path = current_directory.parent_path();
                        let new_directory = if parent_path == "/" {
                            &mut root
                        } else {
                            find_directory(&mut root, &parent_path)
                                .expect("cd to nonexistent directory")
                        };

                        current_directory = new_directory;
                    }
                    name => {
                        let new_directory = current_directory
                            .entries
                            .iter_mut()
                            .find_map(|entry| match entry {
                                Entry::Directory(directory) => {
                                    if directory.name == name {
                                        Some(directory)
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            })
                            .expect("cd to nonexistent directory");
                        current_directory = new_directory;
                    }
                };
            }
        } else {
            let (dir_or_size, name) = line.split_once(" ").unwrap();
            let entry = if dir_or_size == "dir" {
                let mut current_parents = current_directory.parents.clone();

                if current_directory.name != "/" {
                    current_parents.push(current_directory.name.clone());
                }

                Entry::Directory(Directory {
                    name: name.into(),
                    parents: current_parents,
                    entries: Vec::new(),
                })
            } else {
                Entry::File(File {
                    name: name.into(),
                    size: dir_or_size.parse::<usize>().unwrap(),
                })
            };

            current_directory.entries.push(entry);
        }
    }

    root
}

fn find_directory<'a>(
    directory: &'a mut Directory,
    path: &String,
) -> Option<&'a mut Directory> {
    directory.entries.iter_mut().find_map(|entry| match entry {
        Entry::Directory(child_directory) => {
            if child_directory.path() == *path {
                Some(child_directory)
            } else {
                find_directory(child_directory, path)
            }
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(input()),
            Directory {
                name: "/".into(),
                parents: vec![],
                entries: vec![
                    Entry::Directory(Directory {
                        name: "a".into(),
                        parents: vec![],
                        entries: vec![
                            Entry::Directory(Directory {
                                name: "e".into(),
                                parents: vec!["a".into()],
                                entries: vec![Entry::File(File {
                                    name: "i".into(),
                                    size: 584
                                }),]
                            }),
                            Entry::File(File {
                                name: "f".into(),
                                size: 29116
                            }),
                            Entry::File(File {
                                name: "g".into(),
                                size: 2557
                            }),
                            Entry::File(File {
                                name: "h.lst".into(),
                                size: 62596
                            }),
                        ]
                    }),
                    Entry::File(File {
                        name: "b.txt".into(),
                        size: 14848514
                    }),
                    Entry::File(File {
                        name: "c.dat".into(),
                        size: 8504156
                    }),
                    Entry::Directory(Directory {
                        name: "d".into(),
                        parents: vec![],
                        entries: vec![
                            Entry::File(File {
                                name: "j".into(),
                                size: 4060174
                            }),
                            Entry::File(File {
                                name: "d.log".into(),
                                size: 8033020
                            }),
                            Entry::File(File {
                                name: "d.ext".into(),
                                size: 5626152
                            }),
                            Entry::File(File {
                                name: "k".into(),
                                size: 7214296
                            }),
                        ]
                    }),
                ]
            }
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 95437)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 24933642)
    }
}
