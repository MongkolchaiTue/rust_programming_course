// packtpub.com Rust Programming 2023 - A Comprehensive Course for Beginners By : Clarian North
// coding example by Mongkolchai Tue

fn main() {
    //println!("Hello, world!");

    // What's in 1.80.0 stable
    sample_update_v1800();

    // Chapter 5 - Intermediate to Advanced Videos in Rust

    // Chapter 5 - Section 19 - Multi-Threads in Rust
    cpt5_sct18_multi_threads();

    //Chapter 5 - Section 15  What Are Smart Pointers in Rust
    cpt5_sct15_pointers();

    //Chapter 5 - Selection 14 - What are Closures in Rust
    cpt5_sct14_closures();

    // Chapter 5 - Selection 10 - What is Iter in Rust and Iterators
    cpt5_sct10_iterator();

}

fn sample_update_v1800() {
    println!("What's in 1.80.0 stable LazyCell and LazyLock");

    /*
    What's in 1.80.0 stable
    LazyCell and LazyLock
    These "lazy" types delay the initialization of their data until first access.
    They are similar to the OnceCell and OnceLock types stabilized in 1.70,
    but with the initialization function included in the cell.
    This completes the stabilization of functionality adopted into
    the standard library from the popular lazy_static and once_cell crates.

    LazyLock
    is the thread-safe option, making it suitable for places like static values.

    For example,
    both the spawn thread and the main scope will see the exact same duration below,
    since LAZY_TIME will be initialized once, by whichever ends up accessing the static first.
    Neither use has to know how to initialize it,
    unlike they would with OnceLock::get_or_init().
    */

    /*
    LazyCell
    does the same thing without thread synchronization, so it doesn't implement Sync,
    which is needed for static, but it can still be used in thread_local! statics
    (with distinct initialization per thread).
    
    Either type can also be used in other data structures as well,
    depending on thread-safety needs, so lazy initialization is available everywhere!
    */

    use std::cell::LazyCell;
    use std::sync::LazyLock;
    use std::time::Instant;

    static LAZY_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);
    let lazy_time2: LazyCell<Instant> = LazyCell::new(Instant::now);

    let start = Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Thread lazyLock time is {:?}", LAZY_TIME.duration_since(start));
        });
        println!("Main lazyCell time is {:?}", lazy_time2.duration_since(start));
    });

    println!("Checked cfg names and values");
    /*
    Checked cfg names and values
    In 1.79, rustc stabilized a --check-cfg flag, and now Cargo 1.80
    is enabling those checks for all cfg names and values that it knows
    (in addition to the well known names and values from rustc).
    This includes feature names from Cargo.toml as well as new cargo::rustc-check-cfg output from build scripts.

    Unexpected cfgs are reported by the warn-by-default unexpected_cfgs lint,
    which is meant to catch typos or other misconfiguration. For example,
    in a project with an optional rayon dependency,
    this code is configured for the wrong feature value:

    */
    /*
    println!("Hello, world!");

    #[cfg(feature = "crayon")]
    rayon::join(
        || println!("Hello, Thing One!"),
        || println!("Hello, Thing Two!"),
    );
    */

     println!("What's in 1.80.0 stable...end")
}


