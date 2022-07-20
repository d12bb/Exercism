package beer

import (
	"errors"
	"fmt"
)

const (
	regular = `%v of beer on the wall, %v of beer.
Take %v down and pass it around, %v of beer on the wall.
`
	rebuy = `No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
`
)

func Song() string {
	verses, _ := Verses(99, 0)
	return verses
}

func Verses(start, stop int) (string, error) {
	if start < stop {
		return "", errors.New("no drinking backwards")
	}
	out := ""
	for i := start; i>= stop; i-- {
		verse, err := Verse(i)
		if err != nil {
			return "", err
		}
		out = out + verse + "\n"
	}
	return out, nil
}

func Verse(n int) (string, error) {
	if n > 99 {
		return "", errors.New("99 bottles max")
	}
	if n < 0 {
		return "", errors.New("no chalking up")
	}
	switch n {
	case 0:
		return rebuy, nil
	default:
		return fmt.Sprintf(regular, bottles(n), bottles(n), one_it(n), bottles(n-1)), nil
	}
}

func bottles(n int) string {
	switch n {
	case 0:
		return "no more bottles"
	case 1:
		return "1 bottle"
	default:
		return fmt.Sprintf("%v bottles", n)
	}
}

func one_it(n int) string {
	if n == 1 {
		return "it"
	}
	return "one"
}
