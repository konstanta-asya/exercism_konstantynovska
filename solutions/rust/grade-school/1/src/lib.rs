use std::collections::{HashMap, HashSet};

pub struct School {
    roster: HashMap<u32, HashSet<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
            students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.students.insert(student.to_string()) {
            return;
        }
        let students = self.roster.entry(grade).or_insert_with(HashSet::new);
        students.insert(student.to_string());
        
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().copied().collect();
        grades.sort_unstable();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        if let Some(students) = self.roster.get(&grade) {
            let mut students: Vec<String> = students.iter().cloned().collect();
            students.sort();
            students
        } else {
            Vec::new()
        }
    }

    pub fn all_students(&self) -> Vec<String> {
        let mut all: Vec<(u32, String)> = Vec::new();
        for (&grade, students) in &self.roster {
            for student in students {
                all.push((grade, student.clone()));
            }
        }
        all.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        all.into_iter().map(|(_, name)| name).collect()
    }
}
