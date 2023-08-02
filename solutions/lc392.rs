impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_curr = 0;
        let mut t_curr = 0;

        while s_curr < s.len() && t_curr < t.len() {
            let s_n = s.as_bytes()[s_curr];
            let t_n = t.as_bytes()[t_curr];

            if s_n == t_n {
                s_curr += 1;
            }

            t_curr += 1;
        }

        s_curr == s.len()
    }
}