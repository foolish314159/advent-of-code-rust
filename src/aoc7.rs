use std::{
    cell::RefCell,
    fs::read_to_string,
    rc::{Rc, Weak},
};

use regex::Regex;

pub fn size_of_dirs_below(filename: &str, below: usize) -> usize {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    let fs = FileSystem::parse_from_terminal_output(&content.to_string());

    // fs.size_infos()
    //     .iter()
    //     .filter(|size| size < &&below)
    //     .fold(0, |sum, size| sum + size)

    // fs.tree();

    fs.filter_dirs(|f| File::total_size(f) < below)
        .iter()
        .fold(0, |sum, f| sum + File::total_size(f))
}

pub fn dir_to_delete(filename: &str, total_size: usize, required_size: usize) -> usize {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    let fs = FileSystem::parse_from_terminal_output(&content.to_string());

    let remaining = total_size - File::total_size(&fs.root);
    let min_size = required_size - remaining;

    let dirs = fs.filter_dirs(|f| File::total_size(f) >= min_size);
    let dir = dirs
        .iter()
        .min_by(|l, r| File::total_size(*l).cmp(&File::total_size(*r)))
        .unwrap();

    File::total_size(dir)
}

struct File {
    name: String,
    size: usize,
    is_dir: bool,
    files: Vec<Rc<RefCell<File>>>,
    parent: Option<Weak<RefCell<File>>>,
}

impl File {
    fn new(name: &str, parent: &Option<Weak<RefCell<File>>>) -> File {
        File {
            name: name.to_string(),
            size: 0,
            is_dir: true,
            files: vec![],
            parent: parent.clone(),
        }
    }

    fn info_str(&self) -> String {
        if self.is_dir {
            format!("{} (dir)", self.name)
        } else {
            format!("{} (file, size={})", self.name, self.size)
        }
    }

    fn total_size(file: &Rc<RefCell<File>>) -> usize {
        if !file.borrow().is_dir {
            return file.borrow().size;
        }

        File::total_size_impl(file, 0)
    }

    fn total_size_impl(current: &Rc<RefCell<File>>, current_size: usize) -> usize {
        if !current.borrow().is_dir {
            return current.borrow().size;
        }

        let mut size = current_size;
        for f in &current.borrow().files {
            size += File::total_size_impl(f, current_size);
        }

        size
    }
}

struct FileSystem {
    root: Rc<RefCell<File>>,
    cwd: Weak<RefCell<File>>,
}

impl FileSystem {
    fn new(root_path: &str) -> FileSystem {
        let root = Rc::new(RefCell::new(File::new(root_path, &None)));
        let w_root = Rc::downgrade(&root);
        FileSystem {
            root: root,
            cwd: w_root,
        }
    }

    fn parse_from_terminal_output(str: &String) -> FileSystem {
        let mut fs = FileSystem::new("/");

        // Assume root is always "/" and first line of input always is "cd /"
        for line in str.lines().skip(1) {
            fs.parse_terminal_line(line)
        }

        fs
    }

    fn parse_terminal_line(&mut self, line: &str) {
        let re_ls_dir = Regex::new(r"^dir ([a-zA-Z]+)$").unwrap();
        let re_ls_file = Regex::new(r"^(\d+) ([^\s]+)$").unwrap();
        let re_cmd_cd = Regex::new(r"^\$ cd ([^\s]+)$").unwrap();
        let re_cmd_ls = Regex::new(r"^\$ ls$").unwrap();

        match line {
            line if re_ls_dir.is_match(line) => {
                self.ls_file(
                    &re_ls_dir.captures(line).unwrap().get(1).unwrap().as_str(),
                    true,
                    0,
                );
            }
            line if re_ls_file.is_match(line) => {
                let cap = re_ls_file.captures(line).unwrap();
                self.ls_file(
                    cap.get(2).unwrap().as_str(),
                    false,
                    cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                );
            }
            line if re_cmd_cd.is_match(line) => {
                self.cd(&re_cmd_cd.captures(line).unwrap().get(1).unwrap().as_str());
            }
            line if re_cmd_ls.is_match(line) => {
                // Since ls is the only command that produces output we don't need to remember it
            }
            _ => panic!("Invalid terminal line {}", line),
        };
    }

    fn tree_impl(&self, current: &Rc<RefCell<File>>, layer: usize) {
        for f in &current.borrow().files {
            println!(
                "{}- {} [{}]",
                "  ".repeat(layer),
                f.borrow().info_str(),
                File::total_size(f)
            );
            if f.borrow().is_dir {
                self.tree_impl(&f, layer + 1);
            }
        }
    }

    fn tree(&self) {
        println!(
            "- {}  [{}]",
            self.root.borrow().info_str(),
            File::total_size(&self.root)
        );
        self.tree_impl(&self.root, 1);
    }

    fn filter_dirs_impl<P>(
        &self,
        current: &Rc<RefCell<File>>,
        res: &mut Vec<Rc<RefCell<File>>>,
        predicate: &P,
    ) where
        Self: Sized,
        P: Fn(&Rc<RefCell<File>>) -> bool,
    {
        for f in &current.borrow().files {
            if f.borrow().is_dir {
                if predicate(f) {
                    res.push(f.clone());
                }

                self.filter_dirs_impl(f, res, predicate);
            }
        }
    }

    fn filter_dirs<P>(&self, predicate: P) -> Vec<Rc<RefCell<File>>>
    where
        Self: Sized,
        P: Fn(&Rc<RefCell<File>>) -> bool,
    {
        let mut dirs = vec![];
        self.filter_dirs_impl(&self.root, &mut dirs, &predicate);
        dirs
    }

    fn size_infos_impl(&self, current: &Rc<RefCell<File>>, res: &mut Vec<usize>) {
        for f in &current.borrow().files {
            if f.borrow().is_dir {
                res.push(File::total_size(f));
                self.size_infos_impl(f, res);
            }
        }
    }

    fn size_infos(&self) -> Vec<usize> {
        let mut size_infos = vec![];
        self.size_infos_impl(&self.root, &mut size_infos);
        size_infos
    }

    fn ls_file(&mut self, name: &str, is_dir: bool, size: usize) {
        if self.exists(name).is_none() {
            self.touch(name, is_dir, size);
        }
    }

    fn exists(&self, name: &str) -> Option<Rc<RefCell<File>>> {
        if let Some(f) = self
            .cwd
            .upgrade()
            .unwrap()
            .borrow()
            .files
            .iter()
            .find(|f| f.borrow().name == *name)
        {
            Some(f.clone())
        } else {
            None
        }
    }

    fn touch(&mut self, name: &str, is_dir: bool, size: usize) -> Weak<RefCell<File>> {
        let f = Rc::new(RefCell::new(File::new(name, &Some(self.cwd.clone()))));
        f.borrow_mut().is_dir = is_dir;
        f.borrow_mut().size = size;
        let wf = Rc::downgrade(&f);
        self.cwd.upgrade().unwrap().borrow_mut().files.push(f);
        wf
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            if let Some(parent) = &self.cwd.upgrade().unwrap().borrow().parent {
                self.cwd = parent.clone();
                return;
            }
        }

        if let Some(f) = self.exists(dir) {
            self.cwd = Rc::downgrade(&f);
            return;
        }

        self.cwd = self.touch(dir, true, 0);
    }
}
