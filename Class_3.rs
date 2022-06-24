use std::collections::HashMap;
fn main() {
    let mut school = School::new();
   

    school.add(5,"tranvana");
    school.add(8, "e");
    school.add(8, "c");
    school.add(9, "ngo d");

    println!("school: {:?}", school);
    let grad_op = school.grades();
    println!("grade: {:?}", grad_op);
    let studen_op = school.grade(8);
    println!("studen: {:?}", studen_op);

}

#[derive(Debug)]
pub struct School {
    students: HashMap<String, u32>,
    
}
impl School {
    pub fn new() -> School {
        School{
            students: HashMap::new(),
        }
     
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.to_string(), grade);

        
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grad_op = Vec::new();
        for (_, value) in self.students.iter(){
            grad_op.push(*value);
        }
        grad_op.sort();
        grad_op.dedup();
        
        grad_op
    }


    pub fn grade(&self, grade: u32) -> Vec<String> {
     
        let mut grad_re = Vec::new();
        for (key, value) in self.students.iter(){
            if *value == grade {
                grad_re.push(key.to_string());
            }
        }
        grad_re.sort();
        grad_re
        
    }
}
