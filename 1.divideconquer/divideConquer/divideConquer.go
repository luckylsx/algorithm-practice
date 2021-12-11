package divideconquer

func divideConquerSum(arr []int) int {
	if len(arr) == 0 {
		return 0
	}
	return arr[0] + divideConquerSum(arr[1:])
}
