use crate::person::Person; // Import Person struct from person module
use crate::traits::Introduce; // Import Introduce trait from traits module

/// Represents a student, which is a person with additional details.
pub struct Student {
    _person: Person,  // Composition of Person
    _student_id: String,
    _major: String,
}

impl Student {
    /// Creates a new `Student` instance with the given details.
    /// 
    /// # Arguments
    /// * `person` - A reference to a `Person` object, representing the student.
    /// * `student_id` - The student's ID.
    /// * `major` - The student's major.
    pub fn new(person: &Person, student_id: &str, major: &str) -> Self {
        Self {
            _person: person.clone(),  // Clone the person to avoid moving ownership
            _student_id: student_id.to_string(),
            _major: major.to_string(),
        }
    }

    /// Displays the student's information.
    pub fn display_student_info(&self) {
        self._person.display_info();  // Displays the person's information
        println!("Student ID: {}", self._student_id);
        println!("Major: {}", self._major);
    }

    // ----- Getter methods -----
    /// Returns the student's ID.
    pub fn get_student_id(&self) -> &str {
        &self._student_id
    }

    /// Returns the student's major.
    pub fn get_major(&self) -> &str {
        &self._major
    }

    // ----- Setter methods -----
    /// Setter method to update the student's major.
    pub fn set_major(&mut self, new_major: &str) {
        self._major = new_major.to_string();
    }

    /// Setter method to update the student's ID.
    pub fn set_student_id(&mut self, new_student_id: &str) {
        self._student_id = new_student_id.to_string();
    }
}

/// Implementing the `Introduce` trait for the `Student` struct.
impl Introduce for Student {
    fn introduce(&self) {
        self._person.introduce();  // Call the `introduce` method of Person
        println!("I am a student majoring in {}.", self._major);
    }
}
