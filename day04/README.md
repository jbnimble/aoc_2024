# Advent of Code 2024 Day 4

Setup

```bash
cargo add regex
cargo add unicode-segmentation
```

Execute code

```bash
cargo run -- data.txt
```

Part 1 test

5 h
3 v
10 d
18 total

```
....1XMAS.
.SAMXMS...
...S..A...
..A.A.MS.1
XMASAMX.MM
1.....1A.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.1.2.2MAS1
```
dr
ul ul ul ul

dl dl
ur ur ur

```bash
# original
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

# down left, up right = 5
..A.
X..MX
XMASXX
S.A..MM
...SA.SA
.AS.A.A.S
.....MM...
XMAS.XS..
....X...
XMASAMX
.....M
XMASA

# down right, up left
M..X
XS..M
XMASM
.X.M..S
.M.XASA
.ASAMX..M
.S..A.SAMX
..AS....S
...A.SAMA
.M....M
X.SAMX
X....
SAMX
```