use std::fmt;
use std::fmt::Display;

pub mod commands;

pub struct ItemTree {
    items: Vec<Item>
}

pub enum Item {
    File(FileItem),
    Dir(DirItem)
}

pub struct FileItem {
    name: String,
    size: usize
}

pub struct DirItem {
    name: String,
    parent: Option<usize>,
    items: Vec<usize>
}

impl ItemTree {
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }

    pub fn push_item(&mut self, item: Item, parent_id: Option<usize>) -> usize {
        let idx = self.items.len();
        if parent_id.is_some() {
            let parent = &mut self.items[parent_id.unwrap()];
            parent.add_child(idx);
        }
        self.items.push(item);
        idx
    }

    pub fn get_at(&self, idx: usize) -> Option<&Item> {
        if idx < self.items.len() {
            Some(&self.items[idx])
        } else {
            None
        }
    }

    fn get_at_mut(&mut self, idx: usize) -> Option<&mut Item> {
        if idx < self.items.len() {
            Some(&mut self.items[idx])
        } else {
            None
        }
    }

    pub fn get_item_size(&self, idx: usize) -> Option<u64> {
        if idx < self.items.len() {
            let item = &self.items[idx];
            let size = match item {
                Item::Dir(di) => self.get_dir_item_size(&di),
                Item::File(fi) => fi.size as u64
            };
            Some(size)
        } else {
            None
        }
    }

    fn get_dir_item_size(&self, item: &DirItem) -> u64
    {
        let mut total: u64 = 0;
        for idx in &item.items {
            let item = self.get_at(*idx).unwrap();
            total += match &*item {
                Item::Dir(di) => self.get_dir_item_size(&di) as u64,
                Item::File(fi) => fi.size as u64
            }
        }
        total
    }

    fn fmt_dir_item(&self, item: &DirItem, indent: i32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..indent {
            let res = write!(f, "-");
            if res.is_err() { return res; }
        }
        let mut res = item.fmt(f);
        if res.is_err() { return res; }

        for idx in &item.items {
            let item = self.get_at(*idx).unwrap();
            res = match &*item {
                Item::Dir(di) => self.fmt_dir_item(&di, indent + 1, f),
                Item::File(fi) => self.fmt_file_item(&fi, indent + 1, f)
            }
        }
        res
    }

    fn fmt_file_item(&self, item: &FileItem, indent: i32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..indent {
            let res = write!(f, "-");
            if res.is_err() { return res; }
        }
        item.fmt(f)
    }
}

impl fmt::Display for ItemTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = 0;
        let root = self.get_at(0).unwrap();

        let res = match root {
            Item::Dir(di) => self.fmt_dir_item(&di, indent, f),
            Item::File(fi) => self.fmt_file_item(&fi, indent, f)
        };
        res
    }
}

impl Item {
    pub fn add_child(&mut self, item_id: usize) {
        match self {
            Item::File(_) => panic!("Can't add child to File Item!"),
            Item::Dir(ref mut dir) => dir.items.push(item_id)
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
}

impl fmt::Display for Item {
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
            size
        }
    }
}

impl fmt::Display for FileItem{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, (file, size={})\n", self.name, self.size) 
    }
}

impl DirItem {
    pub fn new(name: String, parent: Option<usize>) -> Self {
        Self {
            name,
            parent,
            items: vec![]
        }
    }
}

impl fmt::Display for DirItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (dir)\n", self.name)
    }
}