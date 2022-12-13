use std::fmt;
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod commands;
pub mod problem;

pub enum Item {
    File(FileItem),
    Dir(DirItem)
}

pub struct FileItem {
    name: String,
    parent: RefCell<Weak<Item>>,
    size: usize
}

pub struct DirItem {
    name: String,
    parent: RefCell<Weak<Item>>,
    items: RefCell<Vec<Rc<Item>>>
}

impl Item {
    pub fn add_child(self: &Rc<Item>, item: &Rc<Item>) {
        match &**self {
            Item::File(_) => panic!("Can't add child to File Item!"),
            Item::Dir(dir) => {
                item.set_parent(Rc::downgrade(&self));
                dir.items.borrow_mut().push(item.clone());
            }
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Item::Dir(_) => true,
            _  => false
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Item::Dir(di) => &di.name,
            Item::File(fi) => &fi.name
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Item::Dir(di) => di.size(),
            Item::File(fi) => fi.size
        }
    }

    pub fn flatten_dirs(self: &Rc<Item>) -> Box<Vec<Rc<Item>>> {

        let mut ret: Box<Vec<Rc<Item>>> = Box::new(vec![]);

        if !(*self).is_dir() { panic!("Unexpected call on non dir."); }

        //add the current dir to the vec
        (*ret).push(self.clone());

        if let Item::Dir(di) = &**self {
            for item in &*di.items.borrow() {
                if item.is_dir() {
                    let subdirs = *item.flatten_dirs();
                    for s in &subdirs {
                        ret.push(s.clone());
                    }
                }
            }
        }
        ret
    }

    fn set_parent(&self, parent: Weak<Item>) {
        match self {
            Item::Dir(di) => *di.parent.borrow_mut() = parent,
            Item::File(fi) => *fi.parent.borrow_mut() = parent
        }
    }

    fn find_sub_dir(&self, dir_name: &str) -> Option<Rc<Item>> {
        match self {
            Item::Dir(di) => {
                let idx = di.items.borrow().iter().position(|x| {
                    match &**x {
                        Item::Dir(dt) => dt.name == dir_name,
                        _ => false
                    }
                });
                match idx {
                    Some(i) => {
                        match self {
                            Item::Dir(di) => {
                                Some(di.items.borrow()[i].clone())
                            },
                            _ => None
                        }
                    },
                    _ => None
                }
            }
            _ => None
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::File(file) => file.fmt(f),
            Item::Dir(d) => d.fmt(f)
        }
    }
}

impl FileItem {
    pub fn new(name: String, size: usize) -> Self {
        Self {
            name,
            parent: RefCell::new(Weak::new()),
            size
        }
    }
}

impl Display for FileItem{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, (file, size={})\n", self.name, self.size) 
    }
}

impl DirItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parent: RefCell::new(Weak::new()),
            items: RefCell::new(vec![])
        }
    }

    fn size(&self) -> usize {
        let mut total: usize = 0;
        for i in &*self.items.borrow() {
            total += i.size()
        }
        total
    }
}

impl Display for DirItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "{} (dir)\n", self.name);
        if res.is_err() { return res; }

        for i in &*self.items.borrow() {
            res = i.fmt(f);
            if res.is_err() { break; }
        }
        res
    }
}
