fn main() {
    //println!("Hello, world!");
    //Chapter 5 - Intermediate to Advanced Videos in Rust

    //Chapter 5 - Section 15  What Are Smart Pointers in Rust
    cpt5_sct15_pointers();

    //Chapter 5 - Selection 14 - What are Closures in Rust
    cpt5_sct14_closures();

    //Chapter 5 - Selection 10 - What is Iter in Rust and Iterators
    cpt5_sct10_iterator();

}


fn cpt5_sct15_pointers() {
    //Chapter 5 - Section 15  What Are Smart Pointers in Rust
    println!("Chapter 5 - Section 15  What Are Smart Pointers in Rust");
    /*
    Smart Pointers in Rust

    A pointer is general concept for a variable that contains an address in memory.
    This address refers to , or "points at," some other data.
    The most common kind of pointer in Rust is a referrence,
    Referrences are indicated by the & symbol and borrow the value they point to.
    They don't have any special capabilities other than referring to data, and have no overhead.

    Smart pointers, on the other hand, are data structures that act like a pointer
    but also have additional metadata and capabilities.

    The concept of smart pointers isn't unique ro Rust: smart pointers originated in C++
    and exist in other languages as well.
    Rust has a variety of smart pointers defined in the standard library
    that provide functionality beyond that provided by referrences.

    Box

    Box is a smart pointer that allows you to store memory on the heap explicitly rather than the stack
    and the stack contains the pointer to the heap data

    Two Important Traits implemented by Smart Pointers

    1. Deref - immutable dereferencing operations

        std::ops::Deref

    2. Drop - when a value goes out of scope you can run some code also known as a destructor

        std::ops::Drop
    
    */

    //---What is box and dereferencing with the unary * operator---
    let greeting = "hello";
    let greeting_heap = Box::new(greeting);

    println!("{}", "hello"==greeting);
    println!("{}", "hello"==*greeting_heap);

    println!("Chapter 5 - Section 15...end");
}

fn cpt5_sct14_closures() {
    //Chapter 5 - Selection 14 - What are Closures in Rust
    
    println!("Chapter 5 - Selection 14 - What are Closures in Rust");

    /*
    Closures in Rust

    Closures are functions within functions that are nameless (anonyumous functions)

    We can assign closures to variables and then pass a function as a parameter to other functions.
    They are also known as in line functions

    Closures: Anonymous Functions that capture Their Environment
    
    Rust's closures are anonymous functions you can save in a variable
    or pass as arguments to other functions.
    You can create the closure in one place and then call the closure elsewhere
    to evaluate it in different context.
    Unlike functions, closures can capture values from the scope in which they're definded.
    We'll demonstrate how these closure features allow for code reuse and behavior customization.

    let closures_functions = |parameter| {
        //pass some logic
    }

    */

    let is_even = |n| {
        n%2==0
    };

    let num = 12;

    println!("{} is even? {}", num, is_even(num));


    println!("Chapter 5 - Selection 14...end");
}


fn cpt5_sct10_iterator() {
    //Chapter 5 - Selection 10 - What is Iter in Rust and Iterators
    println!("Chapter 5 - Selection 10 - What is Iter in Rust and Iterators");

    /*
        Chapter 5 - Selection 10 - What is Iter in Rust and Iterators

        Iterates traverse and iterate over a different collections of values
        from type such as arrays, vectors, maps, etc.

        The Iterator trait is invoked from iterators defined in the Rust standard library.
        The iter() method returns an iterator object of the collection of items.
        The next() method traverses through items and returns none once it reaches the end of items

    */

    let x = [1, 2, 3];
    let iter = x.iter();
    for items in iter {
        print!("{}\t",items);
    }


    // into_iter method moves values in the collection into an inter object via ownership
    let values = vec!["a", "b", "c"];
    println!("{:?}", values);
    for value in values.into_iter() {
        match value {
            "c" => println!("c in a good time"),
            _ => println!("iteration: {}", value)
            
        }

    }
    //println!("{:?}", values); //error[E0382]: borrow of moved value: `values` 

    println!("Chapter 5 - Selection 10 - Exercise - Iterate through a Vector in Rust");
    /*
    Exercise - Iterate through a Vector in Rust

    Write a vactor called pets which contains the 3 string items cat, dog and goldfish.
    Use the for in inter_mut() signature to match dog so that the program prints "cute doggy!"
    The default underscore _ should print "hello!" and each pet name for each iteration

    */
    let mut pets = vec!["Cat", "Dog", "Goldfish"];
    println!("{:?}", pets);
    for fav in  pets.iter_mut() {
        match fav {
            &mut "Dog" => println!("cute Doggy!"),
            _ => println!("hello! {}", fav)
            
        }
    }
    println!("{:?}", pets);

    println!("Chapter 5 - Selection 10...end"); 
}

