use std::collections::HashMap;

fn main() {
    println!("Hello, world!");



    // Enums and Patterns

    // Enums are useful whenever a value might be either one thing or another. The "price" of using them is that we must access the data safely, using pattern matching.

    // Rust patterns are a little like regular expressions for all our data. They're used to test whether or not a value has a particular desired shape. They can extract several fields from a struct or tuple into local variables all at once. Like regular expressions, they are concise, typically doing it all in a single line of code.



    // Enums

    // Simple, C-style enums are straightforward:
    enum Ordering {
        Less,
        Equal,
        Greater
    }

    // This declares a type Ordering with three possible values, called variants or constructors. Ordering::less, Ordering::Equal, and Ordering::Greater. This particular enum is part of the standard library, so Rust code can import it by itself:
    use std::cmp::Ordering;

    fn compare(n: i32, m: i32) -> Ordering {
        if n < m {
            Ordering::Less
        } else if n > m {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    // Or with all its constructors:
    use std::cmp::Ordering;
    use std::cmp::Ordering::*; // `*` to import all children

    fn compare(n: i32, m: i32) -> Ordering {
        if n < m {
            Less
        } else if n > m {
            Greater
        } else {
            Equal
        }
    }

    // After importing the constructors, we can write Less instead of Ordering::Less, and so on. But because this is less explicit, it's generally considered better style not to import them except when it makes our code much more readable.

    // To import the constructors of an enum declared in the current module, use a self import:
    enum Pet {
        Orca,
        Giraffe,
        ...
    }

    use self::Pet::*;

    // In memory, values of C-style enums are stored as integers. Occasionally it's useful to tell Rust which integers to use:
    enum HttpStatus {
        Ok = 200,
        NotModified = 304,
        NotFound = 404,
        ...
    }

    // Otherwise Rust will assign the numbers for us, starting at 0.

    // By default, Rust stores C-style enums using the smallest built-in integer type that can accommodate them. Most fit in a single byte:
    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2); // 404 doesn't fit in a u8

    // We can override Rust's choice of in-memory representation by adding a #[repr] attribute to the enum. More on that in chapt 21.

    // Casting a C-style enum to an integer is allowed:
    assert_eq!(HttpStatus::Ok as i32, 200);

    // However, casting in the other direction, from the integer to the enum, is not. Unlike C and C++, Rust guarantees that an enum value is only ever one of the values spelled out in the enum declaration. An unchecked cast from an integer type to an enum type could break this guarantee, so it's not allowed. We can either write our own checked conversion:
    fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
        match n {
            200 => Some(HttpStatus::Ok),
            304 => Some(HttpStatus::NotModified),
            404 => Some(HttpStatus::NotFound),
            ...
            _ => None
        }
    }

    // or use the enum_primitive crate (crates.io). It contains a macro that autogenerates this kind of conversion code for us.

    // As with structs, the compiler will implement features like the == operator for us, but we have to ask.
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TimeUnit {
        Seconds, Minutes, Hours, Days, Months, Years
    }

    // Enums can have methods, just like structs:
    impl TimeUnit {
        /// Return the plural noun for this time unit.
        fn plural(self) -> &'static str {
            match self {
                TimeUnit::Seconds => "seconds",
                TimeUnit::Minutes => "minutes",
                TimeUnit::Hours => "hours",
                TimeUnit::Days => "days",
                TimeUnit::Months => "months",
                TimeUnit::Years => "years"
            }
        }

        /// Return the singular noun for this time unit
        fn singular(self) -> &'static str {
            self.plural().trim_right_matches('s')
        }
    }



    // Enums with Data

    // Some programs always need to display full dates and times down to the millisecond. But for most apps, it's more user-friendly to use a rough approximation, like "two months ago". We can write an enum to help with that:
    /// A timestamp that has been deliberately rounded off, so our program
    /// says "6 months ago" instead of "February 9, 2016, at 9:49 AM".
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum RoughTime {
        InThePast(TimeUnit, u32),
        JustNow,
        InTheFuture(TimeUnit, u32)
    }

    // Two of the variants in this enum, InThePast and InTheFuture, take arguments. These are called tuple variants. Like tuple structs, these constructors are functions that create new RoughTime values.
    let four_score_and_seven_years_ago =
        RoughTime::InThePast(TimeUnit::Years, 4*20 + 7);

    let three_hours_from_now =
        RoughTime::InTheFuture(TimeUnit::Hours, 3);

    // Enums can also have struct variants, which contain named fields, just like ordinary structs:
    enum Shape {
        Sphere { center: Point3d, radius: f32 },
        Cuboid { corner1: Point3d, corner2: Point3d }
    }

    let unit_sphere = Shape::Sphere { center: ORIGIN, radius: 1.0 };

    // In all, Rust has three kinds of enum variant, echoing the three kinds of struct shown in the prev chapt. Variants with no data correspond to unit-like structs. Tuple variants look and function just like tuple structs. Struct variants have curly braces and named fields. A single enum can have variants of all three kinds.
    enum RelationshipStatus {
        Single,
        InARelationship,
        ItsComplicated(Option<String>),
        ItsExtremelyComplicated {
            car: DifferentialEquation,
            crd: EarlyModernistPoem
        }
    }

    // All constructors and fields of a public enum are automatically public.



    // Enums in Memory

    // In memory, enums with data are stored as a small integer tag, plus enough memory to hold all the fields of the largest variant. The tag field is for Rust's internal use. It tells which constructor created the value, and therefore which fields it has. See page 341 for a diagram.

    // Rust makes no promises about enum layout, however, in order to leave the door open for future optimizations. In some cases, it would be possible to pack an enum more efficiently than the figure suggests. Later in the chapter we'll show how Rust can optimize away the tag field for some enums.



    // Rich Data Structures Using Enums

    // Enums are also useful for quickly implementing tree-like data structures. For example, suppose a Rust program needs to work with arbitrary JSON data. In memory, any JSON document can be represented as a value of this Rust type:
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        Object(Box<HashMap<String, Json>>)
    }

    // The JSON standard specifies the various data types that can appear in a JSON document. null, Boolean values, numbers, strings, arrays of JSON values, and objects with string keys and JSON values. The Json enum simply spells out these types.

    // The Box around the HashMap that represents an Object serves only to make all Json values more compact. In memory, values of type Json take up four machine words. String and Vec values are three words, and Rust adds a tag byte. Null and Boolean values don't have enough data in them to use up all that space, but Json values must be the same size. The extra space goes unused. See page 342 for diagram.

    // A HashMap is larger still. If we had to leave room for it in every Json value, they would be quite large, eight words or so. But a Box<HashMap> is a single word. It's just a pointer to heap-allocated data. We could make Json even more compact by boxing more fields.

    // If we were to write the equivalent in C++ we'd need at least 30 lines of code (see page 343 and 344) just to start. By the time it's written out, the C++ library will require studying to understand how everything works. This is compared to Rust's 8 lines of code.



    // Generic Enums

    // Enums can be generic. Two examples from the standard library are among the most-used data types in the language:
    enum Option<T> {
        None,
        Some(T)
    }

    enum Result<T, E> {
        Ok(T),
        Err(E)
    }

    // These types are familiar enough by now, and the syntax for generic enums is the same as for generic structs. One unobvious detail is that Rust can eliminate the tag field of Option<T> when the type T is a Box or some other smart pointer type. An Option<Box<i32>> is stored in memory as a single machine word, 0 for None and nonzero for Some boxed value.

    // Generic data structures can be built with just a few lines of code:
    // An ordered collection of `T`s.
    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>)
    }

    // A part of a BinaryTree.
    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>
    }

    // These few lines of code define a BinaryTree type that can store any number of values of type T.

    // Each BinaryTree value is either Empty or NonEmpty. If it's Empty, then it contains no data at all. If NonEmpty, then it has a Box, a pointer to a heap-allocated TreeNode.

    // Each TreeNode value contains one actual element, as well as two more BinaryTree values. This means a tree can contain subtrees, and thus a NonEmpty tree can have any number of descendants.

    // See page 347 for a diagram. As with Option<Box<T>>, Rust eliminates the tag fields, so a BinaryTree value is just one machine word.

    // See diagram, building any particular node in the tree is straightforward:
    use self::BinaryTree::*;
    
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty
    }));

    // Larger trees can be built from smaller ones:
    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree
    }));

    // This assignment transfer ownership of jupiter_node and mercury_node to their new parent node.

    // The remaining parts of the tree follow the same patterns. The root node is no different from the others:
    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree
    }));

    // Later in the chapter, we'll see how to implement an add method on the BinaryTree type so that we can instead write:
    let mut tree = BinaryTree::Empty;
    for planet in planets {
        tree.add(planet);
    }

    // No matter what language we're coming from, creating data structures like BinaryTree in Rust will likely take some practice. It won't be obvious at first where to put the Boxes. One way to find a design that will work is to draw a picture (see earlier diagram, pg 346) that shows how we want things laid out in memory. Then work backward from the picture to the code. Each collection of rectangles is a struct or tuple. Each arrow is a Box or other smart pointer. Figuring out the type of each field is a bit of a puzzle, but a manageable one. The reward is control over our program's memory usage.

    // Now comes the "price" mentioned in the into. The tag field of an enum costs a little memory, up to 8 bytes in the worst case, but that is usually negligible. The real downside to enums (if it can be called that) is that Rust code cannot throw caution to the wind and try to access fields regardless of whether they are actually present in the value:
    let r = shape.radius; // error: no field `radius` on type `Shape`

    // The only way to access the data in an enum is the safe way, using patterns.




}
