package armstrong

import "math"

func IsNumber(n int) bool {
	tmp := n
	digits := []int{}
	for tmp > 0 {
		digits = append(digits, tmp%10)
		tmp /= 10
	}
	pow := float64(len(digits))
	sum := 0
	for i := range digits {
		sum += int(math.Pow(float64(digits[i]), pow))
	}
	return sum == n
}
