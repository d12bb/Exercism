package prime

import "errors"

// Nth returns the nth prime number. An error must be returned if the nth prime number can't be calculated ('n' is equal or less than zero)
func Nth(n int) (int, error) {
	if n < 1 {
		return 0, errors.New("input must be greater than 1")
	}
	if n == 1 {
		return 2, nil
	}

	var i int
	n--
	for i = 3; n > 0; i += 2 {
		if prime(i) {
			n--
		}
	}
	return i-2, nil
}

func prime(n int) bool {
	for i := 3; i * i <= n; i += 2 {
		if n % i == 0 {
			return false
		}
	}
	return true
}
