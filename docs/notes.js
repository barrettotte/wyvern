// https://www.youtube.com/watch?v=3VQ382QG-y4

// grammar:
// expression :: = variable                   identifier
//               | expression expression      application
//               | λ variable . expression    abstraction
//               | ( expression )             grouping

// Idiot combinator - Identity
I = a => a
// I := λa.a

// Application
f(a)      // f a
f(a)(b)   // f a b      curried function
(f(a))(b) // (f a) b    optional parenthesis; Functions are left-associative
f(a(b))   // f (a b)    force order of execution

// Abstraction - Apply a function on the left to the argument on the right.
a => b           // λa.b
a => b(x)        // λa.b x
a => (b(x))      // λa.(b x)               useless parenthesis
(a => b)(x)      // (λa.b) x               force definition
a => b => a      // λa.λb.a    
a => (b => a)    // λa.(λb.a)              useless parenthesis
a => b => c => b // λa.λb.λc.b = λabc.b    shortened syntax

// β-Reduction - act of taking a function and applying to argument until done
//
// ((λa.a)λb.λc.b)(x)λe.f
// (λb.λc.b)      (x)λe.f     replaced all 'a' with 'λb.λc.b'
//    (λc.x)         λe.f     replaced all 'b' with 'x'
//        x                   replaced all 'c' with 'λe.f' (none)
// now in β-Normal Form

// Mockingbird combinator - self-application
M = f => f(f)
// M := λf.ff

// M M = M M = M M = ... halting problem!
//   also known as Ω combinator

// Kestrel combinator - first, const
K = a => b => a
// K := λa.λb.a == λab.a

// Kite combinator - second
KI = a => b => b
// KI := λa.λb.b == λab.b

// combinator - a function with no free variables
// λb.b         combinator; variable b is bound to the parameter b
// λb.a         not a combinator; variable a is not bound to parameter b
// λab.a        combinator
// λa.ab        not a combinator
// λabc.c(λe.b) combinator

// Cardinal combinator - reverses arguments
C = f => a => b => f(b)(a)
// C := λfab.fba

// Booleans
// T := K == λa.λb.a         true
// F := KI == λa.λb.b        false
// NOT := λp.pFT or C        negation
// AND := λpq.pqF or λpq.pqp conjunction
// OR := λpq.pTq or λpq.ppq  intersection
// EQ := λpq.p q (NOT q)     equality

// The famous Y combinator
// λf.(λx.f(xx))(λx.f(xx))
