package divideconquer

import "testing"

func Test_divideConquerSum(t *testing.T) {
	type args struct {
		arr []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test-1",
			args: args{
				arr: []int{1, 3, 5, 7},
			},
			want: 16,
		},
		{
			name: "test-2",
			args: args{
				arr: []int{1, 3, 5},
			},
			want: 9,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := divideConquerSum(tt.args.arr); got != tt.want {
				t.Errorf("divideConquerSum() = %v, want %v", got, tt.want)
			}
		})
	}
}
