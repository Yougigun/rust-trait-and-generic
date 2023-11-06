// Importing necessary items from the standard library.
use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

// Defining a static atomic variable to keep track of the next customer id.
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

// Defining a generic Customer struct with a phantom type parameter T.
pub struct Customer<T> {
    id: u64,               // Customer's ID.
    name: String,          // Customer's name.
    _type: PhantomData<T>, // Phantom type to enable generic behavior without holding any value of T.
}
// Implementing methods for any generic Customer.
impl<T> Customer<T> {
    pub fn new(name: String) -> Self {
        // Constructor method to create a new Customer instance.
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed), // Incrementing and getting the next ID atomically.
            name,
            _type: PhantomData, // Initializing phantom data.
        }
    }
}

// Defining a struct for FreePlan with no fields.
pub struct FreePlan;

// Defining a struct for PersonalPlan with a single field for payment amount.
pub struct PersonalPlan(f32);

// Defining a trait for free plan features.
pub trait Free {
    fn feature1(&self); // Method for feature 1.
    fn feature2(&self); // Method for feature 2.
}

// Defining a trait for personal plan features, extending Free trait.
pub trait Personal: Free {
    fn advance_feature(&self); // Method for advanced feature accessible to personal plan.
}

// Implementing Free trait for any generic Customer.
impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name); // Implementing feature 1 method.
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name); // Implementing feature 2 method.
    }
}

// Implementing Personal trait specifically for Customer with PersonalPlan type.
impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        ); // Implementing advanced feature method.
    }
}

// Implementing From trait to convert a Customer<FreePlan> into a Customer<PersonalPlan>.
impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name) // Creating a new Customer<PersonalPlan> using the name of Customer<FreePlan>.
    }
}

// Function to handle subscription of a customer to a paid plan.
pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    // Creating a new PersonalPlan instance with the payment amount.
    let _plan = PersonalPlan(payment);
    // Storing plan to DB (Database operation is not implemented).
    // ...
    customer.into() // Converting and returning a Customer<PersonalPlan>.
}

// Module for unit tests.
#[cfg(test)]
mod tests {
    use super::*; // Importing all items from the outer scope.

    #[test] // Attribute to denote a test function.
    fn test_customer() {
        // Initially creating a free plan customer.
        let customer = Customer::<FreePlan>::new("Tyr".into());
        // Using free plan features.
        customer.feature1();
        customer.feature2();
        // Customer decides to subscribe to a paid plan.
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature1();
        // Paid customer unlocks new feature.
        customer.advance_feature();
    }
}
