package bubble

import (
	"reflect"
	"testing"
)

func Test_bubbleSort(t *testing.T) {
	type args struct {
		arr []int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "test#1",
			args: args{
				arr: []int{90,100,8,7,17,29,10},
			},
			want: []int{7,8,10,17,29,90,100},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := bubbleSort(tt.args.arr); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("bubbleSort() = %v, want %v", got, tt.want)
			}
		})
	}
}