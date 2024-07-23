// packtpub.com Rust Programming 2023 - A Comprehensive Course for Beginners By : Clarian North
// coding example by Mongkolchai Tue
// https://gist.github.com/MongkolchaiTue/836866552bd222f32cbecc2945b566d2
fn main() {
    //Chapter 5 - Intermediate to Advanced Videos in Rust
    
    //Chapter 5 - Section 6 Reader and Writer Types in Rust - Opening the File
    cpt5_sct05_reader_writer_file();
    
    //Chapter 5 - Section 4 What Are Generic Types in Rust
    cpt5_sct04_generic();
    
    //Chapter 5 - Section 1 Error Handling in Rust
    cpt5_sct01_error_handling();
    
    //Chapter 4 - Section 21 What Are Modules in Rust
    cpt4_sct21_modules();

}


fn cpt5_sct05_reader_writer_file() {
    //Chapter 5 - Section 6 Reader and Writer Types in Rust - Opening the File
    println!("Chapter 5 - Section 6 Reader and Writer Types in Rust - Opening the File");
    
    /*
    Reader and Writer Types in Rust – Opening the File
    
    In Rust, reading from and writing to files are important tasks for many applications,
    from processing data to logging. To read from and write to files in Rust,
    you will need to work with the Reader and Writer types, which provide an interface
    for reading and writing data to files and other sources.
    
    In this video,
    you will learn how to work with the Reader and Writer types in Rust,
    including how to open a file, read data from it, and write data to it.
    */
    
    /*
    Reader and Writer Types (input and outputs in Rust)
    
    Rust's standard library feaures for input and output are organized around two traits - Read, Write
    
    1. Read - Methods for byte-oriented input: readers
        Stdin, File
    
    2. Write - Support both byte-oriented and UTF-8 text output: writers
        Stdout, File
        
    */

    // Exsample read, write file
    use std::fs::File;
    
    // write file
    let file_name = "cat.txt";
    match File::create(file_name) {
        Ok(file) => println!("writing {:?}", file),
        Err(_) => println!("unable to create the file {}", file_name)
    }
    
    // open file
    match File::open(file_name) {
        Ok(file) => println!("reading {:?}", file),
        Err(_) => println!("unable to open the file {}", file_name)
    }
    
    /*
    Section 7
    Reader and Writer Inputs and Outputs in Rust – Reading the File
    
    In Rust, working with files is a crucial aspect of many applications,
    from handling user data to processing large amounts of information.
    To work with files in Rust, you will need to use the Reader and Writer types,
    which provide an interface for reading and writing data to files and other sources.
    
    In this video,
    we will focus on how to read files in Rust using the Reader type,
    including how to open a file, read data from it, and handle errors.
    
    */
    
    // Section 7 - buff reader provides a buffering component for reading
    use std::io::{Error, Read, BufReader};
    // read file
    fn buff_reader() -> Result <(), Error> {
        let file_name = "cat.txt";
        
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        
        reader.read_to_string(&mut contents)?;
        println!("this is the contents..{}", contents);
        Ok(())
    }
    
    let _readfile = buff_reader();

    println!("Chapter 5 - Section 6-7...end");
}

fn cpt5_sct04_generic() {
    //Chapter 5 - Section 4 What Are Generic Types in Rust
    println!("Chapter 5 - Section 4 What Are Generic Types in Rust");
    /*
    Generic Data Types in Rust
    
    We use generics to create definitions for items like function signatures or structs,
    which we can then use with many different concrete data types.
    
    Let's first look at how to define functions, structs, enums, and methods using generics.
    Then we'll discuss how generics affect code performance.
    
    The <T> syntax known as the type parameter, is used to declare generic construct.
    T represents any data-type.
    
    --- Traits --
    trait defines functionsality a particular type has and can share with other types.
    
    We can use traits to define shared behavior in an abstract way (overriding functions).
    We can use trait bounds to specify that a generic type can be any type that has certain behavior (concretely).
    
    Traits are often used to impement a standard set of actions (methods) across multiple structures.
    Traits act like interfaces in OOP.
    
    */
    
    // Generic Struct with T
    struct GenericStruct<T> {
        value:T,
    }
    
    let t1:GenericStruct<i32> = GenericStruct{value:100};
    println!("the value1 is : {}", t1.value);
    
    let t2:GenericStruct<String> = GenericStruct{value:"String data type".to_string()};
    println!("the value2 is : {}", t2.value);
    
    
    // structure
    struct Game {
        weapon:&'static str,
        power_level:u32
    }
    
    // declare a trait
    trait Stats {
        fn character_stats(&self);
    }
    
    // imprement a trait
    impl Stats for Game {
        fn character_stats(&self) {
            println!("Printing stats of power level: {}, weapon: {}", self.power_level, self.weapon)
        }
    }
    
    let g1 = Game {
        power_level: 100,
        weapon: "Sword of Fire"
    };
    
    g1.character_stats();
    
    println!("Chapter 5 - Section 4...end");
}

