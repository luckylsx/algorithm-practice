## 冒泡排序

时间复杂度：O(n^2)

相邻两个数比较，小的往前，大的往后，每次循环确定最后大数字

如：4，3，2，1    需要4次循环比较：
1.  3, 4, 2, 1;  3, 2, 4, 1;  3, 2, 1, 4  确定4
2.  2, 3, 1, 4;  2, 1, 3, 4      确定 3
3.  1, 2, 3, 4  确定 2 和 1

动态演示：

https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html