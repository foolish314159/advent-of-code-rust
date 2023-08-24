use std::{collections::VecDeque, fs::read_to_string};

pub fn chars_before_marker<const N: usize>(filename: &str) -> usize {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    CharMarker::<N>::find_index_after_marker(&content)
}

struct CharMarker<const N: usize> {
    vec: VecDeque<char>,
    total_chars: usize,
}

impl<const N: usize> CharMarker<N> {
    fn create() -> CharMarker<N> {
        CharMarker::<N> {
            vec: VecDeque::new(),
            total_chars: 0,
        }
    }

    fn push(&mut self, c: char) {
        if self.vec.len() == N {
            self.vec.pop_front();
        }
        self.vec.push_back(c);
        self.total_chars += 1;
    }

    fn is_present(&self) -> bool {
        let unique = self.vec.iter().fold(vec![], |mut unique, c| {
            if !unique.contains(c) {
                unique.push(*c);
            }
            unique
        });

        self.vec.len() == N && unique.len() == N
    }

    fn find_index_after_marker(str: &String) -> usize {
        let mut marker = CharMarker::<N>::create();
        for c in str.chars() {
            marker.push(c);
            if marker.is_present() {
                return marker.total_chars;
            }
        }

        panic!("marker not found");
    }
}