fn cpt5_sct18_multi_threads() {
    // Chapter 5 - Section 19 - Multi-Threads in Rust
    println!("Chapter 5 - Section 19 - Multi-Threads in Rust");
    /*
    Concurrency in Rust
    
    When you want to have different parts of a program execute independantly
    you are looking at the concept of concurrency.
    Whereas with parralel programming you have different parts of the program
    or computers executing at the same time
    Both of these model stem from the core feature of multiple processing.

    Threads

    We use threads to run codes at the same time.
    Operating systems manage multiple processes at once and programs execute code in a process

    When you have independent parts of your code that run together you are using
    the feature of multi threaded programming.

    How to Create A Thread in Rust

    We can create threads in Rust with thread::spawn

    The parameter for the spawn is a closure which defines the code.

    Every thread is equipped with some basic low-level blocking support,
    via the thread::park function
    and  thread::Thread::unpark method.
    park blocks the current thread,
    which can then be resumed from another thread
    by calling the unpark method on the blocked thread's handle.

    */
    
    //import modules
    use std::thread;
    use std::time::Duration;

    let park_thread = thread::Builder::new().spawn(
        || {
            println!("1.Parking Thread!");
            thread::park();
            println!("4.Thread unparked!");
        }
    ).unwrap(); // give me the result and if there is an error paniccc and stop the program
    thread::sleep(Duration::from_millis(10));

    println!("2.Unpark the Thread");
    park_thread.thread().unpark();
    println!("3.Unwrap the Thread");
    park_thread.join().unwrap(); // the join handle waits for the associated thread to finish


    println!("Section 20 - Exercise—Build Your Own Multi-Threads in Rust");
    // Section 20 - Exercise—Build Your Own Multi-Threads in Rust
    /*
    Exercise - Build your own threads in Rust
    1. Create two definite loops in the main function.

    2. One loop should be set to a spawn thread x which runs from 1 to 20 with a sleep duration
    of half the time of the main thread loop. The spawn thread should also print the index values of each
    iteratin as it loops util completion as should the main thread loop.

    3. The other loop should run in the main thread with a sleep duration of twice the time
    of the spawn threaded loop.
    The loop should run only from 1 to 5 and should also print each index valued tagged as the main thread.

    4. Ensure the spawn thread has a full completion of its run even if the main thread finishes.

    */

    let x_thread = thread::Builder::new().spawn(
        || {
            for x in 1..20 {
                println!("x Thread index is {}", x);
            }
            println!("x Thread loop...end");
        }
    ).unwrap(); // give me the result and if there is an error paniccc and stop the program

    let y_thread = thread::Builder::new().spawn(
        || {
            for y in 1..5 {
               println!("y Thread index is {}", y);
               thread::sleep(Duration::from_millis(0));
            }
            println!("y Thread loop...end");
        }
    ).unwrap(); // give me the result and if there is an error paniccc and stop the program    
    
    println!("x.Unpark the Thread");
    x_thread.thread().unpark();
    println!("x.Unwrap the Thread");
    x_thread.join().unwrap(); // the join handle waits for the associated thread to finish

    println!("y.Unpark the Thread");
    y_thread.thread().unpark();
    println!("y.Unwrap the Thread");
    y_thread.join().unwrap(); // the join handle waits for the associated thread to finish



    println!("Section 21 - Solution - Exercise Build Your Own Multi-Threads in Rust");
    // Exercise Solution
    let x = thread::spawn( || {
        for i in 1..21 {
            println!("spawn thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(2));
    }

    x.join().unwrap(); // the join handle waits for the associated thread to finish

    println!("Chapter 5 - Section 19...end");
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

    println!("Section 16 - Code Along and Customize Your Own Smart Pointer in Rust");

    /*
    Section 16 - Code Along and Customize Your Own Smart Pointer in Rust

    Smart pointers are a powerful tool for managing memory in Rust,
    allowing you to create more complex data structures and avoid common memory management pitfalls.
    However, out of the box, Rust only provides a handful of smart pointer types, such as Box and Rc.
    
    In this video,
    we will take a deep dive into how smart pointers work in Rust,
    and we will code along to create our own custom smart pointer type.

    */
    use std::ops::Deref;

    struct CustomizeSmartPointer<T>(T);

    impl <T> CustomizeSmartPointer<T> {
        // generic structure with static method
        fn heap_allocation(value: T) -> CustomizeSmartPointer<T> {
            CustomizeSmartPointer(value)
        }
    }

    // implement deref trait
    impl <T> Deref for CustomizeSmartPointer<T> {
        type Target = T;    // in traits, type is used to declare an associated type
        fn deref(&self) -> &T {
            &self.0 //syntax for taking the first argument which is 0
        }
    }

    let color = "green";
    //call static method
    let ref_color = CustomizeSmartPointer::heap_allocation(color);
    println!("green value is the same as color which is {}", "green"==color);
    println!("green value is the same as color which is {}", "green"==*ref_color);

    // Section 18 - Exercise—Customize Your Own Custom Smart Pointer
    println!("Section 18 - Exercise—Customize Your Own Custom Smart Pointer");

    /*
    Exercise - Customize your own Smart Pointer in Rust
    In Rust, you can achieve automatic memory deallocation using Drop trait.

    1. Implement the Drop trait for our Custom Pointer.
    It should include a function called specifically drop and print in the function
    a completion run of the drop object from memory.

    */

    use std::ops::Drop;
    // implement drop trait
    impl <T> Drop for CustomizeSmartPointer<T> {
        fn drop(&mut self) {
            println!("dropping CustomSmartPointer object from memory");
        }
    }


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

