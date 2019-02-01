# Permutation Visualiser

An attempt to see patterns in repetition and thus gain an insight into the structure of minimal super permutations.

## Algorithm

The input string of the superpermutation, length n, is used to label rows and columns of 
an n x n matrix whose cells contain a Some(<char_n>) or None depending on the comparisson:

  ```
  if label(A[x][y]) == label(A[a][b]) {
    Some(label(A[x][y])
  } else {
    None
  }
  ```

this is symmetric, with the diagonal all being filled so can be reduced to a Vec<Vec<Char>>
with each inner vector's length equal to it's position in the input string.

eg: n = 3 : abcabacba

=> [
     [ Some(a) ],
     [ Some(b), None ],
     [ Some(c), None, None ],
     [ Some(a), None, None, Some(a) ],
     [ Some(b), None, None, Some(b), None ],
     [ Some(a), None, Some(a), None, None, Some(a) ],
     [ Some(c), None, None, None, Some(c), None, None, None ],
     [ Some(b), None, None, Some(b), None, None, Some(b), None ],
     [ Some(a), None, None, Some(a), None, Some(a), None, None, Some(a) ]
   ]

this is nice becasue we can pre-allocate the Vector space and generate the array as we parse the
input string, going backward from the posiiton we currently occupy, inserting Some's into a pre
-populated None Vector.
We can then draw a window on the screen displaying the nxn pixels designated by the occupied spaces
in the Vec<Vec<_>>. Color coding can be done as we have the labels stored.

Pretty simple algorithm actually.
