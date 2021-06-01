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

}
