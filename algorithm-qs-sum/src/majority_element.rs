#[doc = "
多数元素
给定一个大小为 n 的数组 nums ，返回其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

 

示例 1：

输入：nums = [3,2,3]
输出：3
示例 2：

输入：nums = [2,2,1,1,1,2,2]
输出：2
 

提示：
n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109
 

进阶：尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。

相关标签

Rust



作者：力扣 (LeetCode)
链接：https://leetcode.cn/leetbook/read/top-interview-questions/xm77tm/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
"]
pub fn majority_element() -> i32{
    // use std::collections::HashMap;
    // impl Solution {
    //     pub fn majority_element(nums: Vec<i32>) -> i32 {
    //         let mut num_map = HashMap::new();
    //         let mut res:i32 = 0;
    //         for i in 0..nums.len(){
    //             let count = num_map.entry(nums[i]).or_insert(0);
    //             *count += 1;
    //             if num_map.get(&nums[i]).unwrap_or(&0) > &(&nums.len()/&2){
    //                 res = nums[i];
    //             }
    //         }
    //         return res;
    //     }
    // }
    //  Bad Solution
    //  执行用时：8 ms, 在所有 Rust 提交中击败了5.71%的用户
    //  内存消耗：2.3 MB, 在所有 Rust 提交中击败了77.14%的用户

    
    //This is awesome:
    let nums:Vec<i32> = [2,2,1,1,1,2,2].to_vec();
    let (mut cnt, len, mut res) = (0, nums.len(), nums[0]);
    for i in 0..len {
        cnt += if res == nums[i] { 1 } else { -1 };
        if cnt < 0 {
            res = nums[i];
            cnt = 0;
        }
    }
    println!("result is {}", res);
    res
}
