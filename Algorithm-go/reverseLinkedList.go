package main

import "fmt"

// 节点
type ListNode struct {
  Val int
  Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
		var pre, cur *ListNode
		cur = head
		for cur != nil {
			tem := cur.Next
			cur.Next = pre
			pre, cur = cur, tem
		}
		return pre
}

func runReverseList() {
	var head = ListNode{Val: 0}
	var list = &head
	index := 0
	for i := 0; i < 10; i ++ {
		fmt.Println("index: ", index,"value: ", head.Val)
		l := ListNode{Val: i}
		list.Next = &l
		list = &l
		index ++
	}

	r := reverseList(&head)
	fmt.Println("\n Reversed: ")
	index2 := 0
	for r != nil {
		fmt.Println("value: ", r.Val, "index: ", index2)
		index2 ++
		r = r.Next
	}
}

