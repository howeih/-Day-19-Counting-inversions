#Day 19: Counting inversions
<br>
Count of inversions in array is a textbook showcase of what is the difference between quadratic and linearithmic algorithm.
<br>
While naive method may select each tuple of items to be compared, more efficient approach is to adapt merge-sort to do the counting for you. As a result, function returns number of inversions and the sorted array as bonus.
<br>

Data:
```
[29, 25, 17, 15, 6, 8, 11, 15, 7, 5]
```
<br>
Result:
```
[5, 6, 7, 8, 11, 15, 15, 17, 25, 29], number of inversions:37
```