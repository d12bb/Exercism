package rotationalcipher

func RotationalCipher(plain string, shiftKey int) string {
	res := make([]rune, 0, len(plain))
	key := rune(shiftKey % 26)

	for _, r := range plain {
		if r >= 'a' && r <= 'z' {
			c := r+key
			if c > 'z' {
				c -= 26
			}
			res = append(res, c)
		} else if r >= 'A' && r <= 'Z' {
			c := r+key
			if c > 'Z' {
				c -= 26
			}
			res = append(res, c)
		} else {
			res = append(res, r)
		}
	}
	return string(res)
}
