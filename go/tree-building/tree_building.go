package tree

import (
	"errors"
	"sort"
)

type Record struct {
	ID     int
	Parent int
}

type Node struct {
	ID       int
	Children []*Node
}

func Build(records []Record) (*Node, error) {
	if len(records) == 0 {
		return nil, nil
	}

	sort.Slice(records, func(i, j int) bool { return records[i].ID < records[j].ID })
	nodes := make([]*Node, len(records))

	for _, r := range records {
		if r.ID >= len(nodes) {
			return nil, errors.New("non-continuous IDs")
		}
		if nodes[r.ID] != nil {
			return nil, errors.New("duplicate ID")
		}
		if r.ID == 0 && r.Parent != 0 {
			return nil, errors.New("root node has parent")
		}

		nodes[r.ID] = &Node{ID: r.ID}

		if r.ID == 0 {
			continue
		}
		if r.ID <= r.Parent {
			return nil, errors.New("cycle detected")
		}

		if nodes[r.Parent].Children == nil {
			nodes[r.Parent].Children = []*Node{nodes[r.ID]}
		} else {
			nodes[r.Parent].Children = append(nodes[r.Parent].Children, nodes[r.ID])
		}
	}

	if nodes[0] == nil {
		return nil, errors.New("no root node")
	}

	return nodes[0], nil
}
