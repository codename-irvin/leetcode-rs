impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut available_plots = 0;
        let mut consec_zeros = 1; // Handles the case when the array starts with zeros.

        // Find spots where there are 3 consecutive empty plots so that we know the middle plot can be filled.
        for val in flowerbed {
            if val == 0 {
                consec_zeros += 1;
            } else {
                consec_zeros = 0;
            }

            if consec_zeros == 3 {
                available_plots += 1;
                consec_zeros = 1;
            }
        }

        // Handles the case when the array ends with zeros.
        if consec_zeros > 1 {
            available_plots += 1;
        }

        available_plots >= n
    }
}