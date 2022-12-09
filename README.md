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

## Day 6: 12/7/2022 - 12/9/2022

Well, that was a journey. Capturing my thoughts after part 1, because, well, it took me 2+ days to get to the right answer. The good news is I learned a lot about Rust, which is the point. People talk about "Idiomatic Rust", I think the correct phrase should be "Ideosycratic Rust". But I digress.

### Day 1

The input today looks like someone traversing a directory tree on the command line. We're supposed to read that in, build the tree and figure out the sizes of all the directories based on their contents, including subdirectories. Then find the directories with a size less than or equal to 100000, and add them up. (I misread that last part, and that was the last thing that slowed me down. More on that below).

Anyway, build a tree, should be straightforward enough, right? I may be a Rust n00b, but I'm not a software n00b. I cut my teeth on C/C++ and n-ary trees are old hat. I first looked for some tree structure built-in to the language. Most modern standard libraries have generic data structures that include trees. I found a crate called `trees` but it didn't appear to part of the `std` lib, so I kept looking. I googled "Trees in Rust" and found some articles about how trees and Rust are "hard" and had some examples of working around the problem with an internal vector. None of these seemed authoritative, so I figured I'd give it a shot myself.

First I had to come up with the "node" type for my tree. There were two types of nodes I needed to deal with: Directories and Files. So I thought, I'll create an interface with the common methods between them and use that as the node type for my tree. In Rust the closest thing to an `interface` in other languages is a `trait`. Then I created two structs, `Dir` and `File` and had them `impl` the trait. Easy enough, right? One of the fields in the `Dir` struct was a `Vec<Item>` for the contents of the directory, and each object also had an `Option<Item>` for it's parent in the tree. `Option` because the root node has no parent. (One of the instructions in the input is `cd ..` so I'd need a way to traverse up from a node.)

One of the methods on `Item` was `size`. It could be overloaded so that the `File` implementation would just return it's size, but the `Dir` implementation would just need to loop through its vector or `Item` and call `size` polymorphically and add up the values. If one of the items was a sub-dir it would call the dir implementation recursively. Seemed pretty elegant.

The first ideosyncracy I ran into was `Vec<Item>` would not compile. The compiler told me it needed to be `Vec<dyn Item>`. The reason behind this, as Item is a trait, the compiler does not know the concrete type of the objects at compile time, so for traits as generic parameters you must declare them `dyn` for "dynamic dispatch". Now, I'm familiar with this concept. Basically this is saying that the compiler will "late bind" to this type -- find the methods at runtime and invoke them dynamically -- rather than "early bind" -- build the invocation in directly at compile time. There is generally a runtime performance hit with dynamic dispatch, but I'm not worried about that for this problem. If it works, I don't care about a few extra cycles. This ideosycracy makes Rust different from some other languages I've used. In C# `interface`s are concrete types of their own and can generally be early-bound; the compiler lays things out in such a way that this "just works." C# has late-binding features as well, of course. But that is _last job_, let's move on.

Ok, so I've got my types defined let's build the tree. I start writing the functions to parse the input and start running into the borrow checker. This is one of the main features of Rust, of course. It makes sure that you don't leak references to objects, or worse, try to access objects that have already gone out of scope and been deleted. OK, so there has to be a correct way to do this, I'll take the hints from the compiler to fix things up. The first thing it tells me to do is all "lifetime annotations". I'd seen these before but hadn't used them yet, so I thought a good time to learn how they work. Basically, these are hints that you provide to the compiler. The compiler is saying "I don't know how these inputs and outputs related to each other life-time wise, you need to help me." Or "the lifetime of this borrowed reference on this type you defined needs to outlive the object itself, add annotations to help me help you."

Speaking of "borrowed references", I should note that at this point, I was storing the "name" field on the `Dir` and `File` types as a `&str`. This is technically a reference to a string slice, the string itself can live on the heap, on the stack, or be statically baked into the binary. I'd learned that, in general, you pass strings around this way in functions, so I thought this was the right thing to do in my types as well. This turned out to be learning the difference between "owned" and "borrowed" types the hard way. Because the string was a "borrowed reference" it had to outlive the object borrowing it, so it needed an annotation `&'a str` and the same annotation on the object type itself: `Dir<'a>`. Then these objects are getting passed in and out of functions, so I start sprinkling `<'a>` all over the place. I tried to be thoughtful about the lifetimes and only put these where they were necessary, but they were necessary -- a lot.

So, I beat down many layers of compile errors -- this is taking me hours, by the way -- and I think I'm getting somewhere, when I run into something I couldn't solve. The compiler says something like "You can't move this type because it doesn't implenet the `Copy` trait" and when I try to implement the `Copy` trait, it tells me that the type _can't_ implement the `Copy` trait because the fields are not Copy. I do some googling and research and eventually come to the conclusion that the internet was right all along, this is never going to work, and I need to start over. I'd been barking up the wrong tree all day.

So, I went back to those articles I saw earlier. They are [here](https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/) and [here](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6). Both recommend using an "Arena". It is a simple struct containing a vector of nodes. The nodes refer to each other via the indexes in the vector. In general, this is not the way I had liked to implent trees (or linked-lists) in the past (though it is not super odd either). I would usually like the root of the tree to be a "node" of the tree, not some outer object. But at this point, I'm not going to be a stickler, if it can work, that would be fine. Long story short, it didn't work. I still couldn't figure out how to deal with the back references, back up the tree. At this point, I was done for the day. I decided to give it a rest and think about it.

Starting from scratch it occurred to me what I really need is a "vector of vectors", maybe there would be something there. And what I was doing wasn't "Idiomatic Rust". What was "Idiomatic Rust"? Enums! Maybe I could model `Dir` and `File` as variants of an `enum` and make my life easier. Time to sleep on it.

### Day 2

I deleted all of the `Item` related code and started from scratch. I decided the `enum` idea had some legs, so started there. I created `DirItem` and `FileItem` structs again, but now they were data objects on the `Item::Dir` and `Item::File` variants of the `Item` `enum`. This worked much better, and allowed the dir and file types to have different interfaces. No trait between them. No `dyn`. And no lifetime annotations. I still needed the `DirItem` to refer to its children, and I'm starting to anticipate the borrow checker, so I revisited the Arena stuff above. I realized what I had missed the day before -- all of the traversal methods need to be on the outer Tree type for it to work. The types I ended up with (except for one field we'll talk about in day 3 below) looked like this (you can look at the code to see all of the methods).

```Rust
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
    items: Vec<usize>
}
```

The `item` on the `DirItem` type are indexes into the `items` on the `ItemTree`. The `size` method needed to be implemented at the `ItemTree` level rather than on the `DirItem` directly because it requires traversing the tree. `DirItem` only has indexes, `ItemTree` has the items. And, you will note there are no back-pointers to parents in this structure. I solved the `cd ..` problem a different way. I kept a stack of directory indexes that got passed through the traversal methods, and when we changed directory I popped or pushed the index onto the stack. Not super elegant, but it worked. In the end, I had something build the tree from the sample input and look just like it. I even implemented the `Display` trait on the objects to output it to be sure.

Ok, now is when things get hairy again. And this time, it is not my n00b-ness that is the problem, it is the fact that I didn't read the problem correctly. The problem says get the total of all of the directories whose size is _at most_ 100000. I read that as "get the total of all directories whose size is at most 100000 _without going over_. In other words, the total itself had to remain under 100000. So, I naively (this is a little embarassing) looped over the directories, got their sizes, added them up if they were less than 100000 and stopped if the total would exceed 100000. With the sample input this worked fine because there were only two directories. In my defense on the misreading, this total _was_ less than 100000.

Anyway, I plugged in the real puzzle input and it spit out a number that was totally wrong. Doh! Of course, I can't just add up the first few, there could be ones after it that come closer, in combination, to 100000. Huh, this is harder than I thought it would be. I had to run out and run some errands at this point, so as I was driving I contemplated the combinatorics of this problem. Basically, I'd need to loop through the array of candidates (less then 100000) in a nested fashion equal to the number of candidates to find the "n" sizes that add up closest to 100000 without going over. When I got home, I did a little googling, and found there was a known problem and there was a recursive solution -- of course, that's how you do the nesting. Anyway, spent some time coding that up. Again, this worked fine with the sample input because there were only two candidates. When I plugged in the real input, and started it running, it seemed to be taking a long time. I realized then that the algorithm was O(2^n). 2^2 is only 4, but how many candidates do we have in the real input? Turned out there were 62. 2^62 is a _very_ large number. This thing could run for literally weeks! I let it run for a while, and it spit out some higher and higher numbers approaching 100000. I optimistically plugged them in, but again, _wrong_.

At this point, I'm sure I'm missing something (but did not take the time to read the problem again). We have a Slack channel with colleagues sharing experiences with AoC, so I thought I'd take a quick look there. There is a thead for each day, with spoilers, that I would usually not look at until I'm done, but at this point I'm desparate. The first entry screamed "READING COMPREHENSION" with my colleague saying he misread the problem and it slowed him down. Should have taken this hint at this point, but I didn't. The rest of the thread talked about similar issues people had with parsing the input, but nothing on an O(2^n) algorithm that would take weeks to run. One of the colleagues is using Rust as his language as well. He is decidedly not a n00b, so I tend to look at his answers after I complete mine to see what I can learn. I looked at his answer to see what he did. There was simple summing there. Huh. I also looked to see how he solved the tree problem, and his was much simpler than mine. He discared data that had nothing to do with the solution -- like file names. Makes sense, but I was into it now, and my tree was working. But where is the complicated recursive algorithm? It wasn't there. Finally "READING COMPREHENSION" clicked and I re-read the problem

Doh!, again. Just find the directories whose size is at most 100000 and add them up. Ok, easy enough. Wrote that loop, got a number plugged it in: _wrong again_. Oh yeah, I had `x < 100000` needed `x <= 100000`. Same answer, still wrong. I noticed that in the Rust solution I looked at above, he was using `u64` for the file size total, maybe something had wrapped around and given me the wrong answer. This, of course, is a red herring, `usize` is 64-bits on 64-bit machines. So, something in the code is still wrong.

I noticed that some of the candidate directory sizes are 0. While this _could_ be true, there were several, maybe this is an indication of a problem. I looked for one of the zero-sized directory names in the input and discovered the problem: that name was in there thirty times. In my traversal code, when we add a new directory, I checked to see if it was already there. I only matched by "name" so names had to be unique -- but in the input, just like real directory systems -- names can be repeated at different levels, they only need to be unique in their own scope. Of course, the sample input didn't repeat names so I didn't run into this problem there. So I needed another piece of data to uniquely identify the directory: the parent directory id perhaps? That still didn't explain why the directory size was zero, though. At this point it was getting late, so off to bed.

### Day 3

At it again, I run the thing again and look at the output. I want to figure out what is wrong that makes certain directories zero if their name is repeated. Looking more carefully, I see that there _are_ n directories with that name in the output and all are sized 0 _except for 1_. Now it is all making sense. The directories get created, but when I look up by name it always finds the first one. So all of the files are going in the first directory with that name, not the correct one. So I add a parent_id field to the `DirItem` type and populate it, and change the lookup code to match both the name and the parent id. I run it again, and the output has zero empty directories. It pops out a number, and I plug it in to the problem. __CORRECT__! Gold star for me, finally. Geez.

As I was writing this, it occurred to me that with the parent_id on the `DirItem` maybe I don't need the dir_stack I mentioned above anymore. That said, the code worked, and this is _not_ my job, so I'm just going to leave it. On to the second half. I hope this goes quicker, I'm already two days of problems behind.