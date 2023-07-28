/*
    There is a better, more mathematical solution to this problem.
*/

/// Returns true if str1 divides str2.
fn divides(str1: &str, str2: &str) -> bool {
    let times = str2.len() / str1.len();

    let mut str1_repeated = String::new();
    for i in 0..times {
        str1_repeated.push_str(str1);
    }
    
    return str1_repeated == str2;
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut solution = String::new();
        let mut curr = String::new();

        let min_len = std::cmp::min(str1.len(), str2.len());

        for i in 0..min_len {
            if str1.as_bytes()[i] != str2.as_bytes()[i] {
                break;
            }
            
            curr.push(str1.as_bytes()[i] as char);
            
            if divides(&curr, &str1) && divides(&curr, &str2) {
                solution = curr.clone();
            }
        }

        solution
    }
}