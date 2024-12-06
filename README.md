# aoc

## Description

This repo will track my progression through `Advent of Code 2024`.  
This year, I'll try learning `Rust` :)

## Days

### Day 06

#### Part. 2

When walking in a `direction` and hitting a block (`#`), mark the whole line (from behind (`start`) to now (`end`), surrounded by `#` or `EOL`), either it is vertical or horizontal as `[start; end] direction`.  
*Note: the order of `start` and `end` doesn't matter, but we'll chose to pick our `current pos` as `end`*

The idea is that when crossing a line, if the line with `direction` is `90°` from our current direction, adding a block in front of us we'll force us into a loop.

Example : 

```
....
>..#
#...
..#.
```

The guard will go from :
1. `(0, 1)` to `(2, 1)` (`RIGHT`), marking `[(0, 1); (2, 1)] right` as a line
2. `(2, 1)` to `(2, 2)` (`BOTTOM`), marking `[(2, 0); (2, 2)] bottom` as a line
3. `(2, 2)` to `(1, 2)` (`LEFT`), marking `[(3, 2); (1, 2)] left` as a line
4. `(1, 2)` to `(1, 0)` (`TOP`), marking nothing because we reached the end **BUT** crossing the `[(0, 1); (2, 1)] right` line (in `(1, 1)`). Since `Right` = `Top + 90°`, we can infer that putting a block in `(1, 0)` would make us loop.

## Links

[Advent of Code](https://adventofcode.com/)
