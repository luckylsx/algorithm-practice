package binarySearch

import "testing"

func Test_binarySearch(t *testing.T) {
	type args struct {
		arr    []int
		target int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test#1",
			args: args{
				arr:    []int{7, 8, 10, 17, 29, 90, 100},
				target: 0,
			},
			want: -1,
		},
		{
			name: "test#2",
			args: args{
				arr:    []int{7, 8, 10, 17, 29, 90, 100},
				target: 10000,
			},
			want: -1,
		},
		{
			name: "test#3",
			args: args{
				arr:    []int{7, 8, 10, 17, 29, 90, 100},
				target: 100,
			},
			want: 6,
		},
		{
			name: "test#4",
			args: args{
				arr:    []int{7, 8, 10, 17, 29, 90, 100},
				target: 90,
			},
			want: 5,
		},
		{
			name: "test#5",
			args: args{
				arr:    []int{7, 8, 10, 17, 29, 90, 100},
				target: 10,
			},
			want: 2,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := binarySearch(tt.args.arr, tt.args.target); got != tt.want {
				t.Errorf("binarySearch() = %v, want %v", got, tt.want)
			}
		})
	}
}
