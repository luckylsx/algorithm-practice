## 题目

使用分而治之思想求和数组 [1,3,5,7,9,12]

## 思路

* 基线条件： 空数组[], 其和为0
* 递归：[1,3,5,7,9,12]
    * 1 + sum([3,5,7,9,12])
    * 3 + sum([5,7,9,12])
    * 5 + sum([7,9,12])
    * 7 + sum([9,12])
    * 9 + sum([12])
    * 12 + sum([])