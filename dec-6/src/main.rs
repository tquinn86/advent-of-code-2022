use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));
    
    //this one is different. Instead of looping through lines, we're just dealing with a single
    //long string.

    println!("{}", contents);

    let chars : Vec<char> = contents.chars().collect();
    'char_iter: for i in 0..chars.len() {
        let mut temp = String::new();
        //Part 1
        //for j in 0..4 {
        //Part 2 looks for 14 chars
        for j in 0..14 {
            let c = chars[i + j];
            if ! temp.contains(c) {
                temp.push(c);
            } else {
                continue 'char_iter;
            }
        }
        //Part 1
        //println!( "First non-repeating 4 char block ends at {}", i + 4);
        //Part 2
        println!( "First non-repeating 14 char block ends at {}", i + 14);
        break;
    }
}