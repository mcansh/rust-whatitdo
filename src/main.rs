use std::cmp::min;

fn main() {
    {
        let x = 42;
        println!("The value of x is: {}", x);
    }

    {
        let pair = ("a", 17);
        let (label, value) = pair;
        println!("The value of label is: {}", label);
        println!("The value of value is: {}", value);
    }

    {
        let slice = [1, 2, 3, 4, 5];
        let middle = 3;
        let (l, r) = slice.split_at(middle);
        println!("The value of l is: {:?}", l);
        println!("The value of r is: {:?}", r);
    }

    {
        let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
            .iter()
            .map(|x| x + 3)
            .fold(0, |x, y| x + y);

        println!("The value of x is: {}", x);
    }

    {
        let least = min(7, 3);
        println!("The value of least is: {}", least);
    }

    {
        struct Number {
            odd: bool,
            value: i32,
        }

        let x = Number {
            odd: false,
            value: 2,
        };
        let y = Number {
            value: 3,
            odd: true,
        };

        fn print_number(x: Number) {
            println!(
                "The value of x is: {}, and it is {}",
                x.value,
                if x.odd { "odd" } else { "even" }
            );
        }

        print_number(x);
        print_number(y);
    }

    {
        struct Number {
            odd: bool,
            value: i32,
        }

        let x = Number {
            odd: false,
            value: 2,
        };
        let y = Number {
            value: 3,
            odd: true,
        };

        fn print_number(n: Number) {
            match n.value {
                1 => println!("One!"),
                2 => println!("Two!"),
                _ => println!("{}", n.value),
            }
        }

        print_number(x);
        print_number(y);
    }

    {
        struct Number {
            odd: bool,
            value: i32,
        }

        impl Number {
            fn is_positive(self) -> bool {
                self.value > 0
            }
        }

        let minus_two = Number {
            odd: false,
            value: -2,
        };

        print!("{}", minus_two.is_positive());
    }

    {
        struct Number {
            odd: bool,
            value: i32,
        }

        let mut n = Number {
            odd: true,
            value: 17,
        };

        println!(
            "The value of n is: {}, and it is {}",
            n.value,
            if n.odd { "odd" } else { "even" }
        );

        n.value = 19;

        println!(
            "The value of n is: {}, and it is {}",
            n.value,
            if n.odd { "odd" } else { "even" }
        );
    }

    {
        fn foobar<T>(x: T) -> T {
            x
        }

        let x = foobar(42);
        println!("The value of x is: {}", x);

        struct Pair<T> {
            a: T,
            b: T,
        }

        let mut p1 = Pair { a: 1, b: 2 };
        let mut p2 = Pair { a: true, b: false };

        fn swap<T>(pair: Pair<T>) -> Pair<T> {
            Pair {
                a: pair.b,
                b: pair.a,
            }
        }

        println!("The value of p3 is: ({}, {})", p1.a, p1.b);
        println!("The value of p4 is: ({}, {})", p2.a, p2.b);

        p1 = swap(p1);
        p2 = swap(p2);

        println!("The value of p3 is: ({}, {})", p1.a, p1.b);
        println!("The value of p4 is: ({}, {})", p2.a, p2.b);
    }

    {
        let mut v1 = Vec::new();
        v1.push(1);

        let mut v2 = vec![];
        v2.push(false);

        println!("The value of v1 is: {:?}", v1);
        println!("The value of v2 is: {:?}", v2);
    }

    // {
    //     panic!("crash and burn");
    // }

    {
        let o1: Option<i32> = Some(128);
        o1.unwrap();

        let o2: Option<i32> = None;
        // o2.unwrap();
    }

    // {
    //     let s = std::str::from_utf8(&[195, 40]).expect("valid utf-8");
    //     println!("The value of s is: {}", s)
    // }

    {
        match_error();
        if_let_error();
        let _ = bubble_up_error();
        let _ = easy_bubble_error();
    }

    {
        // 0 or greater
        let first = (0..).contains(&100); // true
                                          // 20 or less than 20
        let second = (..=20).contains(&20); // true
                                            // only 3, 4, 5
        let third = (3..6).contains(&4); // true

        println!("The value of first is: {}", first);
        println!("The value of second is: {}", second);
        println!("The value of third is: {}", third);
    }

    {
        for i in vec![52, 49, 21] {
            println!("The value of i is: {}", i);
        }

        for i in &[52, 49, 21] {
            println!("The value of i is: {}", i);
        }

        for c in "rust".chars() {
            println!("Give me a {}", c);
        }
    }

    {
        for c in "SuRpRiSE InBOuNd"
            .chars()
            .filter(|c| c.is_lowercase())
            .flat_map(|c| c.to_uppercase())
        {
            print!("{}", c);
        }
    }
}

fn match_error() {
    let melon = &[240, 159, 141, 137];
    match std::str::from_utf8(melon) {
        Ok(s) => println!("The value of s is: {}", s),
        Err(e) => println!("The melon is not valid utf-8: {}", e),
    }
}

fn if_let_error() {
    let melon = &[240, 159, 141, 137];
    if let Ok(s) = std::str::from_utf8(melon) {
        println!("The value of s is: {}", s);
    }
}

fn bubble_up_error() -> Result<(), std::str::Utf8Error> {
    let melon = &[240, 159, 141, 137];
    match std::str::from_utf8(melon) {
        Ok(s) => println!("The value of s is: {}", s),
        Err(e) => return Err(e),
    }

    Ok(())
}

fn easy_bubble_error() -> Result<(), std::str::Utf8Error> {
    let melon = &[240, 159, 141, 137];
    let s = std::str::from_utf8(melon)?;
    println!("The value of s is: {}", s);
    Ok(())
}
