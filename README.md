# lambda-calculus-rs

An untyped lambda calculus interpreter made to learn the foundations 
of lambda calculus and functional programming.

## Background

Lambda(λ) calculus is a formal system developed by [Alonzo Church](https://en.wikipedia.org/wiki/Alonzo_Church) 
to express mathematical functions. It is the foundation of functional programming languages. 
Lambda calculus (function-based) and Turing machines (state-based) are both universal models of computation, 
part of the [Church-Turing Thesis](https://en.wikipedia.org/wiki/Church%E2%80%93Turing_thesis).

See [grammar.ebnf](grammar.ebnf) for [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) grammar of untyped lambda calculus.

## Examples

```rs
// TODO:

// church booleans
// church numerals
// data structures
// y combinator?

// fibonacci sequence
```

## Commands

- Run - `cargo run`
- Build release - `cargo build --release`

## References

- [An Introduction to Functional Programming Through Lambda Calculus. Michaelson (9780486478838)](https://isbnsearch.org/isbn/9780486478838)
- [Computerphile - Lambda Calculus](https://www.youtube.com/watch?v=eis11j_iGMs&ab_channel=Computerphile)
- [Computerphile - Y Combinator](https://www.youtube.com/watch?v=9T8A89jgeTI&ab_channel=Computerphile)
- [Church Encoding](https://en.wikipedia.org/wiki/Church_encoding)
- [Untyped Lambda Calculus slides](https://www3.cs.stonybrook.edu/~cram/cse526/Spring20/Lectures/untyped-lambda.pdf)
