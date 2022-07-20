package linkedlist

import "errors"

type List struct {
	head, tail *Node
}

type Node struct {
	Value      interface{}
	next, prev *Node
}

func NewList(args ...interface{}) *List {
	list := &List{}

	for _, val := range args {
		list.Push(val)
	}

	return list
}

func (n *Node) Next() *Node {
	return n.next
}

func (n *Node) Prev() *Node {
	return n.prev
}

func (l *List) Unshift(v interface{}) {
	node := &Node{Value: v}
	if l.head != nil {
		node.next = l.head
		l.head.prev = node
	} else {
		l.tail = node
	}
	l.head = node
}

func (l *List) Push(v interface{}) {
	node := &Node{Value: v}
	if l.tail != nil {
		node.prev = l.tail
		l.tail.next = node
	} else {
		l.head = node
	}
	l.tail = node
}

func (l *List) Shift() (interface{}, error) {
	if l.head == nil {
		return nil, errors.New("empty list")
	}

	val := l.head.Value
	l.head = l.head.next

	if l.head == nil {
		l.tail = nil
	} else {
		l.head.prev = nil
	}

	return val, nil
}

func (l *List) Pop() (interface{}, error) {
	if l.tail == nil {
		return nil, errors.New("empty list")
	}

	val := l.tail.Value
	l.tail = l.tail.prev

	if l.tail == nil {
		l.head = nil
	} else {
		l.tail.next = nil
	}

	return val, nil
}

func (l *List) Reverse() {
	cur := l.tail
	for cur != nil {
		cur.prev, cur.next = cur.next, cur.prev
		cur = cur.next
	}
	l.head, l.tail = l.tail, l.head
}

func (l *List) First() *Node {
	return l.head
}

func (l *List) Last() *Node {
	return l.tail
}
