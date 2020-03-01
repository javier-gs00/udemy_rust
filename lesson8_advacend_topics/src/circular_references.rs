use std::rc::{Rc};
use std::cell::RefCell;

// student* --- *course (Many to Many)
// struct Student<'a> {
//     name: String,
//     courses: Vec<&'a Course<'a>>
// }

// impl<'a> Student<'a> {
//     fn new(name: &str) -> Student<'a> {
//         Student {
//             name: name.into(),
//             courses: Vec::new()
//         }
//     }
// }

// struct Course<'a> {
//     name: String,
//     students: Vec<&'a Student<'a>>
// }

// impl<'a> Course<'a> {
//     fn new(name: &str) -> Course<'a> {
//         Course {
//             name: name.into(),
//             students: Vec::new()
//         }
//     }

//     fn add_student(&'a mut self, student: &'a mut Student<'a>) {
//         student.courses.push(self);
//         self.students.push(student);
//     }
// }

// ---------- Implementation with Rc and RefCell
// struct Student {
//     name: String,
//     courses: Vec<Rc<RefCell<Course>>>
// }

// impl Student {
//     fn new(name: &str) -> Student {
//         Student {
//             name: name.into(),
//             courses: Vec::new()
//         }
//     }
// }

// struct Course {
//     name: String,
//     students: Vec<Rc<RefCell<Student>>>
// }

// impl Course {
//     fn new(name: &str) -> Course {
//         Course {
//             name: name.into(),
//             students: Vec::new()
//         }
//     }

//     fn add_student(
//         course: Rc<RefCell<Course>>,
//         student: Rc<RefCell<Student>>
//     ) {
//         student.borrow_mut().courses.push(course.clone());
//         course.borrow_mut().students.push(student);
//     }
// }

// pub fn enter() {
//     let john = Rc::new(
//         RefCell::new(
//             Student::new("John")       
//         )
//     );

//     let jane = Rc::new(
//         RefCell::new(
//             Student::new("Jane")       
//         )
//     );

//     let course = Course::new("Rust Course");
//     let magic_course = Rc::new(RefCell::new(course));
    
//     Course::add_student(magic_course.clone(), john); 
//     Course::add_student(magic_course, jane);
// }

// ---------------- A better implemenatation using Normalization
// basically like a db
// students
// course
// Vec<Enrollment{ course, student }>

struct Student {
    name: String
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
            
    }
}

struct Course {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

pub fn enter() {
    let john = Student {
        name: "John".into()
    };

    let course = Course {
        name: "Intro to Rust".into()
    };

    let mut p = Platform::new();
    p.enroll(&john, &course);

    for c in john.courses(p) {
        println!("John is taking {}", c);
    }
}