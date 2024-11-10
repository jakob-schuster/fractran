A little Rust implementation of Conway's Fractran, specifically [devine's version of it](https://wiki.xxiivv.com/site/fractran.html).

# Semantics

A Fractran program is made up of:
1. An initial program state, essentially a bag of words.
1. A series of rules, each one representing a trade which consumes some words to generate some other words.

When you run the program, the first rule that can be applied is applied, mutating the program state. This loops until no more rules can be applied, reaching a final state. For a proper explanation, read [devine's blog post](https://wiki.xxiivv.com/site/fractran.html)!

# Syntax

## Words

A word is a series of letters. The only non-alphabetic character allowed is a hyphen `-`.

```
apple-pie
```

## Rules

Rules are separated by newlines. Each rule starts with two colons `::`, with a left side and a right side separated by a right-angle-bracket `>`. Either side of the rule consists of a space-separated list of words. Words on the left are consumed, generating the words on the right.
```
:: log > plank plank plank plank
```
To write succintly, you can use the caret `^` to repeat a word a number of times.
```
:: log > plank^4
```

## Program state

The program state is just a space-separated list of words.
```
log plank^3
```