package binarySearch

func binarySearch(arr []int, target int) int {
	if len(arr) == 0 {
		return -1
	}
	low, heigh := 0, len(arr)-1
	for low <= heigh {
		middlen := (heigh + low) >> 1
		if arr[middlen] < target {
			low = middlen + 1
		} else if arr[middlen] > target {
			heigh = middlen - 1
		} else {
			return middlen
		}
	}
	return -1
}
