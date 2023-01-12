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

## Day 7: 12/7/2022 - 12/9/2022

[Problem](https://adventofcode.com/2022/day/7) [Answer](dec-7/)

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

OK, part 2 was no big deal, thankfully. I already had what I needed. Find the smallest directory bigger than x where x is how much space we need to free to get to 3000000 out of 7000000. Done.

### Day 4

OK, apparently I can't let things go. Which hopefully is a good thing. I've learned more about Rust today than I had in all of the previous days put together. I have been working on the next day's puzzles, but the issues with this one were still bugging me. I remembered this morning that the compiler, in addition to telling me to use `dyn` with the Item trait I conjured up in the beginning, it told me to wrap it in a `Box` which it referred to as a "smart pointer". I'm familiar with the concept of smart pointers from modern C++, though it's been a few years. So `Vec<Box<dyn Item>>` was what it was looking like. I was flailing at the time I was writing that, so kind of glossed over it amid myriad compile errors, just doing what the compiler told me to do, and failing to get anything to work. This morning, when I remembered it, I realized I was missing some fundamental knowledge on how the language dealt with values allocated on the stack vs. the heap.

In C and especially C++ it is clear from the variable declaration whether it is a "stack" variable or a "heap" variable. Stack variables are _not_ pointers, their size is known at compile time and the whole thing goes into the stack, and lives as long as that stack frame lives. Of course you can pass stack variables by reference and refer to them via pointers (and that is where C and C++ can get messy) but the memory is allocated automatically on the stack. Heap variables are _always_ pointers, and the memory is allocated explicilty via a memory manager. In C++ you have the abstractions of constructors and destructors pared with the `new` and `delete` keywords to deal with allocation and deallocation of objects on the heap.

Modern C++ arose when the notion of "smart pointers" were introduced along with the general concept of RAII (which is short and for *R*esource *A*cquisition *i*s *I*nitialization). Someone (I probably should know who) came up with the nifty idea that they can create a simple class that wraps a pointer of a given type, and automatically deletes it when it goes out of scope. The smart pointer is _itself_ a stack variable, that wraps a heap variable, and automatically calls `delete` on the heap object when the stack variable goes our of scope, in its own destructor. This innovation meant that programmers almost _never_ had to call delete, explicitly, anymore, greatly simplifying memory management in C++ programs. And since smart pointers are stack variables, they tended to be passed by reference rather than passing pointers by value, significantly changing the way Modern C++ looked when compared to the C++ I wrote when I was getting started. Basically a lot more `&`s and a lot fewer `*`s.

Anyway, this reminiscence made me realize I had glossed over these fundamentals in Rust. I did not know what was on the stack and what was on the heap!?!? Which essentially meant I did not know what I was doing. So, I started doing some research, found a chapter on stack vs. heap in Rust and found out that _everything_ is stack based by default, and if you wanted to use the heap, you needed to use a smart pointer, like `Box<>`. There it was again. OK, things are clicking now. I re-read the chapter on "Ownership" in the Rust book (that I had only glossed over before) and realized that RAII is a built-in concept. I read -- for the first time -- the chapter on smart pointers, and now the whole error about the Copy trait is actually making sense. Hopefully, things will go easier now...

## Day 5 and 6

Seriously, I'm going to make myself crazy with this. I sat down yesterday I fully intended to bang through the AoC problems I hadn't gotten to yet. I even read through the first one and thought about this implementation, but was drawn back to the [Rust book](https://doc.rust-lang.org/book) and I kept reading about Traits, Lifetimes, Smart Pointers, etc. to make sure I was actually getting it. As I was reading I was thinking: "I bet I could do a much better job on that tree problem if I started over. I may even try doing it with an enum and a trait to see which works better..." And as I kept reading (RTFM!!!) I ran across an example in [chapter 15](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) of creating a tree. How about that.

In the example they define the "Node" type thus:

```Rust
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

This is very close to what I want, but the types of the fields are super complicated. Children are a RefCell of a Vector of an Ref-counted Node. and the parent is a RefCell of a weak reference to Node. In [one of the articles](https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/) linked above the author dismissively describes something close to this as an example of something they consider ridiculous:

```Rust
struct Node<T> {
    previous: Rc<RefCell<Box<Node<T>>>>,
// -- snip --
}
```

So, I wanted to understand if all of these nested types are necessary and why. And if that is even good practice.

My first attempt is based on the Item enum that I finished with above. In my node type, the children are owned by the node and the parent is an immutable reference, if set. So I started with this definition for the `DirType` associated with the `Dir` variant of my `Item` enum:

```Rust
pub struct DirItem {
    name: String,
    parent: Option<&Item>,
    items: Vec<Item>
}
```

I immediately get the compiler telling me I need lifetime annotations because of the `&Item` in parent. This is expected, so I add `<'a>` in about a dozen place throughout the file and DirItem now looks like this:

```Rust
pub struct DirItem<'a> {
    name: String,
    parent: Option<&'a Item<'a>>,
    items: Vec<Item<'a>>
}
```

Now I get the following compile errors from my `add_child` method on `Item`:

```
error[E0594]: cannot assign to `dir.parent`, which is behind a `&` reference
  --> src/items_enum/mod.rs:32:17
   |
32 |                 dir.parent = Some(self);
   |                 ^^^^^^^^^^^^^^^^^^^^^^^ `dir` is a `&` reference, so the data it refers to cannot be written

error[E0596]: cannot borrow `dir.items` as mutable, as it is behind a `&` reference
  --> src/items_enum/mod.rs:33:17
   |
33 |                 dir.items.push(*item);
   |                 ^^^^^^^^^^^^^^^^^^^^^ `dir` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error[E0507]: cannot move out of `*item` which is behind a shared reference
  --> src/items_enum/mod.rs:33:32
   |
33 |                 dir.items.push(*item);
   |                                ^^^^^ move occurs because `*item` has type `Item<'_>`, which does not implement the `Copy` trait
```

I tried solving the first two by changing the `&self` to `&'a mut self` but then had multiple immutable borrows which are disallowed by the borrow checker. But I knew I wouldn't be able to get around the third problem anyway. I needed the Item to be in a Smart Pointer of some type so ownership could be moved. Let's try `Box<Item>`.

```Rust
pub struct DirItem<'a> {
    name: String,
    parent: Option<&'a Item<'a>>,
    items: Vec<Box<Item<'a>>>
}
```

Ok, same compile errors. Turns out Box can't be copied, but it can be `clone`d. I tried that, but the cloned object does not adhere to the lifetime constraints. Plus it is trying to do a deep copy, which is not what I want. This isn't working. Let's take the book's suggestion and go with `Rc` instead. That will also allow me to use `Weak` for the parent pointer -- an unowned reference -- which is just what I want. And that should allow me to get rid of the lifetime annotations as well.

```Rust
pub struct DirItem {
    name: String,
    parent: Weak<Item>, 
    items: Vec<Rc<Item>>
}
```

After fooling around with function signatures and `**self` in a `match` statement, I kept getting the "error[E0594]: cannot assign to `dir.parent`, which is behind a `&` reference" errors as above. Hmm, seems like I need interior mutability. Guess I need that `RefCell` after all.

```Rust
pub struct DirItem {
    name: String,
    parent: RefCell<Weak<Item>>,
    items: RefCell<Vec<Rc<Item>>>
}
```

Whadddya know, it compiles. Let's see if it runs.

Ok, it took a while to get the indirections correct. Some cases of `&**self` to get at the right piece of data. But it builds a tree and gets the directory sizes recursively, like I wanted from the start. Unfortuntely, it turned out that having a flat list of directories was more conducive to solving this particular set of problems, so I ended up adding a `flatten_dirs` method to Item to pull out that flat list. This works recursively, too.

Anyway, it spits out the right answers.

Now, I'm going to try doing it with a `trait` and see how that works.

Well, `trait`s are a surprising amount of trouble. It was going smoothly but I could not downcast from `Rc<dyn Item>` to `Rc<Dir>` like I wanted to in some situations. I managed to get everything to compile using just `Rc<dyn Item>` but my
implementation of `flatten_dirs` from the previous version won't work with `Item` because I can't pass it via `self: Rc<dyn Item>` in a trait. So I can build the tree, but using it is causing difficulties. I'm going to have to traverse it differently.

I changed `flatten_dirs` to an associated method so I could preserve the `&Rc<dyn Item>` and that worked. The trait and structs look like this:

```Rust
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
```

The `as_any()` is there so I can get to a `Dir` instance from a `Rc<dyn Item>`. As noted I can't get to a `Rc<Dir>` and preserve the `Rc`-ness, but I can unwrap it by going jumping through some hoops. Example line of code (from `flatten_dirs`):

```Rust
for item in &*tree.as_any().downcast_ref::<Dir>().unwrap().items.borrow() {
    // -- snip --
}
```

The `tree` is `&Rc<dyn Item>`. `as_any()` returns a `&dyn Any`, `downcast_ref` returns `Option<T>` with `T = Dir`, so we unwrap that and go from there. Not pretty but it works.

Once I worked through the ideosyncracies (there's that word again), I think the `trait` version is slightly better? Less code, fewer `match` statements. Sprinking `dyn` everywhere is a tad annoying, but all in all not that big of a deal.

The question I asked above is still relevant though: Is it even a good practice to have all of these nested generics? I'm not sure. The language and its ideosyncracies certainly push you towards that, but there seems to be some consternation in the community as large as well. We could define type aliases for some of the more complicated ones: 

```Rust
pub type ItemVec = RefCell<Vec<Rc<dyn Item>>>;
```

And I can understand the urge to "break into jail" and use `unsafe` code to avoid some of the rigamarole. But in the end, it is possible to do this using pure, safe Rust.

See the new implementation [here](dec-7-2/). The two different implementations are in the `items_enum` and `items_trait` modules respectively. Just comment out the use statements of the one you don't want to use.

## Day 8: 12/8/2022 - 12/10/2022

[Problem](https://adventofcode.com/2022/day/8) [Answer](dec-8/)

This one went much quicker. Both problems were solved with a vector of vectors forming a grid. The logic was a little tricky, but it was fun, and less stressful. It is the weekend now, but because I was still figuring things out from above -- and well, I have a life, I only managed to get this one done today. So still two days behind, and I'll be another day behind at midnight tonight, so 3 days worth tomorrow? We'll see.

## Day 9: 12/9/2922 - 12/13/2022

[Problem](https://adventofcode.com/2022/day/9) [Answer](dec-9/)

As you can see, I got humg up on Day 7 again, so this is the Day 9 problem, completed on the 13th. Getting farther behind, but maybe I can catch up over the next day or two. I got through part 1 pretty quickly. I pulled over the `Position` type I used in Day 8. Put that in a `Rope` type to track the head and tail of the rope around the board. Nothing too tricky. Note I deliberately added `#[derive(Copy, Clone, ...)]` to the Position type to make it copyable and able to be passed by value. That came in particularly handy during part 2. I stored positions in a HashMap to keep track of the history (which is necessary to get the answer), which meant also adding `#[derive(Eq, ParialEq, Hash)]` to the mix. It is a simple struct so the default implementations of these traits come in handly. I added `#[derive(Default)]` for good measure to make initialization of the `Rope` easy.

Anyway, as I said, got through part1 fairly quickly. I struggled with the logic on part2. Eventually I figured out that I was moving laterally too far on the diagonal. This logic worked when there were only 2 knots but not with n. The second part basically lengthen's the rope from two segments to 10, so the part2 `Rope` is more generic version than part1. For this problem, I separated the parts into to separate functions, and separated the supporting types into two separate modules, so you can mix and match. I made sure the part2 types would solve the part1 problem if you start with `let r = Rope::new(2)`. Because it is so divided, only one tag today. I'll try to keep doing it this way going forward.

## Day 10: 12/10/2922 - 12/14/2022

[Problem](https://adventofcode.com/2022/day/10) [Answer](dec-10/)

Catching up a little bit. Finished part one last night and part 2 this morning. This was fun one, especially part two. You had to decode the input such that it output chars in a grid that outlined eight capital letters that were the answer.

Part 1 was simple enough, I did it all in loop, without any supporting types. In part 2 I added a simple `Sprite` type to deal with encapsulating some of the logic, but the same loop structure worked. In both cases I used a closure that was called from a couple points in the main loop to deal with state. First time I've used closures -- of my own -- in Rust (`map` takes a closure, and I've used that several times).

As with yesteday, the two parts are cleanly separated, so a single tag.

## Day 11: 12/11/2922 - 12/17/2022

[Problem](https://adventofcode.com/2022/day/11) [Answer](dec-11/)

Well, as you can see I'm falling farther behind. The why on that becomes clear below. Today involved an object representing a Monkey. Monkeys are playing keep-away with various items. Long story short, there are rounds of throwing and various calculations on each item for each monkey, so lots of nested loops. Part one wasn't too bad. I probably wrote more code than I needed to getting the output to match the example. I have a strange idea of fun, I guess. I stuck the `Monkeys` in `Box<>` and stuck those in `RefCell<>`s because the item collection held by each monkey is constantly mutating. Anyway, glad I learned that lesson, was able to get to that easily.

Part 2 is another story. It seems simple enough, they removed one calculation, and increased the number of loops. I could get from the explanation that it was likely to overflow, so I changed the types from i32 to u32 to give it more room. That was *not* enough. It overlowed _right away_. OK, let's tre u64. Took maybe half a second longer to overflow. I found a crate called [uint](https://crates.io/crates/uint) that provided **much** larger numbers. I defined a U1024 type. Surely that would be big enough. I was amazed at how quickly _that_ overflowed. At this point I realize there must be a trick. I was at a holiday party later in the day and was lamenting to one of my colleagues that there was a trick here, that I had to figure out. He said maybe BigInt _was_ the trick: write a class that can get arbitrarily large and do the math. That didn't feel right though. I confirmed with Slack that there _was_ a trick. Fortunately (or not) there were no spoilers.

***SPOILER ALERT*** Speaking of spoilers, if you don't want to know the trick -- and it isn't so much a trick as an obscure algorithm -- you should stop reading now. I'm going to tell you what the algorithm is, and how I got to it. As I said, bigger and bigger ints weren't working. My next thought was maybe the trick was to simply wrap the numbers around. Rust number types have `wrapping_add` and `wrapping_mul` so I tried that. Well, it didn't overflow, but running it with the exmaple input did not produce the right answers. So that wasn't it. I was on to something with "wrapping" but hadn't got to it yet. 

I observed that the test divisors for all of the monkeys were prime numbers. That seemed to be a clue. Then it occurred to me that all that mattered was the remainder in the calculation with the divisor, the answer of the division. So if the number had the same remainder, we could have a lesser number. This is where the wrap around occurs -- the remainders wrap around:

```
For mod 3 the answers wrap around every third number:
1 2 3 4 5 6 7 8 9 10 ...
1 2 0 1 2 0 1 2 0  1 ...
```

But simply returning the lowest number with the same remained as the calculated number didn't work either. So then I thought it had to have the same remainder of the target calculation, not the source. This was making the logic more complicated: I had to figure out where the thing was going by doing the initial calculation, then redo the calculation by mod-ing the operand with the target modulus and re-doing it. Surely I was on to something here. Coded that up, and the answers were _close_, strangely, but not correct. So, thinking about it some more, I though, OK, it isn't just the source or target but _both_. I did a little googling to refresh my math studies from, well decades ago now, and recalled this was called "congruence". The numbers are congruent across a modulus if they have the same remainder, and we needed multiple points of congruence.

So I coded up a function to find the smallest number that resulted in the same remainders with both the moduli of the target _and_ the source. Surely this was it right? Again, close, but not right. I tried throwing a third number into the mix (the number of monkeys). I'm nots ure why I thought that was important. It turned out it was but not for this reason. Anyway, the number of monkeys was not a prime number itself (4 for the example) but it was _coprime_ with the other primes (meaning it had no factors in common besides 1), which is all congruence requires. So I had `find_congruent2` and `find_congruent3` and fooled around with different source and target combinations, but things were still off. I felt like I was honing in, but was still missing something. I took a break to do something else and it hit me: the new, smaller, number didn't need to just be congruent across the source and target, it needed to be congruent across the source and _all possible targets_, meaning I had to satisfy _all_ of the moduli. So I implented `find_congruent_n` and did it recursively because the example input and the real input had different numbers of monkeys.

Lo and behold: this worked! Turned out I was using something called the `Chinese Remainder Theorem`. You'll see as ton of commented out `println!` statements for debugging purposes. It turned out the idea of the algorithm was correct, the implementation, at first, not so much. I did some more googling and found the formula, which I was shockingly close to on my own, but needed refinement to get the right answers. Once the bugs were fixed, the numbers started lining up with the example output. I started to run with the real input, and it was taking forever across 10000 iterations. I realized that my output was likely way too verbose and commented a bunch of it out. I know that Rust has the concept of Debug output, and I haven't used it yet. Seems like a good time to start doing that.

Will I catch up? Between all of the stuff going on this time of year and work, I don't know. I mean eventually I will if I don't get bored with it, but it is looking unlikely I'll finish day 25 on the 25th.

## Day 12: 12/12/2922 - 12/21/2022

[Problem](https://adventofcode.com/2022/day/12) [Answer](dec-12/)

Still plugging away, but there is not time enough in the day -- at this time of year -- to spend enough time on these, as they get more difficult. I'll catch up eventually.

Anyway, today is a "shortest path" day which tells us "Breadth First Search". We have a grid, and we need to build a breadth-first tree out of it to find the shortest path from the node marked S to the node marked E. 

So, I'm using a combination of the things I learned on day 7. Each node has an x,y position in the grid. So I'm putting each node in a HashMap keyed by position, contained in a `Grid` type, so using the Arena pattern with a `HashMap `instead of a `Vec`tor I also record the start position, end position and the maximum position for this grid (example grid and real grid are very different sizes...). 

As we know, there are two great problems of software engineering: Naming, Caching, and Off-by-one errors. I'll admit to stuggling with the BFS implementation, and my first try was riddled with off-by-one errors. I was trying to be too clever by half as the say. I wanted to do it in one pass, and I was using recursion. The first try wasn't terribly off from what I ended up with, but because of borrowing restrictions in Rust, I could not re-use a node I've already seen on the current path. The example input returned the right number, but the real input was about 100 off as it turned out.

I rewote the thing, starting from the end, going to the front. Again, it was recursive, and again the answer was off. This time by even more. I took a break, thought it through and came up with the non-recursive algorithm that worked.

The values in the HashMap are `RefCell<Box<Node>>`. I had used RefCell for the previous two tries because it was necessary for the recursion. It occured to me I might not need it with the current implementation. I tried to remove it but I needed `RefCell` to change the parent and children of each node as I build the tree. Anyway, then it is just a matter of starting from start, finding all valid children (up, down, left, right: on the grid, and traversable by the rules [lower, equal, or no more than one higher]) that we haven't seen yet. If we've seen it already, it is somewhere higher in the tree. We mark each valid child node as visited, set it as a child of the current node, and crucially, set its parent to the current node. In BFS each child can only have one parent, and that is the shortest path.

So to get the answer, after building the tree, you go to the end node and traverse its parents until you reach a node without a parent (the start node) and count them up. Done.

Part 2 is the same as part 1 except instead of having a single starting node, we need to find the 'a' node with the shortest path to 'E'. So, I imagine there is a more memory efficent way of doing this, but since I had all of the logic already, I simply looped through the grid looking for 'a's. If I find one, I clone the grid, build a tree from that starting point, and get the distance. I collect the distances that are valid (some starting points don't make it to E), and return the smallest number.

## Day 13: 12/13/2922 - 12/22/2022

[Problem](https://adventofcode.com/2022/day/13) [Answer](dec-13/)

This one was not too bad. It was basically arbitrary arrays of arrays of ints. To make this work with `Vec`tors, I had to define a recursive enum:

```Rust
pub enum PairType {
    Int(i32),
    Arr(Vec<PairType>)
}
```

I needed it to be orderable, so implemented the `PartialOrd` trait on the type, according to the rules, and that was basically it. The `<` operator did the work, and I got to the answer.

In part 2, I thought I could just re-use the types from part one, pull all of the lines into a `Vec` and sort it. Turned out `Vec.sort()` requires `Ord` not `PartialOrd` (makes sense) so I had more work to do. I copypasta-ed the types to part2 and added the missing implementation (which was a total of four new lines, plus removing some `Some`s). I took advantage of the copy to change the name of `PairType` to `PacketType` since it was no longer part of a pair. I guess I could have just named it `Packet`...

Anyway the sort worked as expected and the answer was right on the first try.

## Day 14: 12/14/2922 - 12/24/2022

[Problem](https://adventofcode.com/2022/day/14) [Answer](dec-14/)

That one was fun, and not too difficult. Basically you build a pachinko like structure from the input and start dropping balls into it and see where they land based on  specific rules. A few nested loops and that was that. Part 1 you dropped balls (well, sand pebbles) until they start falling into oblivion, in part 2 we build a floor and fill the thing up to the top. I thought the part 2 number would be larger given how long it took to run. Anyway, got it done, on Christmas Eve. Probably won't be looking at the next one for a couple days.
