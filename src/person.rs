use chrono::{DateTime, Utc}; // Import DateTime and Utc for the birth date
use crate::traits::Introduce;

/// Represents a person with basic personal details.
#[derive(Clone)] // Derive Clone to allow cloning of Person instances
pub struct Person {
    _name: String,
    _age: u32,
    _birth_date: DateTime<Utc>,
    _email: String,
    _address: String,
}

impl Person {
    /// Creates a new `Person` instance with the given details.
    /// 
    /// # Arguments
    /// * `name` - A string slice that holds the name of the person.
    /// * `age` - The person's age.
    /// * `birth_date` - A DateTime<Utc> object representing the person's birth date.
    /// * `email` - A string slice that holds the email address of the person.
    /// * `address` - A string slice that holds the address of the person.
    pub fn new(name: &str, age: u32, birth_date: DateTime<Utc>, email: &str, address: &str) -> Self {
        Self {
            _name: name.to_string(),
            _age: age,
            _birth_date: birth_date,
            _email: email.to_string(),
            _address: address.to_string(),
        }
    }

    /// Displays the person's information.
    pub fn display_info(&self) {
        println!("Name: {}", self._name);
        println!("Age: {}", self._age);
        println!("Birth Date: {}", self._birth_date);
        println!("Email: {}", self._email);
        println!("Address: {}", self._address);
    }

    // ----- Getter methods -----
    /// Returns the name of the person.
    pub fn get_name(&self) -> &str {
        &self._name
    }

    /// Returns the age of the person.
    pub fn get_age(&self) -> u32 {
        self._age
    }

    /// Returns the birth date of the person.
    pub fn get_birth_date(&self) -> DateTime<Utc> {
        self._birth_date
    }

    /// Returns the email address of the person.
    pub fn get_email(&self) -> &str {
        &self._email
    }

    /// Returns the address of the person.
    pub fn get_address(&self) -> &str {
        &self._address
    }

    // ----- Setter methods -----
    /// Setter method to update the name of the person.
    pub fn set_name(&mut self, new_name: &str) {
        self._name = new_name.to_string();
    }

    /// Setter method to update the email address of the person.
    pub fn set_email(&mut self, new_email: &str) {
        self._email = new_email.to_string();
    }

    /// Setter method to update the address of the person.
    pub fn set_address(&mut self, new_address: &str) {
        self._address = new_address.to_string();
    }

    // Optional: You can add setters for other fields like age, birth date, etc., if needed.
}

/// Implementing the `Introduce` trait for the `Person` struct.
impl Introduce for Person {
    fn introduce(&self) {
        println!("Hi, my name is {} and I am {} years old.", self._name, self._age);
    }
}
