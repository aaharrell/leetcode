package twoSum


func TwoSum(nums []int, target int) []int {
	valueMap := make(map[int]int)
	for i := range nums {
		needed := target - nums[i]
		_, ok := valueMap[needed]
		if ok {
			return []int{valueMap[needed], i}
		} else {
			valueMap[nums[i]] = i
		}
	}
	return nil
}
