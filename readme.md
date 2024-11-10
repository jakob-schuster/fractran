A little Rust implementation of Conway's Fractran, specifically [devine's version of it](https://wiki.xxiivv.com/site/fractran.html).


# Syntax

The semantics are given on devine's blog, but I will document the syntax used by my implementation. 

A Fractran program is made up of:
1. A series of rules
2. An initial program state

## Rules

Each rule starts with two colons `::`, with a left side and a right side separated by a right-angle-bracket `>`. Either side of the rule consists of a space-separated list of names.
```
:: log > plank plank plank plank
```
To write more succintly, you can use the caret `^` to repeat a word a number of times.
```
:: log > plank^4
```

## Program state

The program state is just a space-separated list of names.
```
log plank^3
```