fn cpt5_sct01_error_handling() {
    //Chapter 5 - Section 1 Error Handling in Rust
    println!("Chapter 5 - Section 1 Error Handling in Rust");
    /*
    Error Handling in Rust
    
    In programming there are recoverable errors.
    Recoverable errors are errors which do not cause the program to fail foe ex:
    'The file type you specified is incorrect'
    
    Unrecoverable errors cause the program to fail such as trying to access locations outside of a datastructure.
    
    This allows a program to terminate immediately and provide feedback to the caller of the program.
    
    This macro is the perfect way to assert conditions in example code and in tests.
    panic! is closely tied with the unwrap method of both Option and Result enums.
    Both implementations call panic! when they are set to None or Err variant
    
    1. unwrap
        unwrap(self): T
        Expects self to be Ok/Some and returns the value contained within.
        If it is Err or None instead, it raises a panic with the contents
    2. expect
        expect(self, msg: &str): T
        
        Behaves like unwrap, except that it outputs a custom message before panicking in addition to the contents of the error.
        
        Enum:
        OK, Err
    */
    
    //---exsample 1
    //panic!("This will cause the program to abruptly end 123");
    //println!("The program is working here");
    
    //---exsample 2
    //use std::fs::File;
    //let f = File::open("doesnotexist.txt").expect("No such thing!");
    //println!("The program is working here {:?}", f);
    
    /*
    Exercise - Write Error Handling for A Program in Rust
    1. Erite a function is_seven which checks whether or not the input is the number 7 and returns true
        if so and an error ie (return Err("...")) if it is not true.
        If it is true return Ok(?)
        - these variats Err and Ok come from the Enum Result.
        The function can return multiple datatypes
        
    2. create a variable 'solution' in the main function which is assigned calling the is_seven function testing various inputs.
    
    3. print the solution variable in the program
    */

    fn is_seven(number:i32) -> Result<bool, String> {
        if number == 7 {
            return Ok(true);
        } else {
            return Err("Error Number not Seven".to_string());
        }
    }
    
    let solution = is_seven(7).unwrap();
    println!("Result from function is_seven is: {}", solution);
    
    println!("Chapter 5 - Section 1...end");
}

// Chapter 4 - Section 21 What Are Modules in Rust
	// The use keyword helps to import a public module.
	// functions are defaulted to private
	// Module Example in Rust
	pub mod songs {
		pub fn play(name: String) {
			println!("track selections: {}", name);
		}
		pub mod song {
			pub fn play(name: String) {
				println!("track selection: {}", name);
			}
		}
	}
	//Exercise - Nested Modules in Rust
    pub mod tracks {
        pub mod rock {
            pub mod indie {
                pub fn select(x:String) {
                    println!("module tracks.rock.indie function select: {}", x);
                }
            }
        }
    }

