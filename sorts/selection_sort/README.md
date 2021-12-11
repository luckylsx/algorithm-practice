## 选择排序

时间复杂度：O(n^2)

排序描述：

选出一个最小的数字 假设为 index 为 0处，即第0个数字
在 1 ～ n 选一个最小的数字，与 0处交换，小数在前 大数在后 第一个位置确定
在 2 ～ n 选一个小的数字，与1处比较交换，小数在前 大数在后 第二个位置确定
......
在 n-2 ～ n-1 选一个小的数字，与n-2处比较交换，小数在前 大数在后 第n-2个位置确定

如：4，3，2，1    需要3次循环比较：
* 第1次循环:  0~n  => 1, 3, 2, 4 确定第0位
* 第2次循环:  1~n  => 1, 2, 3, 4 确定第1位
* 第3次循环: n-1~n => 1, 2, 3, 4 确定n-1位

动态演示：

https://www.cs.usfca.edu/~galles/visualization/ComparisonSort.html