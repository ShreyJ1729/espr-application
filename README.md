# Ancient Treasure Puzzle

https://espr.camp/
https://pair.camp/

You stumble upon an ancient treasure locked behind a convoluted puzzle mechanism: a rotating box with 4 identically looking buttons on it.

Each button has two states - on and off - but it's impossible to distinguish these states from the outside. You can simultaneously press any number of buttons, but immediately after that the box rotates really fast and you lose track of which button ends up where.

You get the treasure if at any moment all the buttons are off.
Can you guarantee this happening? Explain your reasoning.

## Solution

Set of all states:

- all off = 0000
- all on = 1111
- one on: 1000, 0100, 0010, 0001
- two on:
- - opposite = 1010, 0101
- - adjacent = 1100, 0110, 0011, 1001
- three on = 1110, 1101, 1011, 0111

Set of all steps:

- press any one button
- press any two diagonally opposite buttons
- press any two adjacent buttons
- press any three buttons
- press all buttons

First case of 0000 is already taken care of. Note that pressing all buttons keeps the states in their same categories.

Steps to solve:

1. press all buttons

- solved: all on = 1111

2. press any two diagonally opposite buttons, and then press all buttons

- solved: two on opposite = 1010, 0101
- - one on --> (one on) or (three on)
- - two on adjacent --> two on adjacent
- - three on --> (one on) or (three on)

3. press any two adjacent buttons, and then press all buttons

- solved: half of two on adjacent = 1100, 0011
- - one on --> (one on) or (three on)
- - other half of two on adjacent --> two on opposite
- - three on --> (one on) or (three on)

4. press any two diagonally opposite buttons, and then press all buttons

- solved: two on opposite
- - one on --> (one on) or (three on)
- - three on --> (one on) or (three on)

5. press any one button, then press all buttons

- - one on --> two on
- - three on --> two on

6. press any two diagonally opposite buttons, and then press all buttons

- solved: two on opposite
- - two on adjacent --> two on adjacent

7. press any two adjacent buttons, and then press all buttons

- solved: half of two on adjacent
- - other half of two on adjacent --> two on opposite

8. press any two diagonally opposite buttons, and then press all buttons

- solved: two on opposite

Done!

We can represent the solution through chaining XOR operations on the initial state with random bitshifting after each XOR.
