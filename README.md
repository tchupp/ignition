# Ignition
[![Build Status](https://travis-ci.org/tchupp/ignition.svg?branch=master)](https://travis-ci.org/tchupp/ignition)

Ignition is a tool for building custom rules engines, tailored towards SAT solvers  

## Getting Started

A `rule engine` is built two parts, items and rules

### Items

#### Id

#### Attributes/Tags/Labels

```yaml
id: <unique_id>
attributes:
  - key: key_1      # attribute 1
    value: <value>
  - key: key_2      # attribute 2
    value: <value>
  - key: key_3      # attribute 3
    value: <value>
  - key: key_1      # overrides attribute 1
    value: <value>
```

### Rules

#### Id

#### Condition

```

```

### Example

#### Closet

For these examples, we will imagine the problem of choosing an outfit from a closet

A closet has families of clothing (ex. Shirts, Pants, etc.)  
A family of clothing has many possible items (ex. Shirts has red and black, Pants has jeans, etc.)

An outfit is a selection of items, one from each family  
In an outfit, you *must* have at exactly one selection per family (ex. You can't wear two shirts, You can't skip pants!)  
Rules may be defined that exclude or include certain items, given a specific selection (ex. You can't wear jeans with a blue shirt)

If we represent the selection of an item as a boolean (T for selected, F for not selected), each rule in the system can be defined by a logical binary operation  
The output of the binary operation is a boolean, telling us whether the combination is valid.

##### Items

An `item` is a single piece of clothing. The attributes are `color` and `family`.

```yaml
id: blue_shirt
attributes:
  - key: color      # attribute 1
    value: blue
  - key: family     # attribute 2
    value: shirt
```

##### Rules

###### Sibling relationship
A and B are items in the same family  
A and B may not be selected at the same time, but one of them **must** be selected

Equation (DNF): `(A * ~B) + (~A * B)`  

Truth table:

| A | B | V |
|:-:|:-:|:-:|
| 0 | 0 | 0 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

While this equation looks similar to an XOR, it doesn't scale in the same way  
Where a 3-input XOR function allows all three inputs to be `T` at the same time, our sibling relationship does not  
Lets look at the truth table for *three* items in the same family  

Equation (DNF): `(A * ~B * ~C) + (~A * B * ~C) + (~A * ~B * C)`  

Truth table:

| A | B | C | V |
|:-:|:-:|:-:|:-:|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 1 |
| 0 | 1 | 1 | 0 |
| 1 | 0 | 0 | 1 |
| 1 | 0 | 1 | 0 |
| 1 | 1 | 0 | 0 |
| 1 | 1 | 1 | 0 |

The relationship holds true for three items, and the equation will scale quadratically for each item added

###### Exclusion
A and B are items in different families  
Selection of A excludes the selection of B  
Selection of B excludes the selection of A  

Equation: `A -> ~B` or `~(A * B)`  
Equation (DNF): `~A + ~B`  

Truth table:

| A | B | V |
|:-:|:-:|:-:|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 1 |
| 1 | 1 | 0 |

###### Inclusion
A and B are items in different families  
Selection of A requires that B is also selected
Selection of B does not **require** that A is also selected

Equation: `A -> B` or `~(A * ~B)`  
Equation (DNF): `~A + B`  
**NOTE: A and B are *not* interchangeable**

Truth table:

| A | B | V |
|:-:|:-:|:-:|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |
