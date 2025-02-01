mod person;  // Import the person module
mod student;  // Import the student module
mod traits;  // Import the traits module

use crate::person::Person;
use crate::student::Student;
use crate::traits::Introduce;  // Import the Introduce trait

use chrono::{NaiveDate, Utc, TimeZone};

fn main() {
    // Define the specific birth date (May 15, 1990) for the first person
    let naive_date = NaiveDate::from_ymd(1990, 5, 15);
    let birth_date = Utc.from_utc_date(&naive_date).and_hms(0, 0, 0);  // Convert to DateTime<Utc>

    // Create the first person
    let mut person_1 = Person::new("Alice Johnson", 30, birth_date, "alice@example.com", "123 Main St");

    // Display the information for the first person
    println!("Person 1 Info:");
    person_1.display_info();
    println!(); // Add an empty line for separation

    // Modify the name and email of person_1 using setter methods
    person_1.set_name("Alice Williams");
    person_1.set_email("alice.williams@example.com");

    // Display the updated information for the first person
    println!("Updated Person 1 Info:");
    person_1.display_info();
    println!();

    // Create the second person (Bob Smith)
    let naive_date_2 = NaiveDate::from_ymd(1995, 10, 15);
    let birth_date_2 = Utc.from_utc_date(&naive_date_2).and_hms(0, 0, 0);  // Convert to DateTime<Utc>

    let mut person_2 = Person::new("Bob Smith", 29, birth_date_2, "bob@example.com", "456 Elm St");

    // Display the information for the second person
    println!("Person 2 Info:");
    person_2.display_info();
    println!(); // Add an empty line for separation

    // Modify the address of person_2 using the setter method
    person_2.set_address("789 Oak St");

    // Display the updated information for the second person
    println!("Updated Person 2 Info:");
    person_2.display_info();
    println!();

    // Create a student based on the second person
    let mut student = Student::new(&person_2, "S54321", "Mathematics");

    // Display the student's information
    println!("Student Info:");
    student.display_student_info();

    // Modify the student's major using setter method
    student.set_major("Physics");

    // Display the updated student information
    println!("Updated Student Info:");
    student.display_student_info();

    // Introduce both persons and the student
    println!("\nIntroducing Person 1:");
    person_1.introduce();  // Call the `introduce` method for Person 1

    println!("\nIntroducing Person 2:");
    person_2.introduce();  // Call the `introduce` method for Person 2

    println!("\nIntroducing Student:");
    student.introduce();  // Call the `introduce` method for Student
}
