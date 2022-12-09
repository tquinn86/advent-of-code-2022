
use crate::items::*;

impl ItemTree {
    pub fn parse_commands(&mut self, contents: &String) -> Option<&Item> {
        // keep a stack of dirs in a vec as we move
        // through
        let mut dir_stack: Vec<usize> = vec![];
        for line in contents.lines() {
            if line.starts_with("$") {
                self.parse_command(line, &mut dir_stack)
            } else {
                self.parse_item(line, &dir_stack);
            }
        }
        self.get_at(0)
    }

    fn parse_command(&mut self, line: &str, dir_stack: &mut Vec<usize>) {
        println!("in parse command: {}", line);

        if !line.starts_with("$") {
            panic!("Not a command");
        }

        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts[1] == "cd" {
            self.parse_cd(parts[2], dir_stack);
            return;
        } else if parts[1] == "ls" {
            //do nothing
            println!("Starting ls.");
            return;
        } else {
            panic!("what?")
        }
    }

    fn parse_cd(&mut self, dir_name: &str, dir_stack: &mut Vec<usize>) {
        println!("In parse_cd");
        if dir_name == ".." {
            println!("popping up");
            dir_stack.pop();
            Self::print_current_id(dir_stack);
        } else {

            //see if the thing is already in there
            //name can be repeated, so name _and_ parent_id must match
            let parent_id = Self::get_parent_id(dir_stack);
            let dir_id = self.items.iter().position(|i| {
                match i {
                    Item::Dir(dt) => dt.name == dir_name && dt.parent == parent_id,
                    Item::File(_) => false
                }
            });

            match dir_id {
                Some(id) => {
                    dir_stack.push(id);
                    println!("Found dir {}", dir_name);
                    Self::print_current_id(dir_stack);
                },
                None => {
                    println!("Creating new dir {}", dir_name);
                    match parent_id {
                        Some(id) => {
                            let parent: &mut Item = self.get_at_mut(id).unwrap();
                            parent.add_child(id);
                        },
                        _ => ()
                    }
                    let new_id = self.push_item(Item::Dir(DirItem::new(dir_name.to_string(), parent_id)), parent_id);
                    dir_stack.push(new_id);
                    Self::print_current_id(dir_stack);
                }
            }
        }
    }

    fn parse_item(&mut self, line: &str, dir_stack: &Vec<usize>) {
        let parent_id = Self::get_parent_id(dir_stack);
        Self::print_current_id(dir_stack);

        println!("{}", line);
        let parts = line.split(" ").collect::<Vec<&str>>();
        if parts[0] == "dir" {
            self.push_item(Item::Dir(DirItem::new(parts[1].to_string(), parent_id)), parent_id);
        } else {
            let size = parts[0].parse::<usize>().unwrap();
            self.push_item(Item::File(FileItem::new(parts[1].to_string(), size)), parent_id);
        }

    }

    fn get_parent_id(dir_stack: &Vec<usize>) -> Option<usize> {
        if dir_stack.len() > 0 { 
            Some(dir_stack[dir_stack.len() - 1])
        } else {
            None
        }
    }

    fn print_current_id(dir_stack: &Vec<usize>) {
        let parent_id = Self::get_parent_id(dir_stack);
        match parent_id {
            Some(id) => println!("Current directory: {}", id ),
            None => println!("No current directory, dir_stack empty")
        }
    }
}