use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

struct Directory {
    name: String,
    directories: HashMap<String, DirRef>,
    files: HashMap<String, u32>,
    parent: Option<DirRef>,
    size: u32,
}

struct FileSystem {
    root: DirRef,
    current: DirRef,
}

type DirRef = Rc<RefCell<Directory>>;

impl Directory {
    fn new(name: String) -> DirRef {
        Rc::new(RefCell::new(Directory {
            name,
            directories: HashMap::new(),
            files: HashMap::new(),
            parent: None,
            size: 0,
        }))
    }

    fn tree(&self, offset: usize) {
        println!("{}- {} (dir)", " ".repeat(offset), self.name);
        for child in self.directories.values() {
            child.borrow().tree(offset + 2);
        }
        for (name, size) in &self.files {
            println!("{}- {name} (file, {size})", " ".repeat(offset + 2))
        }
    }

    fn compute_size(&mut self) -> u32 {
        let mut dir_size: u32 = 0;
        for child in self.directories.values() {
            dir_size += child.borrow_mut().compute_size();
        }
        for (_, size) in &self.files {
            dir_size += size;
        }

        self.size = dir_size;
        dir_size
    }
}

impl FileSystem {
    fn cd(&mut self, dir: &str) {
        let new_dir = {
            if dir == ".." {
                let current = self.current.borrow();
                current.parent.as_ref().unwrap().clone()
            } else {
                let exists = {
                    let current = self.current.borrow();
                    current.directories.get(dir).is_some()
                };
                if !exists {
                    self.mkdir(dir);
                }

                Rc::clone(self.current.borrow().directories.get(dir).unwrap())
            }
        };

        self.current = new_dir;
    }

    fn mkdir(&self, dir: &str) {
        if !self.current.borrow().directories.contains_key(dir) {
            let new_dir = Directory::new(dir.to_string());
            new_dir.borrow_mut().parent = Some(Rc::clone(&self.current));
            self.current
                .borrow_mut()
                .directories
                .insert(dir.to_string(), new_dir);
        }
    }

    fn mkfile(&self, file: &str, size: u32) {
        self.current
            .borrow_mut()
            .files
            .insert(file.to_string(), size);
    }

    fn compute_sizes(&self) {
        self.root.borrow_mut().compute_size();
    }

    fn get_dirs(&self) -> Vec<DirRef> {
        let mut stack: Vec<DirRef> = Vec::new();
        let mut result: Vec<DirRef> = Vec::new();

        stack.push(self.root.clone());

        while stack.len() != 0 {
            let top = { stack.pop().unwrap().clone() };
            for child in top.borrow().directories.values() {
                stack.push(child.clone());
            }
            result.push(top);
        }

        result
    }

    fn tree(&self) {
        self.root.borrow().tree(0);
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let fs = build_fs(&lines);
    fs.tree();
    fs.compute_sizes();

    let result: u32 = fs
        .get_dirs()
        .iter()
        .filter(|d| d.borrow().size <= 100000)
        .fold(0, |acc, d| acc + d.borrow().size);

    println!("part 1: {result}")
}

fn build_fs(lines: &Vec<String>) -> FileSystem {
    let root = Directory::new("/".to_string());
    let mut fs = FileSystem {
        root: Rc::clone(&root),
        current: Rc::clone(&root),
    };

    for line in lines[1..].iter() {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd.as_slice() {
            ["$", "cd", dir] => {
                fs.cd(dir);
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, file] => {
                let size: u32 = size.parse().unwrap();
                fs.mkfile(file, size);
            }
            c => {
                println!("unrecognized command: {:?}", c)
            }
        }
    }

    fs
}
