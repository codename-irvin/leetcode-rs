impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let max_length = std::cmp::max(word1.len(), word2.len());
        let mut solution = String::new();

        for i in 0..max_length {
            let c1 = word1.chars().nth(i);
            let c2 = word2.chars().nth(i);

            if let Some(c1) = c1 {
                solution.push(c1);
            }

            if let Some(c2) = c2 {
                solution.push(c2);
            }
        }
        
        solution
    }
}