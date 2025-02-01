# OOP in Rust: an example

This project defines two main structures: `Person` and `Student`, each representing a real-world entity with various personal details. The `Student` structure extends the `Person` structure by adding additional information related to a student's ID and major. Both structures support modification of their fields via setter methods, and they include getter methods to retrieve their values.

The program demonstrates how to create and manipulate instances of `Person` and `Student`, modify their attributes using setters, and display their information.

## Features

- **`Person` Struct**: Represents a person with basic details like name, age, birth date, email, and address.
- **`Student` Struct**: Extends `Person` and adds details specific to students, such as student ID and major.
- **Setters and Getters**: Methods to modify and access the values of the fields after creation.
- **`Introduce` Trait**: A trait that allows objects of both `Person` and `Student` to introduce themselves with a simple message.
- **Displaying Information**: Methods to display the information of both `Person` and `Student` objects.
  
## Project Structure

- `main.rs`: The main entry point for the program, where `Person` and `Student` instances are created, modified, and displayed.
- `person.rs`: Defines the `Person` struct and its associated methods, including setters and getters.
- `student.rs`: Defines the `Student` struct, which is a composition of `Person`, along with additional student-specific fields and methods.
- `traits.rs`: Defines the `Introduce` trait, which is implemented for both `Person` and `Student` to allow self-introduction.

## Dependencies

- [chrono](https://crates.io/crates/chrono): A Rust crate for working with date and time. This is used to represent birth dates in the `Person` struct.

## How to Run the Project

1. Clone the repository to your local machine.
   ```bash
   git clone https://github.com/yourusername/POO.git
   ```

2. Navigate to the project folder:
    ```bash
        cd person-student-management
    ```

3. Build and run the project using Cargo:
    ```bash
        cargo run
    ```

## Example Usage

Below is an example of how the program works, including creating, modifying, and displaying Person and Student objects:
    ```rust
        fn main() {
            // Create a Person instance for Alice
            let mut person_1 = Person::new("Alice Johnson", 30, birth_date_1, "alice@example.com", "123 Main St");

            // Modify Alice's details
            person_1.set_name("Alice Williams");
            person_1.set_email("alice.williams@example.com");

            // Create another Person instance for Bob
            let mut person_2 = Person::new("Bob Smith", 29, birth_date_2, "bob@example.com", "456 Elm St");

            // Modify Bob's address
            person_2.set_address("789 Oak St");

            // Create a Student instance based on Bob
            let mut student = Student::new(&person_2, "S54321", "Mathematics");

            // Modify the student's major
            student.set_major("Physics");

            // Introduce both persons and the student
            person_1.introduce();
            person_2.introduce();
            student.introduce();
        }

## Methods Overview

1. Person:
    - `new()`: Creates a new Person with the given details.
    - `display_info()`: Displays all the information of a Person.
    - `set_name()`: Updates the name of the person.
    - `set_email()`: Updates the email of the person.
    - `set_address()`: Updates the address of the person.
    - `get_name()`: Retrieves the name of the person.
    - `get_age()`: Retrieves the age of the person.
    - `get_birth_date()`: Retrieves the birth date of the person.
    - `get_email()`: Retrieves the email of the person.
    - `get_address()`: Retrieves the address of the person.
    - `introduce()`: Prints a message introducing the person.

2. Student
    - `new()`: Creates a new Student instance, including the associated Person information.
    - `display_student_info()`: Displays both Person and student-specific information (ID and major).
    - ``set_major()`: Updates the student's major.
    - `set_student_id()`: Updates the student's ID.
    - `get_student_id()`: Retrieves the student's ID.
    - `get_major()`: Retrieves the student's major.
    - `introduce()`: Prints a message introducing the student, including their major.

## Contributing

Contributions are welcome! Feel free to fork the repository, make changes, and submit pull requests.

## Issues

If you encounter any bugs or issues with the project, please report them by opening an issue in the GitHub repository.

## License

This project is licensed under the MIT License - see the LICENSE file for details.