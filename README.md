[![crates.io](https://img.shields.io/crates/v/sea-canal.svg)](https://crates.io/crates/sea-canal) [![Build Status](https://travis-ci.org/saghm/sea-canal.svg)](https://travis-ci.org/saghm/sea-canal)

SeaCanal
========

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
nails in my blender and listen to that for a while". However, I think we can all
agree that however easy or difficult you may find these, they are so abstract
and blatantly pointless that there are better things to be doing than taking a
test that has these problems in it. That's where I got the idea for SeaCanal.

What is SeaCanal?
-----------------

SeaCanal is a "sequence analyzer" written in Rust ("[seq]uence [anal]ysis" ->
"SeqAnal"...get it? Look, if you don't want to deal with awful puns, you
probably shouldn't be reading things on my Github). Basically, you give it a
sequence of numbers, and it tells you any patterns it can find in it.
Theoretically, if you were to be taking one of those dumb tests, and you had
access to a computer, you could plug in the sequence, and SeaCanal would tell
you what the pattern is (don't actually do that though, cheating is bad).

Installation
------------

### Library

SeaCanal is published on [crates.io](https://crates.io), so you can use it like
any other Cargo package.

Add the following to your `Cargo.toml` under the `[dependencies]` section:

```
sea-canal = "0.3"
```

Add the following to the root of your project:

```
extern crate sea_canal;
```

The next time you run `cargo build`, the package will be installed for that project.

### Standalone CLI

Simply run `cargo install sea-canal`.

Usage
-----

### Library

First, import `Analyzer`:

```
use sea_canal::Analyzer;
```

Create an analyzer for whichever sequence you want to analyze:

```
let analyzer = Analyzer::from_seq(&[7, 1, 3, 9, 3, 5, 25, 19]);
```

Then call either one of the `find_patterns` methods to find all patterns, or one
of the `find_any_pattern` methods to find a single pattern (giving either a
maximum length or exact length of the pattern, depending on the method):

```
println!("{:?}", analyzer.find_any_pattern(7))
```

### Standalone CLI

Assuming you've set up your path correctly for `cargo install`, you can run
SeaCanal with `scnl`. Then just type in a (whitespace-delimited) sequence of
integers, and hit "enter".

Alternately, to see a (very small) sample of SeaCanal analyzing some preset
sequences, run `scnl --sample`.

How does SeaCanal work?
-----------------------

(This is really verbose, so feel free to skip it, especially if you find basic
arithmetic boring).

First, SeaCanal looks at each pair of adjacent numbers in the sequence and
computes the possible operations that could lead from one to another. You can,
see the operations it supports in [Operation Types](#operation-types).
For example, the analysis of the first sequence above would look like this:

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
generate useless patterns. This is generally helpful; for this example, the
pattern `[/7, *3, *3, /3, +2, *5, -6]` is not very meaningful for the above
sequence, despite the fact that these numbers *could* technically be the first
iteration of such a pattern.

To identify a pattern of a given length `n`, SeaCanal divides up the transitions
into slices of size `n` (the last slice might be smaller, which is fine), and
then groups together the ones that occur in the same location in the slices.
Finally, SeaCanal tries to find a common choice among all the transitions in a
given group.

Following our example, the slicing for `n = 1` would just be each transition in
its own slice, so all of the transitions would be grouped together. Finding
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
Slice 1: 7 -> 1,  1 -> 3
Slice 2: 3 -> 9,  9 -> 3
Slice 3: 3 -> 5,  5 -> 25
Slice 4: 25 -> 19
```

and the groups would be:

```
Group 1: 7 -> 1, 3 -> 9, 3 -> 5, 25 -> 19
Group 2: 1 -> 3, 9 -> 3, 5 -> 25
```

(If you evenly space out each slice in a row, then the groups are just the
individual columns).

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
Slice 1: 7 -> 1,  1 -> 3, 3 -> 9
Slice 2: 9 -> 3,  3 -> 5, 5 -> 25
Slice 3: 25 -> 19
```

And the groups would be:

```
Group 1: 7 -> 1, 9 -> 3, 25 -> 19
Group 2: 1 -> 3, 3 -> 5
Group 3: 3 -> 9, 5 -> 25
```

Looking at the first group, we find the common operation `-6`:

```
7 -> 1:   =1,  -6,  /7
9 -> 3:   =3,  -6,  /3, root 2
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

(Note that the common operations for each transition don't actually have to be
in the same "column" like they are for these three groups; I just put them like
that so it would be visually easier to notice).

## Operation Types

### Built-in

#### Basic Arithmetic

* +
* -
* \*
* /

**NOTE**: Modulus is not yet implemented

#### Exponents

* Square
* Square root
* Cube
* Cube root

### Constants

A constant element in a sequence. For example, the sequence `2 6 8 4 12 8 4`
could be described by the pattern `*3, =8, -4` (where `=8` means a constant
value of `8`).

On their own, patterns with constants are not very interesting because they tend
to start repeating after the second iteration. However, when used with
[meta-patterns](#meta-patterns), this will not necessarily occur.

### Custom operations

You can define custom operation by making an instance of the
`CustomPatternElem` struct. Doing this requires defining a function
of type `(i32, i32) -> bool`, which acts as a test to determine if a given pair
of adjacent numbers in a sequence can be described by that operation. For
example, the following code tests a sequence with a custom operation for raising
a number to the fourth power:

```rust
#[macro_use]
extern crate sea_canal;

use sea_canal::Analyzer;
use sea_canal::{CustomPatternElem, Pattern};
use sea_canal::PatternElem::*;

fn pow4(i: i32, j: i32) -> bool {
    i * i * i * i == j
}

let pow4_pattern = CustomPatternElem::new(pow4, "^4");
let slice = &[2, 16, 3, 81, 68];
let analyzer = Analyzer::with_custom_patterns(slice, vec![pow4_pattern.clone()]);

assert_eq!(Some(pat![Custom(pow4_pattern), Minus(13)]), analyzer.find_any_pattern(4));
```

### Meta-Patterns

A "meta-pattern" occurs when an operation is not constant but itself follows a
pattern. For instance, consider the following sequence:

```
1 2 4 7 11...
```

Notice that each transition is a plus operation with a operand one higher than
the previous (`+1`, `+2`, `+3`...). We can't describe this sequence with a
pattern the way we've defined them above, but we still might want a way to be
able to identify the behavior of such sequences.

Meta-patterns don't have to describe every operand in the sequence; just like a
regular operand, they only need to describe a given choice. For example, the
following sequence fits the pattern `[+1, +2, +3, +4 ...], =10`:

```
10 11 10 12 10 13
```

A meta-pattern also doesn't have to have only one operand type. For example, we
can modify the above sequence to fit the pattern `[+1, *2, +3, *4...], =10`:

```
10 11 10 20 10 13 10 40
```

Meta-patterns are implemented by analyzing the sequence of the operands of the
choices and seeing if a pattern emerges. To be a valid meta-pattern, each
operand must have a numerical parameter (e.g. `/` works, but not `root 2`, since
there is no implemented operation for arbtrary roots) and the types of the operands
must be repeating.

Note that when searching for a pattern among the numerical values of the
operands, meta-patterns are *not* considered. For instance, if a sequence is
described by the operations `[+1, +2, +4, +7, +11]`, it will not report finding a
meta-pattern.


#### Finding meta-patterns

To find meta-patterns, use the `with_meta` constructor:

```rust
let slice = &[10, 11, 10, 12, 10, 13];
let analyzer = Analyzer::with_meta(slice);

assert_eq!(pat!(Meta(pat!(Plus(1), Plus(2), Plus(3))), Const(10)), analyzer.find_any_pattern(4));
```

To use custom operations when searching for meta-patterns, use the `with_options` constructor.
