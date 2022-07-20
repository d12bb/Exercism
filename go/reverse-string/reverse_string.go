package reverse

import "unicode/utf8"

func Reverse(input string) string {
	rev := make([]rune, 0, len(input))
	for len(input) > 0 {
		r, size := utf8.DecodeLastRuneInString(input)
		rev = append(rev, r)
		input = input[:len(input)-size]
	}
	return string(rev)
}
