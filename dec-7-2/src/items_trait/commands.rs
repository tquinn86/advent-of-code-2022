use crate::items_trait::*;
use std::rc::Rc;

pub fn parse_commands(contents: &String) -> Rc<dyn Item> {
    
    let lines: Vec<&str> = contents.lines().collect();
    let mut cur_dir: Rc<dyn Item>;
    if lines[0] == "$ cd /" {
        cur_dir = Rc::new(Dir::new("/".to_string(), None));
    } else {
        panic!("First line is not root!")
    }

    //keep another reference around to return
    let root: Rc<dyn Item> = cur_dir.clone();
    for line in &lines[1..] {
        if line.starts_with("$") {
            cur_dir = parse_command(line, &cur_dir);
        } else {
            parse_item(line, &cur_dir);
        }
    }
    root
}

fn parse_command(line: &str, cur_dir: &Rc<dyn Item>) -> Rc<dyn Item> {
    println!("in parse command: {}", line);

        if !line.starts_with("$") {
            panic!("Not a command");
        }

        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts[1] == "cd" {
            return parse_cd(parts[2], cur_dir);
        } else if parts[1] == "ls" {
            //do nothing
            println!("Starting ls.");
            return cur_dir.clone();
        } else {
            panic!("what?")
        }
}

fn parse_cd(dir_name: &str, cur_dir: &Rc<dyn Item>) -> Rc<dyn Item> {
    println!("In parse_cd");
    if dir_name == ".." {
        println!("popping up");
        return cur_dir.as_any().downcast_ref::<Dir>().unwrap().parent.borrow().upgrade().unwrap();
    } else {

        //it should be a subdir already
        let opt_dir = cur_dir.as_any().downcast_ref::<Dir>().unwrap().find_sub_dir(dir_name);

        if opt_dir.is_some() {
            println!("Found dir: {}", dir_name);
            opt_dir.unwrap()
        } else {
            println!("Dir {} not found!", dir_name);
            panic!("Should have found it.")
        }
    }
}


fn parse_item(line: &str, cur_dir: &Rc<dyn Item>) {
    println!("{}", line);
    
    let parts = line.split(" ").collect::<Vec<&str>>();
    let new_item: Rc<dyn Item>;
    if parts[0] == "dir" {
        new_item = Rc::new(Dir::new(parts[1].to_string(), Some(Rc::downgrade(cur_dir))));
    } else {
        let size = parts[0].parse::<usize>().unwrap();
        new_item = Rc::new(File::new(parts[1].to_string(), size, Rc::downgrade(cur_dir)));
    }

    cur_dir.as_any().downcast_ref::<Dir>().unwrap().add_child(&new_item);
}