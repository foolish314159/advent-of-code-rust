use std::fs::read_to_string;

pub fn sum_full_overlaps(filename: &str) -> usize {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content
        .lines()
        .map(|line| assignment_pair(line))
        .filter(|pair| pair.0.full_overlap(&pair.1))
        .count()
}

pub fn sum_partial_overlaps(filename: &str) -> usize {
    let content = read_to_string(filename).unwrap_or(String::from(""));
    content
        .lines()
        .map(|line| assignment_pair(line))
        .filter(|pair| pair.0.partial_overlap(&pair.1))
        .count()
}

fn assignment_pair(line: &str) -> (SectionAssignment, SectionAssignment) {
    let mut parts = line.split(',');
    (
        assignment(parts.next().unwrap()),
        assignment(parts.next().unwrap()),
    )
}

fn assignment(str: &str) -> SectionAssignment {
    let mut assignment_parts = str.split('-');
    SectionAssignment {
        start: assignment_parts.next().unwrap().parse::<u32>().unwrap(),
        end: assignment_parts.next().unwrap().parse::<u32>().unwrap(),
    }
}

struct SectionAssignment {
    start: u32,
    end: u32,
}

impl SectionAssignment {
    fn full_overlap(&self, other: &SectionAssignment) -> bool {
        (self.start >= other.start && self.end <= other.end)
            || (other.start >= self.start && other.end <= self.end)
    }

    fn contains(&self, num: u32) -> bool {
        num >= self.start && num <= self.end
    }

    fn overlaps(&self, other: &SectionAssignment) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }

    fn partial_overlap(&self, other: &SectionAssignment) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}
