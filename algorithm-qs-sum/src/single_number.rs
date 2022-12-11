#[doc = 
"只出现一次的数字
给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。

示例 1 ：
输入：nums = [2,2,1]
输出：1

示例 2 ：
输入：nums = [4,1,2,1,2]
输出：4

示例 3 ：
输入：nums = [1]
输出：1

提示：
1 <= nums.length <= 3 * 104
-3 * 104 <= nums[i] <= 3 * 104
除了某个元素只出现一次以外，其余每个元素均出现两次

作者：力扣 (LeetCode)
链接：https://leetcode.cn/leetbook/read/top-interview-questions/xm0u83/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。"]
pub fn single_number() {
    let nums: Vec<i32> = [2, 2, 1, 3, 3, 1, 6, 7, 5, 4, 4, 5, 6].to_vec();
    let mut res = 0;
    for i in 0..nums.len() {
        res ^= nums[i];
    }

    println!("res is {}", res);
}
