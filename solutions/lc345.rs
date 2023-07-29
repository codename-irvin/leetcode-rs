impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        // Find all of the vowels in the string.
        let mut found_vowels: Vec<char> = s.chars().filter(|c| vowels.contains(&c)).collect();
        
        // Replace the vowels in reverse order using pop() to grab the last
        // vowel in the list each time.
        s.chars().map(|c| {
            if vowels.contains(&c) {
                return found_vowels.pop().unwrap();
            }

            c
        }).collect()
    }
}