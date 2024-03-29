
use std::collections::HashMap;

impl Anagram {
    pub fn is_anagram(s: String, t: String) -> bool {
        /*
        - String is a mutable, growable, heap-allocated string type.
        It has ownership of the memory it represents and it can be dynamically resized.
        - str is an immutable, fixed size view into a sequence of UTF-8 bytes.
        It does not own the underlying data.
         */
        let c1 = Self::count_chars(s);
        let c2 = Self::count_chars(t);
        return c1 == c2;

    }

    pub fn count_chars(input: String) -> HashMap<char, usize> {
        let mut counter = HashMap::new();
        for character in input.chars() {
            let value = counter.entry(character).or_insert(0);
            *value += 1;
        }
        return counter;
    }
}