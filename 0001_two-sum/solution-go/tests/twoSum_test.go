package tests

import (
	"slices"
	"testing"
	"solution-go/providers"
)


func TestTwoSum(t *testing.T) {
	type args struct {
		nums []int
		target int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{"basic", args{[]int{2, 7, 11, 15}, 9}, []int{0, 1}},
		{"no-solution", args{[]int{1, 2, 3}, 100}, nil},
		{"duplicates", args{[]int{3, 3}, 6}, []int{0, 1}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := twoSum.TwoSum(tt.args.nums, tt.args.target)
			if !slices.Equal(got, tt.want) {
				t.Fatalf("TwoSum(%v, %d) = %v; want %v", tt.args.nums, tt.args.target, got, tt.want)
			}
		})
	}
}
