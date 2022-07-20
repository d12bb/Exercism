package matrix

import (
	"errors"
	"strconv"
	"strings"
)

type Matrix [][]int

func New(s string) (*Matrix, error) {
	m := Matrix{}
	size := -1
	for _, line := range strings.Split(s, "\n") {
		l := []int{}
		for _, col := range strings.Split(line, " ") {
			if col == "" {
				continue
			}
			num, err := strconv.Atoi(col)
			if err != nil {
				return nil, err
			}
			l = append(l, num)
		}
		if size >= 0 && size != len(l) {
			return nil, errors.New("invalid matrix")
		}
		size = len(l)
		m = append(m, l)
	}
	return &m, nil
}

// Cols and Rows must return the results without affecting the matrix.
func (m *Matrix) Cols() [][]int {
	out := [][]int{}
	for ci := 0; ci < len((*m)[0]); ci++ {
		col := []int{}
		for ri := 0; ri < len(*m); ri++ {
			col = append(col, (*m)[ri][ci])
		}
		out = append(out, col)
	}
	return out
}

func (m *Matrix) Rows() [][]int {
	out := [][]int{}
	for i := 0; i < len(*m); i++ {
		out = append(out, append([]int{}, (*m)[i]...))
	}
	return out
}

func (m *Matrix) Set(row, col, val int) bool {
	if row >= 0 && col >= 0 && len(*m) > row && len((*m)[row]) > col {
		(*m)[row][col] = val
		return true
	}
	return false
}
