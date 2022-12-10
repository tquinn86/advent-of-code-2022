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

    //Part two we need a visability grid as well, but
    //now we look out from a given tree and stop counting in each direction
    //when something is of equal height or taller, or hit the edge.
    //things on the edge have a value of zero because at least one direction
    //there is nothing there.

    let mut visibility_grid: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];

    print_grid(&visibility_grid);

    //just loop through the entire grid and
    //and calculate the score for each direction.
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            visibility_grid[i][j] = get_visibility_score(&grid, Position {x: j, y: i});
        }
    }

    print_grid(&visibility_grid);

    //find the highest number:
    let mut highest = visibility_grid.iter().flatten().map(|x| *x).collect::<Vec<u32>>();
    highest.sort();
    println!("{}", highest[highest.len() - 1]);
}

#[derive(Copy, Clone)]
struct Position{
    x: usize,
    y: usize
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

fn get_visibility_score(grid: &Vec<Vec<u32>>, pos: Position)  -> u32 {

    let mut score = get_up_score(&grid, pos);
    score *= get_right_score(&grid, pos);
    score *= get_down_score(&grid, pos);
    score *= get_left_score(&grid, pos);

    score
}

fn get_up_score(grid: &Vec<Vec<u32>>, pos: Position)  -> u32 {
    let cur_height = grid[pos.y][pos.x];

    //we're looking up, so if y is zero we are on the edge and 
    //the score is 0.
    if pos.y == 0 {
        return 0;
    }

    //otherwise we look up along this column and add up until we
    //run into something the same height or taller.
    let mut score = 0;
    for i in (0..pos.y).rev() {
        if grid[i][pos.x] < cur_height {
            score += 1;
        }
        else {
            //hit the wall
            //add one more and break
            score += 1;
            break;
        }
    }
    score
}

fn get_right_score(grid: &Vec<Vec<u32>>, pos: Position)  -> u32 {
    let cur_height = grid[pos.y][pos.x];
    //println!("Looking at position {{ x:{} y:{} }} with height {}", pos.x, pos.y, grid[pos.y][pos.x]);

    //we're looking right, so if x is equal to max x we are on the edge and 
    //the score is 0.
    if pos.x == grid[pos.x].len() - 1 {
        return 0;
    }

    //otherwise we look right along this row and add up until we
    //run into something the same height or taller.
    let mut score = 0;
    for i in (pos.x + 1)..grid[pos.x].len() {
        if grid[pos.y][i] < cur_height {
            score += 1;
        }
        else {
            //hit the wall
            //add one more and break
            score += 1;
            break;
        }
    }
    score
}

fn get_down_score(grid: &Vec<Vec<u32>>, pos: Position)  -> u32 {
    let cur_height = grid[pos.y][pos.x];

    //we're looking down, so if y is max y we are on the edge and 
    //the score is 0.
    if pos.y == grid.len() - 1 {
        return 0;
    }

    //otherwise we look down along this column and add up until we
    //run into something the same height or taller.
    let mut score = 0;
    for i in (pos.y + 1)..grid.len() {
        if grid[i][pos.x] < cur_height {
            score += 1;
        }
        else {
            //hit the wall
            //add one more and break
            score += 1;
            break;
        }
    }
    score
}

fn get_left_score(grid: &Vec<Vec<u32>>, pos: Position)  -> u32 {
    let cur_height = grid[pos.y][pos.x];
    //println!("Looking at position {{ x:{} y:{} }} with height {}", pos.x, pos.y, grid[pos.y][pos.x]);

    //we're looking left, so if x is equal to 0 we are on the edge and 
    //the score is 0.
    if pos.x == 0 {
        return 0;
    }

    //otherwise we look left along this row and add up until we
    //run into something the same height or taller.
    let mut score = 0;
    for i in (0..pos.x).rev() {
        if grid[pos.y][i] < cur_height {
            score += 1;
        }
        else {
            //hit the wall
            //add one more and break
            score += 1;
            break;
        }
    }
    score
}


    //Everything from here on down is Part 1,
    //commenting out and pushing down. Look at tag 'dec-8-1' to see where it belongs...
    //Tomorrow I'll try to 
    //remember to use modules to separate the two halve of the problem...

    //need to make a matching grid of 1 and 0 to mark visibility
    //we only count things as visible ones.
    //so a grid of the same dimensions. Edges are intialized to 1,
    //interiors, initialized to 0
    /* let mut visibility_grid: Vec<Vec<u32>> = vec![];

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
 */