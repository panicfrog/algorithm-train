//
//  TwoSum.swift
//  Algorithm
//
//  Created by Ye Yongping on 2019/8/27.
//  Copyright © 2019 Yeyongping. All rights reserved.
//


func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
    var m = [Int:Int]()
    for (i, v) in nums.enumerated() {
        let remind = target - v
        if let si = m[remind] {
            return [si, i]
        }
        m[v] = i
    }
    return []
}

func runTwoSum() {
    print(twoSum([3, 3, 4], 6))
}
