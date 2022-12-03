# wyvern

An untyped lambda calculus interpreter made to learn the basics 
of lambda calculus and concepts of functional programming.

## Background

Lambda(λ) calculus is a formal system developed by [Alonzo Church](https://en.wikipedia.org/wiki/Alonzo_Church) 
to express mathematical functions and is the foundation of functional programming languages. 
Lambda calculus (function-based) and Turing machines (state-based) are both universal models of computation, 
part of the [Church-Turing Thesis](https://en.wikipedia.org/wiki/Church%E2%80%93Turing_thesis).

Grammar at a glance:

- Identifier: `x` - Identifier as expected in a programming language
- Grouping: `(x)` - Grouping terms to avoid ambiguity
- Abstraction: `λx.y` - Define function or lambda (anonymous function)
- Application: `x y` - Invoke a lambda

See [grammar.ebnf](grammar.ebnf) for [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) 
grammar of this untyped lambda calculus.

## Commands

- Run - `cargo run`
- Build release - `cargo build --release`

## References

- [An Introduction to Functional Programming Through Lambda Calculus. Michaelson (9780486478838)](https://isbnsearch.org/isbn/9780486478838)
- [Computerphile - Lambda Calculus](https://www.youtube.com/watch?v=eis11j_iGMs&ab_channel=Computerphile)
- [Computerphile - Y Combinator](https://www.youtube.com/watch?v=9T8A89jgeTI&ab_channel=Computerphile)
- [Untyped Lambda Calculus slides](https://www3.cs.stonybrook.edu/~cram/cse526/Spring20/Lectures/untyped-lambda.pdf)
- https://en.wikipedia.org/wiki/Church_encoding
- https://en.wikipedia.org/wiki/Beta_normal_form
- https://hbr.github.io/Lambda-Calculus/lambda2/lambda.html
- https://tadeuzagallo.com/blog/writing-a-lambda-calculus-interpreter-in-javascript/
- https://blog.shaynefletcher.org/2016/10/eliminating-left-recursion.html
- https://lucasfcosta.com/2018/07/29/An-Introduction-to-Lambda-Calculus-Part-1.html
- https://www.youtube.com/watch?v=3VQ382QG-y4
