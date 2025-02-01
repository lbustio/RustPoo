/// Define the `Introduce` trait that specifies the behavior of introducing a person or entity.
pub trait Introduce {
    /// Method to introduce the entity implementing this trait.
    ///
    /// This method should print a message introducing the entity, typically with details such as name, age, etc.
    ///
    /// # Example
    /// ```
    /// let person = Person::new("Alice", 30, "alice@example.com", "123 Main St");
    /// person.introduce();  // This would output: "Hi, my name is Alice and I am 30 years old."
    /// ```
    fn introduce(&self);
}
