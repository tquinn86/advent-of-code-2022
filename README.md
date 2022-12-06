# Advent of Code 2022

I've never done this before -- though I've been aware of it for a few years. Decided to get in on the fun this year. Am also taking the opportunity to teach myself Rust, of which I am admitedly quite the n00b.

The programs all run with `cargo run -- ./input.txt` The input is provided in the problem. As checked in, the command will find the answer to the day's second problem.
See the corresponding section below to get to the first.

***DISCLAIMER:*** I'm definitely not trying to be super robust or "correct" in the code, just trying to solve the problem while learning the language.

## Day 1 : 12/1/2022

The problem is [here](https://adventofcode.com/2022/day/1). I won't reiterate the problems here (though I will link). Note, each day has two problems that build upon each other. I didn't know that today, so I called the directory `dec-1-1`, thinking there would be a `dec-1-2` but there is not. Answer is [here](dec-1-1/)

Code is pretty straightforward imperative, and brute force, but gets the job done. As written, solves the second problem -- didn't think to save the answer to problem 1. Originally was just collecting a `Vec<i32>` and looped through to find the biggest one. Added the Tuple and sorting for the second part, but will solve the first part too.

## Day 2 : 12/2/2022

[Problem](https://adventofcode.com/2022/day/2) [Answer](dec-2-1/)

Today I absentmindely kept the same naming scheme, so it is `dec-2-1`, for no reason. Both problems are the one project. Again, as checked in, the code solves the day's second problem, but today, at least, I left the problem 1 code in there, commented out. Tomorrow I'll try to remember to do an interim commit for problem 1.

As you can see I was having fun with `enum`s today and a `struct` each with some impls. I managed to override the comparison operators on the `RPS` `enum` (RockPaperScissors) to deal with the circular relationship of values `Rock > Scissors > Paper > Rock ...`

Didn't have to add too much for Problem 2, just added a new enum and changed the constructor of "Round" to deal with the difference in meaning of the input.

## Day 3: 12/3/2022

[Problem](https://adventofcode.com/2022/day/3) [Answer](dec-3/)

Today was fun with Vectors and slices. Some code reuse between 1 and 2, some copypasta as well. Solution is in `dec-3` folder, but I committed after problem one and set a tag: `dec-3-1` so preserved that naming there. As above, where I replaced code for problem 2, I commented out the old code and added the new code around that. Tagged problem two as `dev-3-2` as well.

## Day 4: 12/4/2022

[Problem](https://adventofcode.com/2022/day/4) [Answer](dec-4/)

Easy one today, problem 2 was an easier version of problem one, just had to change a `==` into a `> 0`. HashSets, Vectors, `collect()` and `intersection()`. As above, tagged problems 1 and 2.

## Day 5: 12/5/2022

[Problem](https://adventofcode.com/2022/day/5) [Answer](dec-5/)

This one was pretty tricky. First, parsing of the input was divided into two distinct sections, and the first section simulates a physical, vertical, layout of crates in stacks, so translating that from horizontal lines was interesting. As I said, there were stacks, so using a `stack` data structure made sense. Closest thing in the Rust standard lib is `VecDeque` which is a double-ended (de) queue (que) which can work like either a stack (LIFO) or a queue (FIFO) or any combination. This turned out to be useful in problem 2. Most of the work in problem 1 was the parsing. Once I parsed the stacks into `VecDeque`s and then pulled the Moves into a little struct, performing the moves was fairly straigtforward. Problem 2 was just changing some ordering, and, as I said, the double-ended-ness of the `VecDeque` came in handy. See `peform_moves` and `perform_moves2` to see the difference.

## Day 6: 12/6/2022

[Problem](https://adventofcode.com/2022/day/6) [Answer](dec-6/)

Pretty easy today. Whole thing in 40 lines of code. A nested loop over a `String` and used the loop label feature to pop out of the inner. Had to change two lines of code for Part 2.