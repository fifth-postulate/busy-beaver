layout: true
class: middle, center

---
# Uncomputable Functions

--
### Exploring the frontiers of science

---
background-image: url(image/PhDKnowledge.012.jpg)
background-position: center
background-size: contain

---
background-image: url(image/turing.jpg)
background-position: center
background-size: contain

---
.turing-machine-description[{
  "tm": {
    "tape": {
      "left": [],
      "current": "I",
      "right": ["I", "I", "I"]
    },
    "state": 0,
    "transitions": [
      { "current": [0, "_"], "next": [1, "I", "L"] },
      { "current": [0, "I"], "next": [0, "I", "R"] },
      { "current": [1, "_"], "next": [2, "_", "R"] },
      { "current": [1, "I"], "next": [1, "I", "L"] }
    ]
  },
  "blank": "_",
  "visible_tape": 4,
  "running": false
}]

---

| | 0 | 1 | 2 |
|-|---|---|---|
|_|IL1|_R2| H |
|I|IR0|IL1| H |

---

| | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|-|---|---|---|---|---|---|---|
|_|_R1|_L2|_R7|_L5|IL5|_R6| H |
|I|IR0|IR1|_L3|_L4|IL4|IL5| H |

---
.turing-machine-description[{
  "tm": {
    "tape": {
      "left": [],
      "current": "I",
      "right": ["I", "", "I", "I"]
    },
    "state": 0,
    "transitions": [
      { "current": [0, "_"], "next": [1, "_", "R"] },
      { "current": [0, "I"], "next": [0, "I", "R"] },
      { "current": [1, "_"], "next": [2, "_", "L"] },
      { "current": [1, "I"], "next": [1, "I", "R"] },
      { "current": [2, "_"], "next": [7, "_", "R"] },
      { "current": [2, "I"], "next": [3, "_", "L"] },
      { "current": [3, "_"], "next": [5, "_", "L"] },
      { "current": [3, "I"], "next": [4, "_", "L"] },
      { "current": [4, "_"], "next": [5, "I", "L"] },
      { "current": [4, "I"], "next": [4, "I", "L"] },
      { "current": [5, "_"], "next": [6, "_", "R"] },
      { "current": [5, "I"], "next": [5, "I", "L"] }
    ]
  },
  "blank": "_",
  "visible_tape": 4,
  "running": false
}]

---

```rust
/// The different directions the tape head can move in.
pub enum Direction {
    /// The tape head can move left
    Left,
    /// The tape head can move right
    Right,
}
```

---

```rust
/// The various symbols that can be written on the tape.
pub enum Symbol {
    /// the blank symbol, represented as "0"
    Blank,
    /// the non blank symbol, represented as "1"
    NonBlank,
}
```

---

```rust
/// The states a Turing machine can be in
pub enum State {
    /// The halted state signals that
    /// the Turing machine finished operating.
    Halted,
    /// a non-halted state, indexed by an natural number.
    Number(u8),
}
```

---

```rust
/// Which actions a Turing machine can do
pub enum Action {
    /// A Turing machine can halt
    Halt,
    /// Or a Turing machine can do something
    Do {
        /// Symbol to write to the tape
        symbol: Symbol,
        /// The direction to move the tape head in
        direction: Direction,
        /// Which state to transition in
        state: State,
    },
}
```

---

```rust
/// The current configuration of a Turing machine
pub struct Key {
    /// The state the Turing machine is in
    pub state: State,
    /// The symbol that the tape head is reading
    pub symbol: Symbol,
}
```

---

```rust
/// The instructions for a Turing machine
pub trait Program {
    /// Lookup the action for the specific key
    fn lookup(&self, key: &Key) -> Lookup;
}
```

--

```rust
/// The result of looking up a certain key in a program.
pub enum Lookup {
    /// The key is not known to the program. A semantic error
    Unknown,
    /// The determined action for this key
    Determined(Action),
}
```

---

```rust
/// Characteristics of a tape.
pub trait Tape {
    /// Move the tape head in a direction
    fn move_to(&mut self, direction: &Direction);
    /// read the symbol from the tape
    fn read(&self) -> Symbol;
    /// write a symbol to the tape
    fn write(&mut self, symbol: Symbol);
}
```

---

```rust
/// A Turing machine
pub struct Machine<'a, T>
where
    T: Tape + Sized,
{
    tape: T,
    program: &'a dyn Program,
    state: State,
}
```

---

```rust
/// Take a single step
pub fn step(&mut self) -> Progress {
    if !self.state.halted() {
        let key = Key {
            state: self.state,
            symbol: self.tape.read(),
        };
        match self.program.lookup(&key) {
            Lookup::Unknown => Progress::Stuck,
            Lookup::Determined(Action::Halt) => {
                self.state = State::Halted;
                Progress::Made
            }
            Lookup::Determined(Action::Do {
                symbol,
                direction,
                state,
            }) => {
                self.tape.write(symbol);
                self.tape.move_to(&direction);
                self.state = state;
                Progress::Made
            }
        }
    } else {
        Progress::Halted
    }
}
```

