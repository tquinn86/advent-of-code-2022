use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path)
                    .expect(&format!("Cannot find file {}", file_path));

    //each row is a vec of numbers, the grid is a vec of vecs
    let mut grid: Vec<Vec<u32>> = vec![];
    const RADIX: u32 = 10;

    for line in contents.lines() {
        let mut row: Vec<u32> = vec![];
        for c in line.chars() {
            row.push(c.to_digit(RADIX).unwrap());
        }
        grid.push(row);
    }

    print_grid(&grid);

    //need to make a matching grid of 1 and 0 to mark visibility
    //we only count things as visible ones.
    //so a grid of the same dimensions. Edges are intialized to 1,
    //interiors, initialized to 0
    let mut visibility_grid: Vec<Vec<u32>> = vec![];

    for i in 0..grid.len() {
        let mut row: Vec<u32>;
        if  i == 0 || i == grid.len() - 1 {
            row = vec![1; grid[0].len()];
        } else {
            row = vec![0; grid[0].len()];
            row[0] = 1;
            let lastidx = row.len() - 1;
            row[lastidx] = 1;
        }
        visibility_grid.push(row);
    }

    print_grid(&visibility_grid);


    visible_from_left(&grid, &mut visibility_grid);
    visible_from_top(&grid, &mut visibility_grid);
    visible_from_right(&grid, &mut visibility_grid);
    visible_from_bottom(&grid, &mut visibility_grid);

    print_grid(&visibility_grid);

    let visible: u32 = visibility_grid.iter().flatten().map(|x| x).sum();
    println!("Total visible trees: {}", visible);
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for v in grid {
        for n in v {
            print!("{}", n);
        }
        println!();
    }
    println!()
}

fn visible_from_left(grid: &Vec<Vec<u32>>, visibility_grid: &mut Vec<Vec<u32>>) {
    // easiest direction
    // loop through the rows then
    // keep track of the tallest we've seen. if the current
    // is taller than last tallest, toggle the bit, and update tallest
    for i in 1..grid.len() -1 {
        let mut cur_tallest: u32 = grid[i][0];
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] > cur_tallest { 
                visibility_grid[i][j] = 1;
                cur_tallest = grid[i][j];
            }
        }
    }
}

fn visible_from_top(grid: &Vec<Vec<u32>>, visibility_grid: &mut Vec<Vec<u32>>) {
    //we reverse the loops here, iterate over
    //the indices of each row.
    //keep track of tallest as above
    for i in 1..grid[0].len() - 1 {
        let mut cur_tallest: u32 = grid[0][i];
        for j in 1..grid.len() - 1 {
            //loop continues if equal
            if grid[j][i] > cur_tallest { 
                visibility_grid[j][i] = 1;
                cur_tallest = grid[j][i]; 
            }
        }
    }
}

fn visible_from_right(grid: &Vec<Vec<u32>>, visibility_grid: &mut Vec<Vec<u32>>) {
    //same as the first one but backwards
    for i in (1..grid.len() -1).rev() {
        let mut cur_tallest: u32 = grid[i][grid[0].len() - 1];
        for j in (1..grid[0].len() - 1).rev() {
            if grid[i][j] > cur_tallest { 
                visibility_grid[i][j] = 1;
                cur_tallest = grid[i][j];
            }
        }
    }
}

fn visible_from_bottom(grid: &Vec<Vec<u32>>, visibility_grid: &mut Vec<Vec<u32>>) {
    //same as the second one but backwards
    //accumulate while < next row same index
    for i in (1..grid[0].len() - 1).rev() {
        let mut cur_tallest: u32 = grid[grid.len() - 1][i];
        for j in (1..grid.len() - 1).rev() {
            //loop continues if equal
            if grid[j][i] > cur_tallest { 
                visibility_grid[j][i] = 1;
                cur_tallest = grid[j][i];
            }
        }
    }
}