fn cpt4_sct21_modules() {
    //Chapter 4 - Section 21 What Are Modules in Rust
    println!("Chapter 4 - Section 21 What Are Modules in Rust");
    
    /*
    What Are Modules in Rust
    
    Rust provides a powerful module system that can be used to hierarachically splite code in logical units (modules),
    and manage visiblelity (public/private) between them. A module is a collection of items:
    functions, structs, traits, impl blocks, and even other modules.
    
    Multiple modules are compiled into a unit called crate.
    The Cargo tool is used to manage crates in Rust.
    -------
    1. crate
        Compiling unit in Rust compiled to binary or library.
    2. cargo
        The official Rust package management tool for crates.
    3. module
        a collection of item: 
            functions, structs, traits, impl blocks, and even other modules.
    4. crates.io
        The official Rust package registry.
        
    */
    
    // The use keyword helps to import a public module.
    // functions are defaulted to private
    // Module Example in Rust
    // pub mod songs {
    //     pub fn play(name: String) {
    //         println!("track selections: {}", name);
    //     }
    //     pub mod song {
    //         pub fn play(name: String) {
    //             println!("track selection: {}", name);
    //         }
    //     }
    // }
    
    use songs::song::play;
    use songs::play as play_other;
    play("Kissed By A Rose".to_string());
    play_other("Kissed By Other".to_string());
    
    /*
    Exercise - Nested Modules in Rust
    
    1. create a module named tracks that contains a module named rock
       that contains another module indie which contains a function called select
       which takes a String object as a parameter and prints the string.
       
    2. Import select and in the main function call select three times printing
        the following song titles: "Serenade", "French Baguette" and "Pineapple Blues"
    
    */
    
    // pub mod tracks {
    //     pub mod rock {
    //         pub mod indie {
    //             pub fn select(x:String) {
    //                 println!("module tracks.rock.indie function select: {}", x);
    //             }
    //         }
    //     }
    // }
    
    
    use tracks::rock::indie::select;
    
    select("Serenade".to_string());
    select("French Baguette".to_string());
    select("Pineapple Blues".to_string());

    
    println!("Section 24 - What Are HashMaps in Rust");
    /*
    Section 24 - What Are HashMaps in Rust
    
    Rust's standard collection library has implementation of some of the most common
    data structures for general purposes.
    We've seen vectors previously, now let's take a look at HashMaps!
    
    the type HashMap<K, V> stores a mapping of keys of type K to values of type V.
    It does this via a hashing function, which determines how it places these keys and values into memory.
    Many differrent programming languages support this kind of data structure,
    but often use a different name, such as has, map oject, hash table, or associative array, just to name a few.
    
    Hash maps are useful for when you want to look up data not by an index, as you can with vectors,
    but by using a key that can be of any type. For example, in a game, you could keep track of each team's
    score in a hash map where each key is a team's name and the values are each team's score.
    Given a team name, you can retrieve its score.
    
    No two entries in a map can have the same key. In short, a map is a lookup table.
    
    1. insert()
        pub fn insert(&mut self, k: K, v: V) -> Option
        Inserts a key/value pair
        
    2. len()
        pub fn len(&self) -> usize
        finds and returns how many elements are in the map
        
    3. get()
        pub fn get<Q: ?Size>(&self, k: &Q) -> Option<&V> where K:Borrow Q:Hash+ Eq
        returns a ref to the value of a key
        
    4. iter()
        pub fn iter(&self) -> Iter<K, V>
        Iteration through all key-value pairs
    5.  contains_key
        pub fn contains_key<Q: ?Sized> (&self, k: &Q) -> bool
        Returns bool whehter a value is true for a specified key
    6.  remove()
        pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>
        Removes a key from the map
    
    */
    
    // HashMap Example
    use std::collections::HashMap;
    
    let mut account_info = HashMap::new();
    account_info.insert("Johnny", "Overdraft!");
    account_info.insert("Sally", "Good Standing!");
    account_info.insert("Superman", "Insufficient funds!");
    
    println!("the size of the map is: {}", account_info.len());
    
    /*
    Exercise - Build A hash table with Hashmap in Rust
    
    1. Create a new HashMap instance variable barDreinks and using the insert method
        add three new key pair values to your table: vodka, beer, and whiskey as keys and the values
        should be set up so that vodka and whiskey indicate that the bar has some left while beer has all run out.
    2. Remove whiskey from the local table.
    
    Good luck and keeop those patrons drinking and happy!
    */
    
    let mut bar_drinks = HashMap::new();
    bar_drinks.insert("vodka", true);
    bar_drinks.insert("beer", false);
    bar_drinks.insert("whiskey", true);
    
    println!("the size of the bar_drinks map before we remove one k/v is: {}", bar_drinks.len());
    bar_drinks.remove(&"whiskey");
    println!("the size of the bar_drinks map after we remove one k/v is: {}", bar_drinks.len());
    
    println!("Chapter 4 - Section 21 Modules - 24 HashMaps...end");
}
