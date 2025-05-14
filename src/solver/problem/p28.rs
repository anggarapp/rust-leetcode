pub struct P28;

impl P28 {
    pub fn solve_v1(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        let haystack_chars = haystack.chars();
        let mut lower = 0;
        let mut upper = 0 + needle.len();

        for i in 0..(haystack_chars.count() - needle.len() + 1) {
            let slice = &haystack[lower..upper];
            if slice == needle {
                return i as i32;
            }
            lower += 1;
            upper += 1;
        }

        return -1;
    }
    pub fn solve_v2(haystack: String, needle: String) -> i32 {
        if haystack == needle {
            return 0;
        }
        
        let needle_len = needle.len();
        let hay_len = haystack.len();
        
        if hay_len < needle_len {
            return -1;
        }

        for i in 0..hay_len - needle_len+1 {
            let test = &haystack[i..needle_len + i];
            if test == needle{
                return i as i32;
            }
        }
        -1
    }
}
