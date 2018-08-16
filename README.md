# Bowtie
[![Build Status](https://travis-ci.org/tclchiam/bowtie.svg?branch=master)](https://travis-ci.org/tclchiam/bowtie)

Bowtie is a library that helps us model Sparse Sets of Subsets Completions  

## Implementation

For these examples, we will imagine the problem of choosing an outfit from a closet

A closet has families of clothing (ex. Shirts, Pants, etc.)  
A family of clothing has many possible items (ex. Shirts has red and black, Pants has jeans, etc.)

An outfit is a selection of items, one from each family  
In an outfit, you *must* have at exactly one selection per family (ex. You can't wear two shirts, You can't skip pants!)  
Rules may be defined that exclude or include certain items, given a specific selection (ex. You can't wear jeans with a blue shirt)

If we represent the selection of an item as a boolean (T for selected, F for not selected), each rule in the system can be defined by a logical binary operation  
The output of the binary operation is a boolean, telling us whether the combination is valid.

## Sibling relationship
A and B are items in the same family  
A and B may not be selected at the same time, but one of them **must** be selected

Equation: `A ^ B`  
Equation (DNF): `(A * ~B) + (~A * B)`  

Truth table:

| A | B | V |
|:-:|:-:|:-:|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

## Exclusion
A and B are items in different families  
Selection of A excludes the selection of B  
Selection of B excludes the selection of A  

Equation: `~(A * B)`  
Equation (DNF): `~A + ~B`  

Truth table:

| A | B | V |
|:-:|:-:|:-:|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

## Inclusion
A and B are items in different families  
Selection of A requires that B is also selected
Selection of B does not **require** that A is also selected

Equation: `~(A * ~B)`  
Equation (DNF): `~A + B`  
**NOTE: A and B are *not* interchangeable**

Truth table:

| A | B | V |
|:-:|:-:|:-:|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |
