layout: true
class: middle, center

---
# Uncomputable Functions

---
## &#129504;&#129657;

---
background-image: url(image/alice-queen.jpg)
background-position: center
background-size: contain

> Alice laughed. “There’s no use trying,” she said: “one _can’t_ believe impossible things.”
>
> “I daresay you haven’t had much practice,” said the Queen. “When I was
> your age, I always did it for half-an-hour a day. Why, sometimes I’ve
> believed as many as six impossible things before breakfast.”

---
background-image: url(image/alliander.png)
background-position: center
background-size: contain

---

> Een geflopt ICT-systeem kost de belastingdienst ruim 200 miljoen euro

---
## Wikipedia

[List of failed and overbudget custom software projects](https://en.wikipedia.org/wiki/List_of_failed_and_overbudget_custom_software_projects)

---
background-image: url(image/eniac.jpg)
background-position: center
background-size: contain

---
background-image: url(image/al-khwarizmi.jpg)
background-position: center
background-size: contain

--
## .caption[Muhammad ibn Musa al-Khwarizmi]

---
## \\(\frac{-b\pm\sqrt{b^{2}+4ac}}{2a}\\)

---
## \\(ax^2+bx+c=0\\)

---
* \\(ax^2+bx+c=0\\)
* \\(ax^2+bx=c\\)
* \\(ax^2+c=bx\\)
* \\(ax^2=bx+c\\)

---
> We are seeking a quantity such that a square of this size to which is added an area of 1887
> is equal to a rectangle with dimensions 88 and the unknown quantity.

---
background-image: url(image/tartaglia.jpg)
background-position: center
background-size: contain

---
background-image: url(image/koch.png)
background-position: center
background-size: contain

---
background-image: url(image/hilbert.jpg)
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
## &#129300;

---
## _Universal_ Turing Machine

--

> a Turing machine that can simulate an arbitrary Turing machine on arbitrary input

---
## Halting Problem

---
## Undecidable

--

> There is **no** Turing machine that accurately decides if a TM will halt on input I

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

> What is the maximum number of steps a Turing Machine with \\(n\\) states can take before it halts.

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
background-image: url(image/how-it-is-going.jpg)
background-position: center
background-size: contain

## &#129327;

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

| | 0      | 1 | 2 | 3 | 4 |
|-|--------|---|---|---|---|
|_|IR1| ? | ? | ? | ? |
|I| ?      | ? | ? | ? | ? |

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
## Discrimination

---
## Let's get ph~~ys~~*ilosoph*ical

---
## Goldbach Conjecture

> Each even natural number bigger then 2 is the sum of two primes

---
## Take away