---
## &#129300;

---
## _Universal_ Turing Machine

--

> a Turing machine that can simulate an arbitrary Turing machine on arbitrary input

---
## Halting Problem

--

> Decide for a TM when given input I if the machine halts eventually.

---
## Undecidable

--

> There is **no** TM that accurately decides if a TM will halt on input I

---
## \\(\mathcal{H}\\)

---
## \\(\mathcal{H}'\\)

> if \\(\mathcal{H}\\) decides that \\(T\overline{T}\\) halts, \\(\mathcal{H}'\overline{T}\\) does **not** halt.<br>
> when \\(\mathcal{H}\\) decides that \\(T\overline{T}\\) does **not** halt,  \\(\mathcal{H}'\overline{T}\\) halts.<br>

---
## \\(\mathcal{H}'\overline{\mathcal{H}'}\\)?

--

> \\(\mathcal{H}'\overline{\mathcal{H}'}\\) halts when \\(\mathcal{H}\\) decides that \\(\mathcal{H}'\overline{\mathcal{H}'}\\) does **not** halt.<br>
> \\(\mathcal{H}'\overline{\mathcal{H}'}\\) does **not** halt when \\(\mathcal{H}\\) decides that \\(\mathcal{H}'\overline{\mathcal{H}'}\\) halts.<br>

---
## &#128136;

---
## Meta-Theorem

> Any interesting aspect is undecidable!

---
## Busy Beaver

--

> What is the maximum number of steps a Turing Machine with \\(n\\) states can take on the empty tape before it halts.

--

> What is the maximum number of 1s a Turing Machine with \\(n\\) states can write on the empty tape before it halts.

---

| \\(n\\)      | 1 | 2 | 3  | 4   | 5         |
|--------------|---|---|----|-----|-----------|
| \\(S\\)      | 1 | 6 | 21 | 107 | 47176870? |
| \\(\Sigma\\) | 1 | 4 | 6  | 13  | 4098?     |

---
## How hard can it be?

---
## \\(10^{80}\\)

---

## \\((2\cdot 2\cdot k + 1)^{2\cdot k}\\)

--

| \\(n\\)      | 1            | 2            | 3            | 4             | 5              |
|--------------|--------------|--------------|--------------|---------------|----------------|
| #TM          | 25           | 6561         | 4826809      | 6975757441    | 16679880978201 |
| \\(10^{m}\\) | \\(10^{2}\\) | \\(10^{4}\\) | \\(10^{7}\\) | \\(10^{10}\\) | \\(10^{14}\\)  |

---
background-image: url(image/how-it-started.jpg)
background-position: center
background-size: contain

## Plan

---

```rust
/// An iterator for `Direction`s.
pub struct Directions {
    current: Option<Direction>,
}

impl Directions {
    /// Creates an iterator that iterates over all directions.
    pub fn all() -> Self {
        Self {
            current: Some(Direction::Left),
        }
    }
}
```

---

```rust
impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current;
        self.current = match item {
            Some(Direction::Left) => Some(Direction::Right),
            _ => None,
        };
        item
    }
}
```

---

```rust
/// Iterator for `State`s.
pub struct States {
    maximum: u8,
    current: Option<State>,
}

impl States {
    /// Create an iterator for states
    /// up to a maximum state index,
    /// including the halt state.
    pub fn up_to(maximum: u8) -> Self {
        Self {
            maximum,
            current: Some(State::Halted),
        }
    }

    /// Create an iterator for states
    /// up to a maximum state index,
    /// excluding the halt state.
    pub fn non_halted_up_to(maximum: u8) -> Self {
        Self {
            maximum,
            current: Some(State::Number(0)),
        }
    }
}
```

---

```rust
impl Iterator for States {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current;
        self.current = match item {
            Some(State::Halted) => {
                if self.maximum > 0 {
                    Some(State::Number(0))
                } else {
                    None
                }
            }
            Some(State::Number(m)) => {
                if m + 1 < self.maximum {
                    Some(State::Number(m + 1))
                } else {
                    None
                }
            }
            _ => None,
        };
        item
    }
}
```

---

```rust
/// Iterator for keys
pub struct Keys {
    iterator: Box<dyn Iterator<Item = Key>>,
}

impl Keys {
    /// Iterate through a number of keys up to a maximum
    pub fn up_to(maximum: u8) -> Self {
        let iterator = cartesian!(
            States::non_halted_up_to(maximum),
            Symbols::all()
        ).map(|tuple| {
            tuple.into()
        });
        Self {
            iterator: Box::new(iterator),
        }
    }
}
```

---

```rust
impl Iterator for Keys {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}
```

