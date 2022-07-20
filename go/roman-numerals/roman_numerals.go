package romannumerals

import "errors"

var (
	roman = map[int][]rune{
		1000: {'M'},
		900:  {'C', 'M'},
		500:  {'D'},
		400:  {'C', 'D'},
		100:  {'C'},
		90:   {'X', 'C'},
		50:   {'L'},
		40:   {'X', 'L'},
		10:   {'X'},
		9:    {'I', 'X'},
		5:    {'V'},
		4:    {'I', 'V'},
		1:    {'I'},
	}
	// range(map) order not guaranteed
	order = []int{1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1}
)

// ToRomanNumeral function    converts an integer into it's roman numeral representation.
// Input allowed in range from 1 to 3000 (both inclusive).
func ToRomanNumeral(input int) (string, error) {
	if input < 1 || input > 3000 {
		return "", errors.New("only numbers 1..3000 allowed")
	}

	out := make([]rune, 0, 14) // longest possible in range is MMDCCCLXXXVIII with len 14
	for _, k := range order {
		for input/k >= 1 {
			out = append(out, roman[k]...)
			input -= k
		}
	}

	return string(out), nil
}
