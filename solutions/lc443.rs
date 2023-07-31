impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut group_start = 0;
        let mut group_char: Option<char> = None;
        let mut group_count = 0;
        
        let mut comp_size = 0;
        
        chars.push('?');

        for i in 0..chars.len() {
            let c = chars[i].clone();

            match group_char {
                Some(gc) => {
                    if c == gc {
                        group_count += 1;
                    } else {
                        chars[comp_size] = gc;
                        comp_size += 1;

                        if group_count > 1 {
                            let count_str = group_count.to_string();
                            for (j, d) in count_str.chars().enumerate() {
                                chars[comp_size + j] = d;
                            }
                            
                            comp_size += count_str.len();
                        }
        
                        group_start = i;
                        group_count = 1;
                        group_char = Some(c);
                    }
                }
                None => {
                    group_char = Some(c);
                    group_count = 1;
                }
            }
        }

        comp_size as i32
    }
}