---

```rust
/// Iterator for actions
pub struct Actions {
    iterator: Box<dyn Iterator<Item = Action>>,
}

impl Actions {
    /// Create a iterator that iterates through a number of states up to a maximum.
    pub fn up_to(maximum: u8) -> Self {
        let iterator =
            once(Action::Halt)
            .chain(States::non_halted_up_to(maximum)
            .flat_map(|state| {
               cartesian!(Symbols::all(), Directions::all())
               .map(move |tuple| {
                    (tuple.0, tuple.1, state).into()
                })
            }));
        Self {
            iterator: Box::new(iterator),
        }
    }
}
```

---

```rust
impl Iterator for Actions {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}
```

---

```rust
/// Iterator for complete progams
pub struct CompletePrograms {
    iterator: Box<dyn Iterator<Item = CompleteProgram>>,
}

impl CompletePrograms {
    /// Create an iterator that iteratos through all complete programs of a certain number of states
    pub fn all(n: u8) -> Self {
        match n {
            1 => all1(),
            2 => all2(),
            3 => all3(),
            4 => all4(),
            5 => all5(),
            _ => panic!("it is unwise to go beyond 5"),
        }
    }
}
```

---

```rust
macro_rules! all_programs {
    ($n:tt, $fname:ident) => {
        fn $fname() -> CompletePrograms {
            let iterator = actions!($n)
            .map(tuple!($n))
            .map(|actions| {
                let mut program = CompleteProgram::new();
                for (key, action) in Keys::up_to($n).zip(actions) {
                    program.insert(key, action);
                }
                program
            });
            CompletePrograms {
                iterator: Box::new(iterator),
            }
        }
    };
}

all_programs!(1, all1);
all_programs!(2, all2);
all_programs!(3, all3);
all_programs!(4, all4);
all_programs!(5, all5);
```
---

| | 0 | 1 | 2 | 3 | 4 |
|-|---|---|---|---|---|
|_| H | ? | ? | ? | ? |
|I| ? | ? | ? | ? | ? |

---

| | 0 | 1 | 2 | 3 | 4 |
|-|---|---|---|---|---|
|_|IR4| ? | ? | ? | ? |
|I| ? | ? | ? | ? | ? |

---

| | 0 | 1 | 2 | 3 | 4 |
|-|---|---|---|---|---|
|_|IR1| ? | ? | ? | ? |
|I| ? | ? | ? | ? | ? |

---

```rust
/// A complete program
#[derive(Debug, PartialEq, Eq)]
pub struct CompleteProgram {
    program: Vec<Action>,
}
```

---

```rust
/// An incomplete program
#[derive(Debug, PartialEq, Eq)]
pub struct IncompleteProgram {
    n: u8,
    program: Vec<Option<Action>>,
}
```

---

```rust
/// The result of looking up a certain key in a program.
pub enum Lookup {
    /// The key is not known to the program. A semantic error
    Unknown,
    /// The key is not known to the program.
    /// But the program can be extended with the lookup key
    Indeterminate,
    /// The determined action for this key
    Determined(Action),
}
```

---

```rust
impl Program for IncompleteProgram {
    fn lookup(&self, key: &Key) -> Lookup {
        let idx = key.idx();
        match self.program.get(idx) {
            Some(Some(action)) => Lookup::Determined(*action),
            Some(None) => Lookup::Indeterminate,
            None => Lookup::Unknown,
        }
    }
}
```

---
.turing-machine-description[{
  "tm": {
    "tape": {
      "left": [],
      "current": "I",
      "right": ["I", "I", "I", "I", "I", "I", "I", "I"]
    },
    "state": 0,
    "transitions": [
      { "current": [0, "_"], "next": [1, "I", "R"] },
      { "current": [0, "I"], "next": [0, "I", "R"] }
    ]
  },
  "blank": "_",
  "visible_tape": 4,
  "running": false
}]

---

```rust
/// A `Tape` implementation that
/// use a run-length encoding of symbols
#[derive(Debug, PartialEq, Eq)]
pub struct CompoundTape {
    right: Vec<(Symbol, Occurrence)>,
    left: Vec<(Symbol, Occurrence)>,
}
```

--

```rust
enum Occurrence {
    Infinite,
    Finite(usize),
}
```

---

```rust
impl CompoundTape {
    /// Create an empty tape
    pub fn empty() -> Self {
        Self {
            right: vec![(Symbol::Blank, Occurrence::Infinite)],
            left: vec![(Symbol::Blank, Occurrence::Infinite)],
        }
    }
}
```

---
## Discrimination

---
## Let's get ph~~ys~~*ilosoph*ical

---
## Goldbach Conjecture

> Each even natural number bigger then 2 is the sum of two primes

---
## Take away

---
## Attribution

[PhD school in pictures](https://matt.might.net/articles/phd-school-in-pictures/)