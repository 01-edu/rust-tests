## expected_variable

### Instructions

Create a function `expected_variable` that receives two strings: one to be evaluated and the other to be compared to (expected). Every comparison should be case insensitive.

If the evaluated string is not in camel case or in snake case according to the `case` crate that you should use, `expected_variable` returns "Err".
Otherwise the evaluated string should be compared to the expected string using the `edit_distance` function that you did in one of the previous quests.

If the result of `edit_distance` has more than 50% of 'alikeness' to the expected string, considering the length of the expected string and the result of `edit_distance`, return that same percentage with a '%' symbol in front of the number.
Otherwise `expected_value` should return "Err".

Example:

```rs
edit_distance("On_Point", "on_point"); // -> 100%
edit_distance("soClose", "So Close"); // -> 88% || 100 - (edit_distance(<both input strings>) * 100 / "So Close".len())
edit_distance("something", "something_completely_different"); // -> Err

```
