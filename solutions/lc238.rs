impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // Calculate the product of all nums after i
        // for each i from 0 to n.
        let mut reversed_mults: Vec<i32> = Vec::new();
        let mut accum = 1;
        for (i, x) in nums.iter().rev().enumerate() {
            reversed_mults.push(accum);
            accum *= x;
        }
        reversed_mults.reverse();

        // Compute the solution by keeping the running product
        // of all nums before i and multiplying by the
        // product of nums after i.
        accum = 1;
        let mut solution: Vec<i32> = Vec::new();
        for (i, x) in nums.iter().enumerate() {
            solution.push(accum * reversed_mults[i]);
            accum *= x;
        }

        solution
    }
}