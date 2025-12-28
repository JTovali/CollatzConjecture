# Collatz Conjecture

# Description:
This project is about the Collatz Conjecture. An intriguing unsolved problem in mathematics defined by a simple rule:
Take any positive integer n, divide it by 2 if it is even, or compute 3n+1 if it is odd, and repeat. The sequence can grow unpredictably before reaching 1. The purpose of this project is to compute Collatz sequence lengths for very large input ranges and to identify the top 10 smallest integers, but with the longest sequence length. This project features 9 distinct implementations of the Collatz Conjecture, including 4 recursive versions, and 5 iterative versions.


# Languages used in Collatz Conjecture project:
```
	- Python
	- Julia
	- Fortran
	- Lisp
	- Rust
```

# Files included:
```
	- Python 
	- Julia        / juliaRecursed
	- Fortran      / fortranRecursion
	- Lisp         / lispRecursion
	- Rust/collatz / rustRecursion/collatz 
```
# ----------------------------------------------------------------------------------

# Steps on how to compile and run each language:

# how to compile & run for iterative version:

# Python:
```
To compile & run:	python3 collatz.py 1st-positive-integer 2nd-positive-integer
```

# Julia:
```
To compile & run:	julia collatz.jl 1st-positive-integer 2nd-positive-integer
```

# Fortran:

```
To compile:		gfortran collatz.f90
To run:			./a.out 1st-positive-integer 2nd-positive-integer
```
# Lisp:
``` 
To compile and run: 	sbcl --script collatz.lisp 1st-positive-integer 2nd-positive-integer

```

# Rust:
```
To compile:		cargo build
To run:			cargo run 1st-positive-integer 2nd-positive-integer

```

# how compile & run for recursive version:

# juliaRecursed:

```
To compile & run:	julia recursion_collatz.jl 1st-positive-integer 2nd-positive-integer
```

# fortranRecursion:
```
To compile:		gfortran recursion_collatz.f90 
To run:			./a.out 1st-positive-integer 2nd-positive-integer
```

# lispRecursion:
```
To compile & run:	sbcl --script recursion_collatz.lisp 1st-positive-integer 2nd-positive-integer
```

# rustRecursion/collatz:
```
To compile:		cargo build
TO run:			cargo run 1st-positive-integer 2nd-positive-integer
```
# Authors and acknowledgment
Dr. Pounds
Jaime Tovali

# License
No license has been provided for this project

