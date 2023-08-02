impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;

        while i < (nums.len() - 1) {
            let n = nums[i];
            
            if n == 0 {
                while j < nums.len() && nums[j] == 0 {
                    j += 1;
                }

                if j == nums.len() {
                    return;
                }

                nums[i] = nums[j];
                nums[j] = 0;
            }
            j += 1;
            i += 1;
        }
    }
}