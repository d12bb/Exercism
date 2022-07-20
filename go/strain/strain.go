package strain

type (
	Ints    []int
	Lists   [][]int
	Strings []string
)

func keep[Ts ~[]T, T int | []int | string](i Ts, filter func(T) bool) (out Ts) {
	if i == nil {
		return nil
	}
	for _, v := range i {
		if filter(v) {
			out = append(out, v)
		}
	}
	return out
}

func (i Ints) Keep(filter func(int) bool) Ints {
	return keep(i, filter)
}

func (l Lists) Keep(filter func([]int) bool) Lists {
	return keep(l, filter)
}

func (s Strings) Keep(filter func(string) bool) Strings {
	return keep(s, filter)
}

func (i Ints) Discard(filter func(int) bool) Ints {
	return i.Keep(func(x int) bool {
		return !filter(x)
	})
}
