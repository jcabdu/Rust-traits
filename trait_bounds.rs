// TRAITS: functionality a type has and can share with other types - shared behavior. 
//  - trait bounds to specify a generic can be any type that has that behavior. 
//  - type's behavior consists on the methods we can call on that type => different types share same behavior if we can call the same methods on them. 
//  - trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {

pub trait Summary {         //trait declaration -  
    fn summarize (&self) -> String;         //summarize method signature, describing the behavior of the types that implement this trait -  
    // ; instead of {} : each type implementing this trait must provide its own custom behavior for the method's body -        
}

// (1) IMPLEMENTATION of the Summary trait on the types NewsArticle and Tweet (both structs):

pub struct NewsArticle {
    pub headline: String, 
    pub location: String, 
    pub author: String, 
    pub content: String, 
}

impl Summary for NewsArticle {
    fn summarize (&self) -> String {
        format! ("{}, by {} ({})", self.headline, self.author, self.location)       //method body with the specific behavior for the type NewsArticle - 
    }
}

pub struct Tweet {
    pub username: String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool, 
}

impl Summary for Tweet {
    fn summarize (&self) -> String{
        format! ("{}: {}", self.username, self.content)
    }
}

    // Calling this trait's method on an instance "tweet" of Tweet: 
    
    let tweet= Tweet {username: String::from ("jcabdu"), content: String::from ("Traits in Rust are fun!"), reply: false, retweet: false}; 
    println! ("1 new tweet: {}", tweet.summarize());

// (2) DEFAULT IMPLEMENTATIONS of methods - instead of requiring one for every method on every type: 

pub trait Summary2 {
    fn summarize2 (&self) -> String {
        String::from ("(Read more...)")         
        //previous specific impl. of this trait (if any) still works 
        //  since the syntax for overriding a default impl. is the same one as impl a trait method that doesn't have a def impl -     
    }
}

impl Summary2 for NewsArticle {}        //empty impl block to use the def. impl. of the method summarize2 on an instance of NewsArticle - 

    let article= NewsArticle { 
        headline: String::from ("Red Coalition Wins Danish General Elections"), 
        location: String::from ("Copenhagen, Denmark"), 
        author: String::from ("Julio Cabdu"),
        content: String::from ("Social Democrats back in government, ... "),
    }; 
    
    println! ("New article available! {}", article.summarize2());       //this prints "New article available! (Read more...)" - 

// (3) Default impl. can call other methods in the same trait, even if they don't have a def impl.: 

pub trait Summary3 {
    fn summarize_author (&self) -> String; 

    fn summarize3 (&self) -> String {
        format! ("(Read more from {}...)", self.summarize_author())
    }
}

    // To use Summary3 we only need to define summarize_author when implementing the trait on a type: 
impl Summary3 for Tweet {
    fn summarize_author (&self) -> String {
        format! ("@{}", self.username)
    }
}

    // Now we can use .summarize3() with its default impl. on an instance of Tweet: 
    let tweet= Tweet {
        username: String::from ("jcabdu"), 
        content: String::from ("Implementing the Summary3 trait for the type Tweet, has given us the behavior of the summarize3 method by default"), 
        reply: false, 
        retweet: false, 
    }; 

    println! ("1 new tweet: {}", tweet.summarize3());       //prints: 1 new tweet: (Read more from jcabdu...) - 

    // It's not possible to call a default impl. from an overriding impl of that same method.   

// (4) TRAITS AS PARAMETERS with "impl trait" syntax:
//  - to define functions that accept many different types, as long as they implement that trait's behaviour. 
//  - this syntax is convenient and makes for more concise code in simple cases. 
//  - appropriate if the parameters can be different types, as long as they implement the proper trait (*). 

pub fn notify (item: impl Summary) {        
        //the item parameter is of any type that implements the Summary trait. Code that calls the fn with any other type, won't compile -  
    println! ("Breaking news! {}", item.summarize());       //the notify fn calls the summarize method on its item parameter -  
}

// (5) TRAIT BOUND SYNTAX:
//  - implementing a trait with the declaration of the generic type parameter T: "<T: trait>" syntax. 
//  - bound syntax can express more complexity than the "impl trait" syntax (4).

pub fn notify2 <T: Summary> (item: T) {  
    println! ("Breaking news: {}", item.summarize());       //notify2 is equivalent to notify but more verbose - 
}

    // Two parameters, forcing them to be the same type (*): 
pub fn notify3 <T: Summary> (item1: T, item2: T) {}         
        //monomorphized from the generic T, the concrete type of the values passed as item1 and item2 must be the same -

// (6) MULTIPLE TRAIT BOUNDS => + syntax: 

use std::fmt::Display; 

pub fn notify4 (item: impl Summary + Display) {}
    
    // Trait bounds on generic types: 
pub fn notify5 <T: Summary + Display> (item:T) {}

// (7) Clearer trait bounds => WHERE CLAUSES: 

use std::fmt::Debug; 

fn some_function <T, U> (t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{32}    //or any other body - 

// (8) RETURNING TYPES that implement traits: 

fn returns_summarizable() -> impl Summary {         //with this syntax, only 1 type is allowed - more than one, it won't compile => use trait objects - 
    Tweet{                                           
        username: String::from("jcabdu"), 
        content: String::from("The returning type is any that implements the trait Summary"), 
        reply: true, 
        retweet: false, 
    }
}

// (9) Trait bounds to conditionally implement methods:

struct Pair <T> {
    x:T, 
    y:T, 
}

impl <T> Pair <T> {         //the type Pair<T> always implements the "new" function - 
    fn new (x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// use std::fmt::Display;       (brought into scope previously - PartialOrd is in the prelude)

impl <T: Display + PartialOrd> Pair <T> {       
        //Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait (to enable comparison) 
        //  and the Display trait (to enable printing) -  
    fn cmp_display (&self) {
        if self.x >= self.y {
            println! ("The largest member is x= {}", self.x); 
        } else {
            println! ("The largest member is y= {}", self.y); 
        }
    }
}

// Blanket implementations: implementations of a trait on any type that satisfies the trait bounds. 
// E.g. the ToString trait on any type that implements the Display trait: 
//      impl <T: Display> ToString for T {}
    
    let s= 9.to_string(); 

    // Traits and trait bounds let us write code that uses generic type parameters to reduce duplication 
    //  but also specify to the compiler that we want the generic type to have particular behavior. 
}
