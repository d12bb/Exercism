package isbn

func IsValidISBN(isbn string) bool {
	var (
		count int32 = 10
		sum   int32
	)

	for _, r := range isbn {
		if r == '-' {
			continue
		}

		n := r - '0'
		if count == 1 && r == 'X' {
			n = 10
		}

		sum += count * n
		count--
	}
	if count != 0 {
		return false
	}
	return sum%11 == 0
}
