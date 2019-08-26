//
//  reverse linked list.swift
//  Algorithm
//
//  Created by Ye Yongping on 2019/8/26.
//  Copyright © 2019 Yeyongping. All rights reserved.
//

import Foundation

public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init(_ val: Int) {
        self.val = val
        self.next = nil
    }
}

func reverseList(_ head: ListNode?) -> ListNode? {
    var (pre, cur): (ListNode?, ListNode?) = (nil, head)
    while cur != nil {
        let next = cur!.next
        cur!.next = pre
        pre = cur
        cur = next
    }
    return pre
}

func runReverseLinkedList () {
    let head = ListNode(0)
    var list = head
    for i in 1..<10 {
        let l = ListNode(i)
        list.next = l
        list = l
    }
    
    var h: ListNode? = head
    var j = 0
    repeat {
        print("value: ",h!.val, "  index: ", j)
        j += 1
        h = h?.next
    } while h?.next != nil
    print("\n")
    
    h = reverseList(head)
    
    var index = 0
    repeat {
        print("value: ",h!.val, "  index: ", index)
        index += 1
        h = h?.next
    } while h?.next != nil
}

