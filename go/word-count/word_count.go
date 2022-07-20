package wordcount

import (
	"strings"
	"unicode"
)

type Frequency map[string]int

func WordCount(phrase string) Frequency {
	freqs := Frequency{}
	words := strings.FieldsFunc(phrase, func(r rune) bool {
		return !unicode.IsNumber(r) && !unicode.IsLetter(r) && r != '\''
	})

	for i := range words {
		words[i] = strings.ToLower(words[i])
		words[i] = strings.TrimFunc(words[i], func(r rune) bool {
			return !unicode.IsNumber(r) && !unicode.IsLetter(r)
		})
		freqs[words[i]]++
	}

	return freqs
}
