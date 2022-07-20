package clock

import "fmt"

type Clock int

func New(h, m int) Clock {
	return Clock(ensure_sanity(h*60 + m))
}

func (c Clock) Add(m int) Clock {
	return Clock(ensure_sanity(int(c) + m))
}

func (c Clock) Subtract(m int) Clock {
	return Clock(ensure_sanity(int(c) - m))
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", int(c)/60, int(c)%60)
}

func ensure_sanity(mins int) int {
	return (mins%1440 + 1440) % 1440
}
