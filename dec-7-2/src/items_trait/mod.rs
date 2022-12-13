use std::fmt;
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::any::Any;

pub mod commands;
pub mod problem;

pub trait Item: Display {
    fn name(&self) -> &str;
    fn size(&self) -> usize;
    fn is_dir(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub struct Dir {
    name: String,
    parent: RefCell<Weak<dyn Item>>,
    items: RefCell<Vec<Rc<dyn Item>>>
}

pub struct File {
    name: String,
    parent: RefCell<Weak<dyn Item>>,
    size: usize
}

impl Dir {
    pub fn new(name: String, parent: Option<Weak<dyn Item>>) -> Self {
        if parent.is_some() {
            Dir {
                name,
                parent: RefCell::new(parent.unwrap()),
                items: RefCell::new(vec![])
            }
        } else {
            Dir {
                name,
                parent: RefCell::new(Weak::<Dir>::new()),
                items: RefCell::new(vec![])
            }
        }
    }

    fn find_sub_dir(&self, dir_name: &str) -> Option<Rc<dyn Item>> {
    
        let idx = self.items.borrow().iter().position(|x| x.is_dir() && x.name() == dir_name);
        match idx {
            Some(i) => {
                Some(self.items.borrow()[i].clone())
            },
            _ => None
        }
    }

    fn add_child(self: &Dir, item: &Rc<dyn Item>) {
        self.items.borrow_mut().push(item.clone());
    }

    //associated method so I can have &Rc<dyn Item> as the param
    pub fn flatten_dirs(tree: &Rc<dyn Item>) -> Box<Vec<Rc<dyn Item>>> {

        let mut ret: Box<Vec<Rc<dyn Item>>> = Box::new(vec![]);

        if !(*tree).is_dir() { panic!("Unexpected call on non dir."); }

        //add the current dir to the vec
        (*ret).push(tree.clone());

        for item in &*tree.as_any().downcast_ref::<Dir>().unwrap().items.borrow() {
            if item.is_dir() {
                let subdirs = Dir::flatten_dirs(&item);
                for s in &*subdirs {
                    ret.push(s.clone());
                }
            }
        }
        ret
    }
}

impl Item for Dir {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        let mut total: usize = 0;
        for i in &*self.items.borrow() {
            total += i.size()
        }
        total
    }

    fn is_dir(&self) -> bool { true }

    fn as_any(&self) -> &dyn Any { self }
}

impl Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "{} (dir)\n", self.name);
        if res.is_err() { return res; }

        for i in &*self.items.borrow() {
            res = write!(f, "{}", i);
            if res.is_err() { break; }
        }
        res
    }
}

impl File {
    fn new(name: String, size: usize, parent: Weak<dyn Item>) -> Self {
        Self {
            name,
            parent: RefCell::new(parent),
            size
        }
    }
}

impl Item for File {
    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_dir(&self) -> bool { false }

    fn as_any(&self) -> &dyn Any { self }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, (file, size={})\n", self.name, self.size) 
    }
}