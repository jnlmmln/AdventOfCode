use std::{cell::RefCell, fs, rc::Rc};

struct Dir {
    #[allow(dead_code)]
    name: String,
    parent: Option<Rc<RefCell<Dir>>>,
    files: Vec<File>,
    dirs: Vec<Rc<RefCell<Dir>>>,
}

impl Dir {
    pub fn new(name: &str) -> Rc<RefCell<Dir>> {
        Rc::new(RefCell::new(Self {
            name: String::from(name),
            parent: None,
            files: vec![],
            dirs: vec![],
        }))
    }
    pub fn add_dir(
        &mut self,
        parent: Rc<RefCell<Dir>>,
        dir: Rc<RefCell<Dir>>,
    ) -> Rc<RefCell<Self>> {
        self.dirs.push(Rc::clone(&dir));
        dir.borrow_mut().parent = Some(Rc::clone(&parent));
        Rc::clone(self.dirs.last().unwrap())
    }
    pub fn add_file(&mut self, name: &str, size: &usize) {
        self.files.push(File {
            name: name.to_string(),
            size: *size,
        })
    }
}

struct File {
    #[allow(dead_code)]
    name: String,
    size: usize,
}

pub fn run() {
    println!("## DAY 7");

    part_1();

    println!("\n");
}

fn part_1() {
    let content = fs::read_to_string("input-day-7");
    if content.is_err() {
        eprintln!("{}", content.unwrap_err());
        return;
    }

    let content = content.unwrap();

    let fs = Dir::new("/");
    let mut current_dir = Rc::clone(&fs);

    for line in content.lines() {
        if line.starts_with("$ cd") {
            let to_dir = line.split_once("$ cd ").unwrap().1;
            match to_dir {
                "/" => current_dir = Rc::clone(&fs),
                ".." => {
                    let cur = Rc::clone(&current_dir);
                    if let Some(parent) = &cur.borrow_mut().parent {
                        current_dir = Rc::clone(&parent);
                    };
                }
                dir => {
                    let new_dir = current_dir
                        .borrow_mut()
                        .add_dir(Rc::clone(&current_dir), Dir::new(dir));
                    current_dir = Rc::clone(&new_dir);
                }
            }
        } else if line.starts_with("$ ls") {
        } else {
            match line.split_once(" ") {
                Some(("dir", _name)) => (),
                Some((size, name)) => current_dir
                    .borrow_mut()
                    .add_file(name, &usize::from_str_radix(size, 10).unwrap()),
                _ => (),
            }
        }
    }

    let part1_total = iter_over_fs(&fs.borrow_mut())
        .iter()
        .fold(0, |sum, size| sum + if *size <= 100000 { *size } else { 0 });
    println!("### Part 1");
    println!("\tResult: {}", part1_total);

    let used_space = get_size_of(&fs.borrow_mut());
    let free_space = 70000000 - used_space;
    let required_to_free = 30000000 - free_space;

    let solution_part2 = iter_over_fs(&fs.borrow_mut())
        .into_iter()
        .filter(|x| *x > required_to_free)
        .min();

    println!("### Part 2");
    println!("\tResult: {}", solution_part2.unwrap());
}

fn iter_over_fs(dir: &Dir) -> Vec<usize> {
    let mut result = vec![];

    dir.dirs.iter().for_each(|d| {
        result.push(get_size_of(&d.borrow_mut()));
        if d.borrow_mut().dirs.len() > 0 {
            result.append(&mut iter_over_fs(&d.borrow_mut()));
        }
    });

    result
}

fn get_size_of(dir: &Dir) -> usize {
    let total = dir.files.iter().fold(0, |sum, f| sum + f.size)
        + dir
            .dirs
            .iter()
            .fold(0, |sum, d| sum + get_size_of(&d.borrow_mut()));

    total
}
