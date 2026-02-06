# Flamegraph staircase repro

Gungraun flamegraphs produce a linear 'staircase' instead of a branching call
tree, even when the call graph has sibling calls.

## Run Examples

```sh
make clean ; make repro fixed -j2
```

The generated flamegraphs are under `./repro` and `./fixed`.

## Example Structure

One peak of call depth 13, one peak of call depth 12. Every function has a
unique body (different work multiplier constant) to prevent the compiler from
deduplicating them. Each callee has a unique name to avoid coalescing.

At each level below `top`, three children are called:

- `aNN_c`
  - `a(NN+1)_l` left work (leaf)
  - `a(NN+1)_c` deeper call (next level)
  - `a(NN+1)_r` right work (leaf)

## Naming and ordering

Leaf functions use a 1:10:100 work multiplier ratio (small/medium/large bands).
Peak A has left = small, right = large; Peak B inverts this.

Peak A's cost-descending order (`_c`, `_r`, `_l`) disagrees with both call order
and lexical order. This asymmetry makes the repro a clear test of which ordering
the renderer actually uses.

### Output

Actual output: Linear staircase instead of a flamegraph, where every function is
nested under the previous one by descending cost.

Expected output: Branching folded stacks reflecting the two-peak call graph.
