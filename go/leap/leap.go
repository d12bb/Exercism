package leap

// IsLeapYear returns whether a given year is a leap year.
func IsLeapYear(year int) bool {
	if year % 4 != 0   { return false }
	if year % 100 != 0 { return true }
	if year % 400 != 0 { return false }
	return true
}
