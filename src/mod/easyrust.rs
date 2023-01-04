pub mod easy_rust {
    use std::fmt::Debug;
    #[derive(Debug)]
    struct Monster {
        health: i32,
    }
    #[derive(Debug)]
    struct Wizard {
        health: i32,
    }
    #[derive(Debug)]
    struct Ranger {
        health: i32,
    }

    trait Magic {}
    trait FightClose {}
    trait FightFromDistance {}

    impl Magic for Wizard {}
    impl FightClose for Ranger {}
    impl FightClose for Wizard {}
    impl FightFromDistance for Ranger {}

    fn attack_with_bow<T>(character: &T, opponent: &mut Monster, distance: i32)
    where
        T: FightFromDistance + Debug,
    {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow. Your opponent now has {} health left. You are now at: {character:?}"
                , opponent.health
            );
        }
    }

    fn attack_with_sword<T>(character: &T, opponent: &mut Monster)
    where
        T: FightClose + Debug,
    {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, character
        );
    }

    fn fireball<T>(character: &T, opponent: &mut Monster, distance: i32)
    where
        T: Magic + Debug,
    {
        if distance < 15 {
            opponent.health -= 20;
            println!("You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {character:?}"
            ,opponent.health);
        }
    }

    pub fn easy_rust_063() {
        //프로그래밍 언어 러스트를 배웁시다! 063 Easy Rust in Korean: Result
        let radagast = Wizard { health: 60 };
        let aragorn = Ranger { health: 80 };

        let mut uruk_hai = Monster { health: 40 };

        attack_with_sword(&radagast, &mut uruk_hai);
        attack_with_bow(&aragorn, &mut uruk_hai, 7);
        fireball(&radagast, &mut uruk_hai, 12);
    }
}

pub mod easy_rust_1 {
    use std::fmt::Debug;
    use std::fmt::Display;

    #[derive(Debug)]
    struct City {
        name: String,
        population: u32,
    }

    #[derive(Debug)]
    struct Country {
        cities: Vec<City>,
    }

    impl City {
        fn new(name: &str, population: u32) -> Self {
            Self {
                name: name.to_string(),
                population,
            }
        }
    }

    impl From<Vec<City>> for Country {
        fn from(cities: Vec<City>) -> Self {
            Self { cities }
        }
    }

    impl Country {
        fn print_cities(&self) {
            for city in &self.cities {
                println!("{:?} has a population of {:?}", city.name, city.population);
            }
        }
    }
    pub fn easy_rust_64() {
        //프로그래밍 언어 러스트를 배웁시다! 064 Easy Rust in Korean: Result
        let helsinki = City::new("Helsinki", 631_695);
        let turku = City::new("Turku", 186_756);

        let finland_cities = vec![helsinki, turku];
        //let finland = Country::from(finland_cities);
        let finland: Country = finland_cities.into();
        finland.print_cities();
    }
}

pub mod easy_rust_2 {
    use std::fmt::{Debug, Display};

    trait Prints {
        fn debug_print(&self)
        where
            Self: Debug,
        {
            println!("I am: {:?}", self);
        }
        fn display_print(&self)
        where
            Self: Display,
        {
            println!("I am {}", self);
        }
    }

    #[derive(Debug)]
    struct Person;
    #[derive(Debug)]
    struct Building;

    impl<T> Prints for T {}

    pub fn easy_rust_65() {
        //프로그래밍 언어 러스트를 배웁시다! 065 Easy Rust in Korean: Result

        let my_person = Person;
        let my_building = Building;
        my_person.debug_print();
        let my_string = String::from("Hello there");
        my_string.debug_print();
        my_string.display_print();
    }
}

pub mod easy_rust_3 {
    use std::fmt::Debug;

    trait PrintSomething {
        fn print_something(&self) {
            println!("I like to do stuff");
        }
    }

    #[derive(Debug)]
    struct Person;
    struct Building;

    impl<T: std::fmt::Debug> PrintSomething for T {}

    pub fn easy_rust_66() {
        //프로그래밍 언어 러스트를 배웁시다! 066 Easy Rust in Korean: Result
        let person = Person;
        let building = Building;

        person.print_something();
    }
}

pub mod easy_rust_4 {
    use std::fmt::{Debug, Display};

    fn print_it<T: Display + AsRef<str>>(input: T) {
        println!("{input}");
    }

    pub fn easy_rust_67() {
        //프로그래밍 언어 러스트를 배웁시다! 067 Easy Rust in Korean: Result
        print_it("Please print me");
    }
}

pub mod easy_rust_5 {
    pub fn easy_rust_68() {
        //프로그래밍 언어 러스트를 배웁시다! 068 Easy Rust in Korean: Result
        // let new_vec = (1..=10).collect::<Vec<_>>();
        // println!("{new_vec:?}");

        let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let new_vec = my_vec
            .into_iter()
            .skip(1)
            .skip(1)
            .skip(1)
            .take(4)
            .collect::<Vec<i32>>();
        println!("{new_vec:?}");
    }
}

pub mod easy_rust_6 {
    pub fn easy_rust_69() {
        //프로그래밍 언어 러스트를 배웁시다! 069 Easy Rust in Korean: Result
        // Iterator = a collection of things that you can call .next() on
        // .iter() - iterator of references &T
        // .iter_mut() - iterator of mutable reference &mut T
        // .into_iter() - consuming iterator

        let vector1 = vec![1, 2, 3];
        let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
        let vector1_b: Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();

        let mut vector2 = vec![10, 20, 30];
        vector2.iter_mut().for_each(|num| *num += 100);

        println!("{vector1_a:?}");
        println!("{vector2:?}");
        println!("{:?}", vector1_b);
    }
}

