package luhn

func Valid(id string) bool {
	nums := make([]int32, 0, len(id))
	for _, num := range id {
		d := num - '0'
		if d >= 0 && d < 10 {
			nums = append(nums, d)
			continue
		}
		if num != ' ' {
			return false
		}
	}

	if len(nums) <= 1 {
		return false
	}

	for i := len(nums) - 2; i >= 0; i -= 2 {
		tmp := nums[i] * 2
		if tmp > 9 {
			tmp -= 9
		}
		nums[i] = tmp
	}

	return sum(nums)%10 == 0
}

func sum(nums []int32) (sum int32) {
	for _, n := range nums {
		sum += n
	}
	return sum
}
