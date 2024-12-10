# Advent of Code 2024

Following [AoC2024](https://adventofcode.com/2024), and trying to git gud at Rust :crab:.

Extra challenges:

- For loops? :thumbsdown: Unreadable chains of iterator adapters? :thumbsup:
- Serde goes on everything
- Never `unwrap()`, never `panic!`
- (From day 6) :zap: Gotta go fast :zap: If you can speed it up with `rayon`, you should

## Challenge Log

Every day: What was hard, and did I learn something interesting?

TODO: Write this up as a blog somewhere...

### Day 1

This is my first try at advent of code, and I'm not expecting to be competitive. Instead,
I'm aiming to use this to improve my Rust skills and get used to solving problems that are
outside my comfort zone.

Day 1 was fairly straightforward, especially for part 1. I spent longer trying to read the
input file using the `csv` crate with `serde` than I did actually solving the problem!
Part 2 was a little more interesting as it taught me about the `Entry` API for `HashMap`:

```rust
map.entry(val).and_modify(|x| *x += 1).or_insert(1)
```

This is a really neat way to handle the cases in which an entry may or not already exist.
You can do it when `HashMap` contains other data structures too:

```rust
// For a HashMap<_, Vec<_>>, insert an empty vec if an entry doesn't exist yet
map.entry(x).or_insert(Vec::new()).push(y)
```

### Day 2

I found this one to be a fair bit harder than day 1, but nothing too challenging. The
coolest thing I learned was `tuple_windows`, which gives a windowed view into an
iterator:

```rust
let diff: Vec<_> = levels
    .iter()
    .tuple_windows::<(_, _)>()
    .map(|(x, y)| y - x)
    .collect();
```

I'm also aware that you can just call `vec.windows(n)`, so that's something to try at
some point.

I was quite happy with my solution to part 2, although it's a bit of a brute force
solution: simply iterate over the possible combinations of the levels minus one element.

```rust
fn is_partially_safe(levels: &[i32]) -> bool {
    levels
        .iter()
        .combinations(levels.len() - 1)
        .map(|x| x.into_iter().copied().collect::<Vec<_>>())
        .any(|x| is_strictly_safe(&x))
}
```

### Day 3

Good ole' regex. I had a pretty easy time with part 1 of this one, but got caught out
in part 2. I first tried deleting everything matching the pattern `don't\(\).*do\(\)`,
but as regex is greedy, this deleted everything from the first `don't()` to the last
`do()`! Instead, I found that splitting on `do\(\)`, deleting everything after
`don't\(\)`, collecting everything into a `String` and feeding the result back into
my solution to part 1 did the trick.

Besides a harsh reminder of the dangers of poorly defined regex, I didn't really pick
up anything new today. I did get a chance to use the `lazy_regex` crate though, which
I find offers a nicer interface than the standard `regex` crate (the performance
benefits are an added bonus!).

### Day 4

I can't say I'm proud of this one...

My self-imposed rule of 'no for-loops' really messed me up, as this is a problem where
the best solution is probably just a double-nested for-loop over indices `i` and `j`.
My solution involved creating horizontal, vertical and diagonal iterator 'stencils'
that scan over the 2D grid using multiple `.zip()` calls. There's lot of error-prone
slicing involved, so in general I wouldn't recommend it!

A cool trick I learned by checking the subreddit afterwards is that padding the 2D
grids is typicaly easier than checking the boundaries. That's one to remember for
later.

### Day 5

I got caught out overthinking this one! I had thought there was a global ordering,
so everything went wrong when I tried to figure it out. After realising that only
a subset of the rules were applicable to each input, things went much more smoothly.
I ended up making a `HashMap` that maps each page to an ordering integer, and then
using that in a sort function. There's probably a more efficient way to handle this.

Part 2 was really easy, as I'd already gone about solving part 1 the hard way. Rather
than just checking if each input followed the rules, I instead sorted the input and
then compared to the original -- if they matched, then the input must follow the
rules! As I was already sorting the inputs, the extension to part 2 was trivial.

### Day 6

This one was fun! It reminded me of playing Metal Gear when I was younger, watching
the guards walk around in circles and trying to figure out when I could pass by
undetected. I also got a chance to try out the grid padding trick I learned in
day 4.

My first solution was painfully slow, and I learned an important lesson about Rust:
characters are not bytes, and `String` cannot be cheaply modified in place! As there
was only a limited set of characters, I had the idea of encoding them as enums and
working with those instead, which made things much faster. I also sped things up
by tracking the positions the guard had walked on and which direction they'd been
facing -- that way, you can detect an infinite loop if they ever step on a familar
tile facing a familiar direction. Previously I'd just been checking that they hadn't
walked 4 times the number of grid cells!

At this point I decided to go all out with optimisations. I used `bit_flags` to
represent the direction they were facing, as then you can really efficiently encode
all four possibilities and all combinations in a `u8`. I used `rayon` to really speed
things up. I eventually got it down to just about 30ms on my 4-core machine, which I
was really happy with! I could probably get it even faster if I tried only putting 
an obstruction on the locations the guard stepped on in part 1.

After this, I decided to add a new rule to my challenges: to aim for optimised
solutions, and to use `rayon` wherever it seems like a good idea.

### Day 7

I found this one really easy compared to the previous days. It was just a simple
recursion problem, and after slapping `rayon` on it I could have the whole thing
solved in less than 100ms. After my intial solution, I got it under 70ms by
avoiding recalculating the inputs that were valid in part 1. There are almost
certainly further optimisations to be made here.

### Day 8

This was an interesting problem. After the pain of working with characters in day 6,
today I opted to read the 2D grid of data as bytes, so instead was working with `u8`
throughout. I also decided to write my own struct to contain the 2D data and `impl`
some common operations on it -- things like a `pos` function which converts a 2D
coordinate into a 1D grid index, or `None` if the coordinate isn't on the grid.

The second part of this challenge initially seemed really tricky, but then I realised
it could be solved quite easily by finding the distance between two antennas in each
dimension and dividing by their greatest-common-divisor (`num` crate to the rescue!).
You can then simply walk in that direction and accumulate the answers until you fall
off the grid. As I'd used `.permutations()` from the `itertools` crate to generate
each antenna pairing, I didn't even need to bother checking in both directions.

There are almost certainly ways to optimise this further, such as keeping a grid of
bit flags to denote where unique antinodes exist and also where the 'resonant harmonics'
nodes should be. I just tracked the unique locations in each case with a `HashSet`,
and with `rayon` I can solve the whole thing in 2ms, which is good enough for me.

#### A follow-up

After some thought, I've managed to get my solution to part 2 down to a really neat
little function:

```rust
    // Takes the positions of each antenna, expressed as 1D indices
    fn antinodes(&self, pos_1: usize, pos_2: usize) -> Vec<usize> {
        // Get the 2D coordinate of antenna 1
        let (row_1, col_1) = self.coord(pos_1);
        // Determine the closest node at which 'resonant harmonics' occurs
        // Uses num::integer::gcd internally
        let nearest = self.nearest_harmonic(pos_1, pos_2);
        // Iterate from 1 to infinity, and -- while we're still on the grid --
        // generate new harmonics
        (1..)
            .map_while(|harmonic| {
                self.pos(row_1 + harmonic * nearest.0, col_1 + harmonic * nearest.1)
            })
            .collect()
    }
```

The `map_while` adapter is definitely one to remember! There are probably a few
awkward bits in earlier challenges that could have been simplified by combining
this with integer slicing.

### Day 9

#### Part 1

This problem had me stumped for a while. After getting the filesystem ready, I tried a
few solutions to build the compacted filesystem with clever `itertools` magic. I first
had the bright idea of setting empty space to be `usize::MAX`, so that any number
compared against it would take priority. I also figured out that the following two
iterators needed to be merged:

- `filesystem.iter()`
- `filesystem.iter().rev().filter(not_space)`

I first tried using `merge()` from `itertools`, but with no luck -- as soon as the first
iterator hits `usize::MAX`, it freezes while the second iterator exhausts itself! I then
tried using a simple `zip()` but this didn't work either, as then both iterators would
march regardless of whether their value had been taken. In the end, I had to manually
keep track of the second iterator and call `next()` whenever the first iterator returned
`usize::MAX`:

```rust
fn compact(filesystem: &[usize]) -> Vec<usize> {
    let len = filesystem.iter().filter(not_space).count();
    let mut rev_iter = filesystem.iter().rev().filter(not_space);
    filesystem
        .iter()
        .scan(filesystem.iter().rev().filter(not_space), |rev_iter, x| {
            if not_space(&x) {
                Some(*x)
            } else {
                Some(*rev_iter.next().unwrap_or(&SPACE)) // unwrap should never fail
            }
        })
        .take(len)
        .enumerate()
        .map(|(idx, val)| idx * val)
        .sum()
}
```

I feel there must be a cleaner solution than this, but it's solving the problem in just
a few milliseconds, so at least it seems somewhat efficient.

I expect part 2 to be a huge pain, as the data structure I used for part 1 doesn't
lend itself nicely to this problem.

#### Part 2

I'll admit that I had to ask a friend for help with this one, as I couldn't think of a
clean way to solve it with my data structure from part 1. The solution was to not bother
building the filesystem, but to instead work directly with the diskmap provided as an
input.

Even knowing how to solve this, I found it to be a real challenge. My solution makes
significant use of the iterator `scan()` function, which works like `fold()` and
`reduce()` but the state passed between iterations is persistent and mutable. This
allowed me to keep track of mutable versions of the diskmap and a `Vec` of the start
locations of each file block/space and then modify them between iterations. In the end,
I don't think my solution is particularly elegant, and there's certainly room for
performance improvements, but it got the job done. Running on the release profile, it's
solving it in about 0.1s, which isn't too bad.