pub mod easy_rust_7 {
    pub fn easy_rust_70() {
        //프로그래밍 언어 러스트를 배웁시다! 070 Easy Rust in Korean: Result
        let my_vec = vec!['a', 'b', '거', '亞'];
        let mut my_vec_iter = my_vec.iter();

        assert_eq!(my_vec_iter.next(), Some(&'a'));
        assert_eq!(my_vec_iter.next(), Some(&'b'));
        assert_eq!(my_vec_iter.next(), Some(&'거'));
        assert_eq!(my_vec_iter.next(), Some(&'亞'));
        assert_eq!(my_vec_iter.next(), None);
    }
}

pub mod easy_rust_8 {
    #[derive(Debug)]
    struct Library {
        library_type: LibraryType,
        books: Vec<String>,
    }

    #[derive(Debug)]
    enum LibraryType {
        City,
        Country,
    }

    impl Library {
        fn add_book(&mut self, book: &str) {
            self.books.push(book.to_string());
        }

        fn new() -> Self {
            Self {
                library_type: LibraryType::City,
                books: Vec::new(),
            }
        }
    }

    impl Iterator for Library {
        type Item = String;

        fn next(&mut self) -> Option<String> {
            match self.books.pop() {
                Some(book_title) => Some(book_title + " is Found!"),
                None => None,
            }
        }
    }

    pub fn easy_rust_71() {
        //프로그래밍 언어 러스트를 배웁시다! 071 Easy Rust in Korean: Result
        let mut my_library = Library::new();
        my_library.add_book("The Doom of the Darksword");
        my_library.add_book("Demian - die Geschinhte einer Jugend");
        my_library.add_book("구운몽");

        //println!("{:?}", my_library.books);

        for item in my_library {
            println!("{item}");
        }
    }
}

pub mod easy_rust_9 {
    pub fn easy_rust_72() {
        //프로그래밍 언어 러스트를 배웁시다! 072 Easy Rust in Korean: Result
        let my_closure = || {
            let my_number = 7;
            let other_number = 10;
            println!("The two numbers are {my_number} and {other_number}");
            my_number + other_number
        };

        let my_var = my_closure();
        println!("{my_var}");
    }
}

pub mod easy_rust_10 {
    pub fn easy_rust_73() {
        //프로그래밍 언어 러스트를 배웁시다! 073 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_11 {
    pub fn easy_rust_74() {
        //프로그래밍 언어 러스트를 배웁시다! 074 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_12 {
    pub fn easy_rust_75() {
        //프로그래밍 언어 러스트를 배웁시다! 075 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_13 {
    pub fn easy_rust_76() {
        //프로그래밍 언어 러스트를 배웁시다! 076 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_14 {
    pub fn easy_rust_77() {
        //프로그래밍 언어 러스트를 배웁시다! 077 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_15 {
    pub fn easy_rust_78() {
        //프로그래밍 언어 러스트를 배웁시다! 078 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_16 {
    pub fn easy_rust_79() {
        //프로그래밍 언어 러스트를 배웁시다! 079 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_17 {
    pub fn easy_rust_80() {
        //프로그래밍 언어 러스트를 배웁시다! 080 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_18 {
    pub fn easy_rust_81() {
        //프로그래밍 언어 러스트를 배웁시다! 081 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_19 {
    pub fn easy_rust_82() {
        //프로그래밍 언어 러스트를 배웁시다! 082 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_20 {
    pub fn easy_rust_83() {
        //프로그래밍 언어 러스트를 배웁시다! 083 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_21 {
    pub fn easy_rust_84() {
        //프로그래밍 언어 러스트를 배웁시다! 084 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_22 {
    pub fn easy_rust_85() {
        //프로그래밍 언어 러스트를 배웁시다! 085 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_23 {
    pub fn easy_rust_86() {
        //프로그래밍 언어 러스트를 배웁시다! 086 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_24 {
    pub fn easy_rust_87() {
        //프로그래밍 언어 러스트를 배웁시다! 087 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_25 {
    pub fn easy_rust_88() {
        //프로그래밍 언어 러스트를 배웁시다! 088 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_26 {
    pub fn easy_rust_89() {
        //프로그래밍 언어 러스트를 배웁시다! 089 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_27 {
    pub fn easy_rust_90() {
        //프로그래밍 언어 러스트를 배웁시다! 090 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_28 {
    pub fn easy_rust_91() {
        //프로그래밍 언어 러스트를 배웁시다! 091 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_29 {
    pub fn easy_rust_92() {
        //프로그래밍 언어 러스트를 배웁시다! 092 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_30 {
    pub fn easy_rust_93() {
        //프로그래밍 언어 러스트를 배웁시다! 093 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_31 {
    pub fn easy_rust_94() {
        //프로그래밍 언어 러스트를 배웁시다! 094 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_32 {
    pub fn easy_rust_95() {
        //프로그래밍 언어 러스트를 배웁시다! 095 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_33 {
    pub fn easy_rust_96() {
        //프로그래밍 언어 러스트를 배웁시다! 096 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_34 {
    pub fn easy_rust_97() {
        //프로그래밍 언어 러스트를 배웁시다! 097 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_35 {
    pub fn easy_rust_98() {
        //프로그래밍 언어 러스트를 배웁시다! 098 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_36 {
    pub fn easy_rust_99() {
        //프로그래밍 언어 러스트를 배웁시다! 099 Easy Rust in Korean: Result
    }
}

pub mod easy_rust_37 {
    pub fn easy_rust_100() {
        //프로그래밍 언어 러스트를 배웁시다! 100 Easy Rust in Korean: Result
    }
}
