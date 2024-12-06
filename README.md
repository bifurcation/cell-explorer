Cellular Automata Explorer
==========================

This repo has a little program that explores cellular automamta.  It makes a
small screen and starts the automaton in a random state, following a random
elementary automaton rule.  If the automaton ends up in a uniform or periodic
state, the program picks a different rule and restarts the automaton in a fresh
random state.  If neither of those conditions causes a refresh after a certain
period of time, then we refresh anyway.

Quickstart
----------

```
> cargo run
```
