[![crates.io](https://img.shields.io/crates/v/sea-canal.svg)](https://crates.io/crates/sea-canal) [![Build Status](https://travis-ci.org/saghm/sea-canal.svg)](https://travis-ci.org/saghm/sea-canal)

SeaCanal
====================================================================================================================================

Ever take one of those tests where you're given a sequence of integers, and you
have to tell them the next one in the sequence? For example, you might be shown
something like this:

```
# Which number fits into the blank spot?
7 1 3 9 3 5 25 19 _
```

The pattern, in this case, is to subtract six, then add two, then square, which
means the correct answer is 21.

Obviously, people have varying experiences when attempting to solve problems
like this, ranging from "oh, this is fun" to "no thanks, I'd rather go put some
nails in my blender and listen to that for a while". However, I think we can
all agree that however easy or difficult you may find these, they are so
abstract and blatantly pointless that there are better things to be doing than
taking a test that has these problems in it. That's where I got the idea for
SeaCanal.

## What is SeaCanal?

SeaCanal is a "sequence analyzer" written in Rust ("seq-anal"...get it? Look, if
you don't want to deal with awful puns, you probably shouldn't be reading things
on my Github). Basically, you give it a sequence of numbers, and it tells you
any patterns it can find in it. Theoretically, if you were to be taking one of
those dumb tests,and you had access to a computer, you could plug in the
sequence, and SeaCanal would tell you what the pattern is (don't actually do
that though, cheating is bad).

## Installation

### Library

SeaCanal is published on [crates.io](https://crates.io), so you can use it like
any other Cargo package:

Add the following to your `Cargo.toml` under the `[dependencies]` section:

```
sea-canal = "0.1.1"
```

Add the following to the root of your project:

```
extern crate sea_canal;
```

The next time you run `cargo build`, the package will be installed for that
project.

### Standalone CLI

Simply run `cargo install sea-canal`.

## Usage

### Library

First, import `Analyzer`, as well as the `Analyze` trait:

```
use sea_canal::{Analyze, Analyzer};
```

Create an analyzer for whichever sequence you want to analyze:

```
let analyzer = Analyzer::from_seq(&[7, 1, 3, 9, 3, 5, 25, 19]);
```

Then call either one of the `find_patterns` methods to find all patterns, or
one of the `find_any_pattern` methods to find a single pattern (giving either a
maximum length or exact length of the pattern, depending on the method):

```
println!("{:?}", analyzer.find_any_pattern(7))
```

### Standalone CLI

Assuming you've set up your path correctly for your `cargo install` binaries,
you can run SeaCanal with `scnl`. Then just type in a (whitespace-delimited)
sequence of integers, and hit "enter".

## How does SeaCanal work?

Feel free to skip this if you don't like boring math. (If this bothers you
because "math isn't boring!", then fine, don't skip it).

First, SeaCanal looks at each pair of adjacent numbers in the sequence and
computes the possible operations that could lead from one to another. Right now,
the operations it supports are rather limited; it supports addition/substraction
of a constant, multiplication/division of a constant, squaring, cubing,
square-rooting, and cube-rooting, as well as standalone constants (e.g.
"always 7"). For example, the analysis of the first sequence above would look
like this:

```
7 -> 1:   =1,  -6,  /7
1 -> 3:   =3,  +2,  *3
3 -> 9:   =9,  +6,  *3, ^2
9 -> 3:   =3,  -6,  /3, root 2
3 -> 5:   =5,  +2
5 -> 25:  =25, +20, *5, ^2
25 -> 19: =19, -6
```

To make it easier to talk about, we'll call the adjacent numbers in a sequence
a "transition", and we'll call the set of possible operations describing a
transition a "choice".

SeaCanal then starts trying to find a pattern with the fewest operations
possible. This means that it first tries to find a pattern of one operation;
if it can't find one that fits, then it tries to find one with two operations.
This keeps repeating until it finds a pattern than matches or it has reaches an
upper bound (which is passed in by the user). This is to ensure that it doesn't
generate useless patterns. For instance, if for this example, the pattern
`[/7, *3, *3, /3, +2, *5, -6]` is a not very meaningful, despite the fact that
these numbers *could* technically be the start of a sequence with that pattern.

To identify a pattern of a given length `n`, SeaCanal divides up the transitions
into slices of size `n` (the last slice might be smaller, which is fine), and
then groups together the ones that occur in the same location in the slices.
Finally, SeaCanal tries to find a common choice among all the transitions in a
given group.

Following our example, the slicing for `n = 1` would just be each transition
in its own slice, so all of the transitions would be grouped together. Finding
whether there is a pattern of length 1 would just mean trying to find a choice
that's in every transition. Obviously, in this case, there is no such choice:

```
# There is no operation that appears in each of the lists below
7 -> 1:   -6,  /7
1 -> 3:   =3,  +2,  *3
3 -> 9:   =9,  +6,  *3, ^2
9 -> 3:   =3,  -6,  /3, root 2
3 -> 5:   =5,  +2
5 -> 25:  =25, +20, *5, ^2
25 -> 19: =19, -6
```

We move on to `n = 2`. The slicing looks like this:

```
7 -> 1,  1 -> 3
3 -> 9,  9 -> 3
3 -> 5,  5 -> 25
25 -> 19
```

and the groups would be:

```
7 -> 1, 3 -> 9, 3 -> 5, 25 -> 19                  1 -> 3, 9 -> 3, 5 -> 25
```

In the first group, our list of choices is this:

```
7 -> 1:   =1,  -6,  /7
3 -> 9:   =9,  +6,  *3, ^2
3 -> 5:   =5,  +2
25 -> 19: =19, -6
```

We still don't have any operation that appears in all of the choices, which
means there aren't any patterns of length 2.

With `n = 3`, the slices would be:

```
7 -> 1,  1 -> 3, 3 -> 9  
9 -> 3,  3 -> 5, 5 -> 25
25 -> 19
```

And the groups would be:

```
7 -> 1, 9 -> 3, 25 -> 19           1 -> 3, 3 -> 5           3 -> 9, 5 -> 25
```

Looking at the first group, we find the common operation `-6`:

```
7 -> 1: =1,  -6,  /7
9 -> 3: =3,  -6,  /3, root 2
25 -> 19: =19, -6
```

In the second group, we find `+2`:

```
1 -> 3:   =3,  +2,  *3
3 -> 5:   =5,  +2
```

And finally, in the last group, we find `^2`, which completes the pattern:

```
3 -> 9:   =9,  +6,  *3, ^2
5 -> 25:  =25, +20, *5, ^2
```
