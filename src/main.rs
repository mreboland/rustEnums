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


}
