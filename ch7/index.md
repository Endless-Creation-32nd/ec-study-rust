# Managing Growing Projects with Packages, Crates, and Modules

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

When we entered the command, Cargo created a Cargo.toml file, giving us a package. Looking at the contents of Cargo.toml, there’s no mention of src/main.rs because Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects. For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers. We can use that functionality in our own projects by bringing the rand crate into our project’s scope. All the functionality provided by the rand crate is accessible through the crate’s name, rand.

Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the rand crate and prevents potential conflicts. For example, the rand crate provides a trait named Rng. We can also define a struct named Rng in our own crate. Because a crate’s functionality is namespaced in its own scope, when we add rand as a dependency, the compiler isn’t confused about what the name Rng refers to. In our crate, it refers to the struct Rng that we defined. We would access the Rng trait from the rand crate as rand::Rng.

### Defining Modules to Control Scope and Privacy

Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

```
cargo new --lib restaurant
```

restaurant/src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

crate
└── front_of_house
... ├── hosting
... │...├── add_to_waitlist
... │...└── seat_at_table
... └── serving
... ... ├── take_order
... ... ├── serve_order
... ... └── take_payment

We define a module by starting with the mod keyword and then specify the name of the module (in this case, front_of_house) and place curly brackets around the body of the module. Inside modules, we can have other modules, as in this case with the modules hosting and serving. Modules can also hold definitions for other items, such as structs, enums, constants, traits, or—as in Listing 7-1—functions.

By using modules, we can group related definitions together and name why they’re related. Programmers using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather than having to read through all the definitions. Programmers adding new functionality to this code would know where to place the code to keep the program organized.

Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.

### Paths for Referring to an Item in the Module Tree

- An **absolute path** starts from a crate root by using a crate name or a literal crate.
- A **relative path** starts from the current module and uses self, super, or an identifier in the current module.

### Exposing Paths with the pub Keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### Starting Relative Paths with super

We can also construct relative paths that begin in the parent module by using super at the start of the path. This is like starting a filesystem path with the .. syntax. Why would we want to do this?

Consider the code in Listing 7-8 that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function fix_incorrect_order calls the function serve_order by specifying the path to serve_order starting with super:

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public

We can also use pub to designate structs and enums as public, but there are a few extra details. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. In Listing 7-9, we’ve defined a public back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field. This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal based on what’s in season and in stock. The available fruit changes quickly, so customers can’t choose the fruit or even see which fruit they’ll get.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword, as shown in Listing 7-10.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant. Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

There’s one more situation involving pub that we haven’t covered, and that is our last module system feature: the use keyword. We’ll cover use by itself first, and then we’ll show how to combine pub and use.

## Bringing Paths into Scope with the use Keyword

We can bring a path into a scope once and then call the items in that path as if they’re local items with the use keyword.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### Creating Idiomatic use Paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with use. Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. The code in Listing 7-13 is unclear as to where add_to_waitlist is defined.

On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

### Providing New Names with the as keyword

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with pub use

When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist. If we hadn’t specified pub use, the eat_at_restaurant function could call hosting::add_to_waitlist in its scope, but external code couldn’t take advantage of this new path.

With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.

### Using External Packages

In Chapter 2, we programmed a guessing game project that used an external package called rand to get random numbers. To use rand in our project, we added this line to Cargo.toml:

Filename: Cargo.toml

rand = "0.8.3"
Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.

Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope. Recall that in the “Generating a Random Number” section in Chapter 2, we brought the Rng trait into scope and called the rand::thread_rng function:

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps: listing them in your package’s Cargo.toml file and using use to bring items from their crates into scope.

Note that the standard library (std) is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std. But we do need to refer to it with use to bring items from there into our package’s scope. For example, with HashMap we would use this line:

```rust
use std::collections::HashMap;
```

This is an absolute path starting with std, the name of the standard library crate.

### Using Nested Paths to Clean Up Large use Lists

```rust
use std::cmp::Ordering;
use std::io;
-----------------------------------
use std::{cmp::Ordering, io};
```

```rust
use std::io;
use std::io::Write;
-----------------------------------
use std::io::{self, Write};
```

### The Glob Operator

If we want to bring all public items defined in a path into scope, we can specify that path followed by \*, the glob operator:

```rust
use std::collections::*;
```

This use statement brings all public items defined in std::collections into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test into the tests module; we’ll talk about that in the “How to Write Tests” section in Chapter 11. The glob operator is also sometimes used as part of the prelude pattern: see the standard library documentation for more information on that pattern.

## Separating Modules into Different Files

- 만일 foo라는 이름의 모듈이 서브모듈을 가지고 있지 않다면, foo.rs라는 이름의 파일 내에 foo에 대한 선언을 집어넣어야 합니다.
- 만일 foo가 서브모듈을 가지고 있다면, foo/mod.rs라는 이름의 파일에 foo에 대한 선언을 집어넣어야 합니다.
