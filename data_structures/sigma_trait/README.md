# Permutable Trait and Sigma Function

This implementation provides a `Permutable` trait for `Vec<i32>` that allows for permuting the elements of the vector. The permutation is implemented as a rotation of the elements to the left by one position.

The `PermutedList` struct stores both the original and permuted lists, and provides a `sigma` function that maps an index `i` in the original list to its corresponding index in the permuted list.

## Sigma Function

The `sigma` function is defined as:

$$\sigma(i) = j$$

where $i$ is an index in the original list, and $j$ is the index of the same element in the permuted list.

In other words, `sigma(i)` returns the new position of the element at index `i` in the original list after permutation.

## Example

Given the original list `[1, 2, 3, 4, 5]`, the permuted list is `[2, 3, 4, 5, 1]`. The $$\sigma$$ function can be used to map indices between the two lists:

- `$$\sigma(0) = 4$$ (the element at index 0 in the original list is at index 4 in the permuted list)
- `$$\sigma(1) = 0$$ (the element at index 1 in the original list is at index 0 in the permuted list)
- `$$\sigma(2) = 1$$ (the element at index 2 in the original list is at index 1 in the permuted list)
- `$$\sigma(3) = 2$$ (the element at index 3 in the original list is at index 2 in the permuted list)
- `$$\sigma(4) = 3 $$ (the element at index 4 in the original list is at index 3 in the permuted list)

## Code

The implementation is provided in the `permutable.rs` file. The `Permutable` trait is defined for `Vec<i32>`, and the `PermutedList` struct is used to store the original and permuted lists. The `sigma` function is implemented as a method on the `PermutedList` struct.

You can use the `permute` method to create a `PermutedList` instance from a `Vec<i32>`, and then call the `sigma` function to map indices between the original and permuted lists.
