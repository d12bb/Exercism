package anagram

import "strings"

func Detect(subject string, candidates []string) []string {
	out := make([]string, 0, len(candidates))
	for _, candidate := range candidates {
		if isAnagram(strings.ToLower(subject), strings.ToLower(candidate)) {
			out = append(out, candidate)
		}
	}
	return out
}

func isAnagram(s1, s2 string) bool {
	if s1 == s2 {
		return false
	}

	letters := make(map[rune]int)

	for _, c := range s1 {
		letters[c]++
	}
	for _, c := range s2 {
		if _, ok := letters[c]; ok {
			letters[c]--
		} else {
			return false
		}
	}

	for _, l := range letters {
		if l != 0 {
			return false
		}
	}

	return true
}
