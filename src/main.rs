#![allow(warnings)]

#[allow(non_snake_case)]
use std::io;

#[path = "mod/easyrust.rs"]
mod easy_rust;
#[path = "mod/easyrust.rs"]
mod easy_rust_1;
#[path = "mod/easyrust.rs"]
mod easy_rust_10;
#[path = "mod/easyrust.rs"]
mod easy_rust_11;
#[path = "mod/easyrust.rs"]
mod easy_rust_12;
#[path = "mod/easyrust.rs"]
mod easy_rust_13;
#[path = "mod/easyrust.rs"]
mod easy_rust_14;
#[path = "mod/easyrust.rs"]
mod easy_rust_15;
#[path = "mod/easyrust.rs"]
mod easy_rust_16;
#[path = "mod/easyrust.rs"]
mod easy_rust_17;
#[path = "mod/easyrust.rs"]
mod easy_rust_18;
#[path = "mod/easyrust.rs"]
mod easy_rust_19;
#[path = "mod/easyrust.rs"]
mod easy_rust_2;
#[path = "mod/easyrust.rs"]
mod easy_rust_20;
#[path = "mod/easyrust.rs"]
mod easy_rust_21;
#[path = "mod/easyrust.rs"]
mod easy_rust_22;
#[path = "mod/easyrust.rs"]
mod easy_rust_23;
#[path = "mod/easyrust.rs"]
mod easy_rust_24;
#[path = "mod/easyrust.rs"]
mod easy_rust_25;
#[path = "mod/easyrust.rs"]
mod easy_rust_26;
#[path = "mod/easyrust.rs"]
mod easy_rust_27;
#[path = "mod/easyrust.rs"]
mod easy_rust_28;
#[path = "mod/easyrust.rs"]
mod easy_rust_29;
#[path = "mod/easyrust.rs"]
mod easy_rust_3;
#[path = "mod/easyrust.rs"]
mod easy_rust_30;
#[path = "mod/easyrust.rs"]
mod easy_rust_31;
#[path = "mod/easyrust.rs"]
mod easy_rust_32;
#[path = "mod/easyrust.rs"]
mod easy_rust_33;
#[path = "mod/easyrust.rs"]
mod easy_rust_34;
#[path = "mod/easyrust.rs"]
mod easy_rust_35;
#[path = "mod/easyrust.rs"]
mod easy_rust_36;
#[path = "mod/easyrust.rs"]
mod easy_rust_37;
#[path = "mod/easyrust.rs"]
mod easy_rust_4;
#[path = "mod/easyrust.rs"]
mod easy_rust_5;
#[path = "mod/easyrust.rs"]
mod easy_rust_6;
#[path = "mod/easyrust.rs"]
mod easy_rust_7;
#[path = "mod/easyrust.rs"]
mod easy_rust_8;
#[path = "mod/easyrust.rs"]
mod easy_rust_9;

use easy_rust::easy_rust::easy_rust_063;
use easy_rust_1::easy_rust_1::easy_rust_64;
use easy_rust_10::easy_rust_10::easy_rust_73;
use easy_rust_11::easy_rust_11::easy_rust_74;
use easy_rust_12::easy_rust_12::easy_rust_75;
use easy_rust_13::easy_rust_13::easy_rust_76;
use easy_rust_14::easy_rust_14::easy_rust_77;
use easy_rust_15::easy_rust_15::easy_rust_78;
use easy_rust_16::easy_rust_16::easy_rust_79;
use easy_rust_17::easy_rust_17::easy_rust_80;
use easy_rust_18::easy_rust_18::easy_rust_81;
use easy_rust_19::easy_rust_19::easy_rust_82;
use easy_rust_20::easy_rust_20::easy_rust_83;
use easy_rust_21::easy_rust_21::easy_rust_84;
use easy_rust_22::easy_rust_22::easy_rust_85;
use easy_rust_23::easy_rust_23::easy_rust_86;
use easy_rust_24::easy_rust_24::easy_rust_87;
use easy_rust_25::easy_rust_25::easy_rust_88;
use easy_rust_26::easy_rust_26::easy_rust_89;
use easy_rust_27::easy_rust_27::easy_rust_90;
use easy_rust_28::easy_rust_28::easy_rust_91;
use easy_rust_29::easy_rust_29::easy_rust_92;
use easy_rust_30::easy_rust_30::easy_rust_93;
use easy_rust_31::easy_rust_31::easy_rust_94;
use easy_rust_32::easy_rust_32::easy_rust_95;
use easy_rust_33::easy_rust_33::easy_rust_96;
use easy_rust_34::easy_rust_34::easy_rust_97;
use easy_rust_35::easy_rust_35::easy_rust_98;
use easy_rust_36::easy_rust_36::easy_rust_99;
use easy_rust_37::easy_rust_37::easy_rust_100;

use easy_rust_2::easy_rust_2::easy_rust_65;
use easy_rust_3::easy_rust_3::easy_rust_66;
use easy_rust_4::easy_rust_4::easy_rust_67;
use easy_rust_5::easy_rust_5::easy_rust_68;
use easy_rust_6::easy_rust_6::easy_rust_69;
use easy_rust_7::easy_rust_7::easy_rust_70;
use easy_rust_8::easy_rust_8::easy_rust_71;
use easy_rust_9::easy_rust_9::easy_rust_72;

enum Menu {
    AddressInput(u8),
    AddressSearch(u8),
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct HumanAddressData {
    UserName: String,
    Address: String,
}

impl HumanAddressData {
    pub fn ShowInfomation(&self) {
        println!("addressInfo : [{:3}] / [{}]", self.UserName, self.Address);
    }
}

pub struct HumanAddressInfomation {
    InfomationDatas: Vec<HumanAddressData>,
}

impl HumanAddressInfomation {
    pub fn addString(&mut self, name: String, address: String) {
        let data = HumanAddressData {
            UserName: name,
            Address: address,
        };

        self.InfomationDatas.push(data);
    }

    pub fn addTuple(&mut self, tuple: (&str, &str)) {
        let data = HumanAddressData {
            UserName: tuple.0.to_string(),
            Address: tuple.1.to_string(),
        };

        self.InfomationDatas.push(data);
    }

    pub fn sort(&mut self) {
        if self.InfomationDatas.len() % 3 == 0 {
            //self.InfomationDatas.sort_by(|first, next|  );
            self.InfomationDatas.sort();
        }
    }

    pub fn remove(&mut self, name: &str) -> bool {
        true
    }

    pub fn showAllData(&self) {
        for data in self.InfomationDatas.iter() {
            data.ShowInfomation();
        }
    }
}

fn Testcode() {
    let mut name = String::new();

    name = String::from("남현준");
    println!("{:?}", name.chars().next().unwrap());
    println!("{:?}", name.into_bytes().get(0..1).unwrap());
}

use HumanAddressInfomation as AddressInfoSys;

fn easy_rust_1() {
    let str_array = ["one", "two", "three"];

    let [a, _, _] = str_array;

    println!("Item a is: {}", a);
}
fn easy_rust_2() {
    let my_number = 5;

    let second_number = match my_number {
        0 => 23,
        1 => 65,
        _ => 0,
    };

    println!("The second number is: {}", second_number);
}
fn easy_rust_3() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure what the weather is."),
    }
}
fn easy_rust_4() {
    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if married == false => {
            println!("Not married with {} children", children)
        }
        (c, m) if c == 0 && married => println!("Married but with no children"),
        _ => println!("Some other type of marriage and children combination"),
    }
}
fn match_colours(rgb: (u32, u32, u32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Every colours has at least 10"),
    }
}
fn easy_rust_5() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}
fn match_number(input: i32) {
    match input {
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number),
        _ => println!("It's greater than ten"),
    }
}
fn easy_rust_6() {
    match_number(10);
    match_number(100);
}
fn easy_rust_7() {
    //unit struct
    struct FireDirectory;
    //tuple struct
    #[derive(Debug)]
    struct Colour(u8, u8, u8);
    //name struct
    #[derive(Debug)]
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
    }

    // let my_colour = Colour(20, 50, 100);
    // println!("The second colour is {:?}", my_colour);

    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Ustin Trudeau".to_string(),
    };

    //println!("The population is: {}\nThe capital is: {}",canada.population,canada.capital);
    println!("The country is : {:#?}", canada);
    //033 Easy Rust in Korean: Struct size
}
fn easy_rust_8() {
    use std::mem::size_of_val;

    #[derive(Debug)]
    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
    }

    let population = 35_000_000;
    let capital = "Ottawa".to_string();
    let leader_name = "Justin Trudeau".to_string();

    let my_country = Country {
        population,
        capital,
        leader_name,
    };

    println!("Country is {} bytes in size", size_of_val(&my_country));
}
fn easy_rust_9() {
    enum ThingsInTheSky {
        Sun,
        Stars,
    }

    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun,
            _ => ThingsInTheSky::Stars,
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun => println!("I can see the sun"),
            ThingsInTheSky::Stars => println!("I can see the stars"),
        }
    }

    let time = 8;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);
    check_skystate(&create_skystate(20));
}
fn easy_rust_10() {
    enum Mood {
        Happy,
        Sleepy,
        NotBad,
        Angry,
    }
    use Mood::*;

    fn match_mood(mood: &Mood) -> i32 {
        match mood {
            Happy => 10,
            Sleepy => 6,
            NotBad => 7,
            Angry => 2,
        }
    }

    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);

    enum Season {
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("The number is: {}", season as u32);
    }
}
fn easy_rust_11() {
    enum Star {
        BrownDwarf = 10,
        RedDwarf = 50,
        YellowStar = 100,
        RedGiant = 1000,
        DeadStar,
    }
    use Star::*;

    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];

    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star: {}", size),
            size if size >= 80 => println!("Pretty big star: {}", size),
            _ => println!("Some other star"),
        }
    }

    println!("What about DeadStar? It is: {}", DeadStar as u32);

    enum Number {
        U32(u32),
        I32(i32),
    }

    fn get_number(input: i32) -> Number {
        match input.is_positive() {
            true => Number::U32(input as u32),
            false => Number::I32(input),
        }
    }

    let my_vec = vec![get_number(-800), get_number(8)];

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number),
        }
    }
}
fn easy_rust_12() {
    let mut counter = 0;
    let mut counter2 = 0;

    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 9 {
            println!("Now entering the second loop");

            'second_loop: loop {
                println!("The second counter is: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}
fn easy_rust_13() {
    let mut counter = 0;

    while counter != 5 {
        counter += 1;
        println!("The counter is now: {}", counter);
    }

    for number in 0..=3 {
        println!("The number is {}", number);
    }

    counter = 5;

    let mut my_number = loop {
        counter += 1;
        if counter % 53 == 3 {
            break counter;
        }
    };

    println!("my_number is now: {}", my_number);
}
fn easy_rust_14() {
    #[derive(Debug)]
    struct Animal {
        age: u8,
        animal_type: AnimalType,
    }
    #[derive(Debug)]
    enum AnimalType {
        Cat,
        Dog,
    }
    impl AnimalType {
        fn get_name(&self) {
            match self {
                AnimalType::Cat => println!("Animal Type is cat"),
                AnimalType::Dog => println!("Animal Type is dog"),
            }
        }
    }

    impl Animal {
        fn new_cat(age: u8) -> Self {
            Self {
                age,
                animal_type: AnimalType::Cat,
            }
        }

        fn new_dog(age: u8) -> Self {
            Self {
                age,
                animal_type: AnimalType::Dog,
            }
        }

        fn print(&self) {
            println!("I am a: {:?}", self);
        }
        fn change_to_dog(&mut self) {
            self.animal_type = AnimalType::Dog;
            println!("Changed to cat! Now I am: {:?}", self);
        }
        fn change_to_cat(&mut self) {
            self.animal_type = AnimalType::Cat;
            println!("Changed to dog! Now I am: {:?}", self);
        }
    }

    let mut my_animal = Animal::new_dog(10);
    my_animal.print();
    my_animal.change_to_cat();
    my_animal.change_to_dog();
}
fn easy_rust_15() {
    #[derive(Debug)]
    struct Animal {
        age: u8,
        animal_type: AnimalType,
    }
    #[derive(Debug)]
    enum AnimalType {
        Cat(String),
        Dog(String),
    }

    impl AnimalType {
        fn print_name(&self) {
            match self {
                AnimalType::Cat(name) => println!("Animal type is cat and name is: {}", name),
                AnimalType::Dog(name) => println!("Animal type is dog and name is: {}", name),
            }
        }
    }
    impl Animal {
        fn new(age: u8, animal_type: AnimalType) -> Self {
            Self { age, animal_type }
        }
    }

    let my_cat = Animal::new(10, AnimalType::Cat("Windy".to_string()));

    my_cat.animal_type.print_name();
}
fn easy_rust_16() {
    struct Person {
        name: String,
        real_name: String,
        height: u8,
        happiness: bool,
    }

    struct Person2 {
        name: String,
        height: u8,
    }

    impl Person {
        fn clone(&self) -> Self {
            Self {
                name: self.name.clone(),
                real_name: self.real_name.clone(),
                height: self.height,
                happiness: self.happiness,
            }
        }
    }
    impl Person2 {
        fn from_person(input: Person) -> Self {
            let Person { name, height, .. } = input;
            Self { name, height }
        }
    }
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let papa_doc2 = papa_doc.clone();

    let Person {
        name,
        real_name,
        height,
        happiness,
    } = papa_doc;

    println!(
        "They call him {} but his real name is {}. He is {} tall and is he happy? {}",
        name, real_name, height, happiness
    );

    let person2 = Person2::from_person(papa_doc2);
}
fn easy_rust_17() {
    struct Item {
        number: u8,
    }
    impl Item {
        fn compare_number(&self, other_number: u8) {
            println!("Are they qual? {}", self.number == other_number)
        }
    }

    let item = Item { number: 10 };

    let reference_item = &item;
    let other_reference_item = &reference_item;

    item.compare_number(10);
    reference_item.compare_number(10);
    other_reference_item.compare_number(10);
}
fn easy_rust_18() {
    use std::fmt::Display;

    fn print_and_give_item() -> i32 {
        let number = 9;
        println!("The number is :{}", number);
        9
    }

    let x = print_and_give_item();

    fn give_thing<T: Display>(input: T) -> T {
        println!("{}", input);
        input
    }

    let y = give_thing(String::from("Take this thing"));
    println!("{}", y);
}
fn easy_rust_19() {
    //프로그래밍 언어 러스트를 배웁시다! 046 Easy Rust in Korean: Option

    fn take_fifth(value: Vec<i32>) -> Option<i32> {
        if value.len() < 5 {
            None
        } else {
            Some(value[4])
        }
    }

    let new_vec = vec![1, 2, 4, 7, 8, 10, 10];
    let index = take_fifth(new_vec);

    println!("{:?}", index);
}
fn esay_rust_20() {
    //프로그래밍 언어 러스트를 배웁시다! 047 Easy Rust in Korean: More Option
    fn take_fifth(value: Vec<i32>) -> Option<i32> {
        if value.len() < 5 {
            None
        } else {
            Some(value[4])
        }
    }

    let new_vec = vec![1, 2, 4];
    let index = take_fifth(new_vec);

    match index {
        Some(number) => println!("I got a number: {}", number),
        None => println!("There was nothing inside"),
    }

    if index.is_some() {
        println!("I got a number: {}", index.unwrap());
    }

    index.expect("Needed at least five items - make sure Vec has at last five");
}
fn easy_rust_21() {
    //프로그래밍 언어 러스트를 배웁시다! 048 Easy Rust in Korean: Result
    fn check_error(input: i32) -> Result<(), ()> {
        if input % 2 == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    if check_error(5).is_ok() {
        println!("It's okay, guys!")
    } else {
        println!("It's an error, guys!")
    }

    match check_error(5) {
        Ok(_) => println!("Okay guys"),
        Err(_) => println!("It's an error"),
    }
}
fn easy_rust_22() {
    //프로그래밍 언어 러스트를 배웁시다! 049 Easy Rust in Korean: Result
    fn check_if_five(number: i32) -> Result<i32, String> {
        match number {
            5 => Ok(number),
            _ => Err("Sorry, the number wasn't five.".to_string()),
        }
    }

    let mut result_vec = vec![];

    fn parse_number(number: &str) -> Result<i32, std::num::ParseIntError> {
        number.parse()
    }

    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("thohunthoe"));
    result_vec.push(parse_number("8"));

    for number in result_vec {
        println!("{:?}", number);
    }
}
fn easy_rust_23() {
    //프로그래밍 언어 러스트를 배웁시다! 050 Easy Rust in Korean: Result
    let my_vec = vec![2, 3, 4];

    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
        // match my_vec.get(index){
        //     Some(number) => println!("The number is: {}", number),
        //     None => {}
        // }
    }

    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the chity of {}:", city[0]);

        while let Some(infomation) = city.pop() {
            if let Ok(number) = infomation.parse::<i32>() {
                println!("The number is: {}", number);
            }
        }
    }
}
fn easy_rust_24() {
    //프로그래밍 언어 러스트를 배웁시다! 051 Easy Rust in Korean: Result
    //HashMap, BTreeMap
    use std::collections::BTreeMap;

    struct City {
        name: String,
        population: BTreeMap<u32, u32>,
    }

    let mut tallin = City {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallin.population.insert(1372, 3_250);
    tallin.population.insert(1851, 24_000);
    tallin.population.insert(2020, 437_619);

    for (year, population) in tallin.population {
        println!("In the year {} the population was {}", year, population);
    }
}
fn easy_rust_25() {
    //프로그래밍 언어 러스트를 배웁시다! 052 Easy Rust in Korean: Result
    use std::collections::HashMap;

    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsrube", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap.get("Bielefeld"));
}
fn easy_rust_26() {
    //프로그래밍 언어 러스트를 배웁시다! 053 Easy Rust in Korean: Result
    use std::collections::HashMap;

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "새도우 오브 유어 스마일",
        "Eye of the World",
        "Eye of the World",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books += 1;
    }

    for (book, number) in book_hashmap {
        println!("{}: {} copies", book, number);
    }
}
fn easy_rust_27() {
    //프로그래밍 언어 러스트를 배웁시다! 054 Easy Rust in Korean: Result
    use std::collections::HashMap;
    use std::collections::HashSet;

    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut servey_hash = HashMap::new();

    for item in data {
        servey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (male_or_female, numbers) in servey_hash {
        println!("{:?}, {:?}", male_or_female, numbers);
    }

    let many_numbers = vec![
        94, 42, 56, 64, 32, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80,
    ];

    let mut number_hashset = HashSet::new();

    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();

    println!(
        "There are {} unique numbers, so we are missing {}.",
        hashset_length,
        100 - hashset_length
    );

    let mut missing_vec = vec![];

    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for number in missing_vec {
        print!("{} ", number);
    }
}
fn easy_rust_28() {
    //프로그래밍 언어 러스트를 배웁시다! 055 Easy Rust in Korean: Result
    use std::collections::BinaryHeap;

    fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
        let mut remainder_vec = vec![];

        for number in input {
            remainder_vec.push(*number);
        }
        remainder_vec
    }

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() {
        println!(
            "Popped off {}. Remaining numbers are: {:?}",
            number,
            show_remainder(&my_heap)
        );
    }
}
fn easy_rust_29() {
    //프로그래밍 언어 러스트를 배웁시다! 056 Easy Rust in Korean: Result
    use std::collections::VecDeque;

    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    for i in 0..6000000 {
        my_vec.pop_front();
    }
}
fn easy_rust_30() {
    //프로그래밍 언어 러스트를 배웁시다! 057 Easy Rust in Korean: Result
    use std::num::ParseIntError;

    fn parse_str(input: &str) -> Result<i32, ParseIntError> {
        let parsed_number = input.parse::<i32>()?;
        Ok(parsed_number)
    }

    for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}
#[derive(Debug)]
struct Book {
    title: String,
    year: u16,
}

fn easy_rust_31() {
    //프로그래밍 언어 러스트를 배웁시다! 058 Easy Rust in Korean: Result

    let my_book = Book {
        title: "Some title".to_string(),
        year: 1919,
    };

    let book_2 = Book {
        title: "Book 2".to_string(),
        year: 2020,
    };

    let width = 10;
    println!("My book name: {my_book:*^width$?}");
}

use std::fmt::Debug;
#[derive(Debug)]
struct MYStruct {
    number: usize,
}

fn easy_rust_32() {
    //프로그래밍 언어 러스트를 배웁시다! 059 Easy Rust in Korean: Result
    fn print_as_debug<T>(input: T)
    where
        T: Debug,
    {
        println!("{input:?}");
    }
}

struct Animal {
    name: String,
}

trait Canine {
    // dog-like
    fn bark(&self) {
        println!("I am bark");
    }
    fn run(&self) {
        println!("I am run");
    }
}

impl Canine for Animal {
    fn bark(&self) {
        println!("멍멍!");
    }
}
fn easy_rust_33() {
    //프로그래밍 언어 러스트를 배웁시다! 060 Easy Rust in Korean: Result
    let my_animal = Animal {
        name: "Mr. Mantle".to_string(),
    };

    my_animal.bark();
    my_animal.run();
}
use std::fmt;
#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let age = self.age;
        write!(f, "{name} {age}")
    }
}
fn easy_rust_34() {
    //프로그래밍 언어 러스트를 배웁시다! 061 Easy Rust in Korean: Result
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("Mr. Mantle is a {mr_mantle:?}");
}
struct Monster {
    health: i32,
}
struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You strike with your sword! Your opponent's health is now {}",
            opponent.health
        );
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You strike with your first! Your opponent's health is now {}",
            opponent.health
        );
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow! Your opponent's health is now {}",
                opponent.health
            );
        }
    }
    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!(
                "You attack with your rock! Your opponent's health is now {}",
                opponent.health
            );
        }
    }
}
impl FightFromDistance for Ranger {}
fn easy_rust_35() {
    //프로그래밍 언어 러스트를 배웁시다! 062 Easy Rust in Korean: Result
    let radagast = Wizard {};
    let aragorn = Ranger {};

    let mut uruk_hai = Monster { health: 40 };

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 7);
}

fn main() {
    //     let mut addressInfoSys = AddressInfoSys{ InfomationDatas: Vec::new() };

    //     addressInfoSys.addTuple(("현준","가시덤불골짜기"));
    //     addressInfoSys.addTuple(("동훈","오그리마"));
    //     addressInfoSys.addTuple(("재민","불모의땅"));
    //     addressInfoSys.addTuple(("용곤","스톰윈드"));

    //    addressInfoSys.showAllData();
    //easy_rust_35();
    //easy_rust_67();
    //easy_rust_68();
    //easy_rust_69();
    //easy_rust_70();
    //easy_rust_71();
    easy_rust_72();
    easy_rust_79();
    easy_rust_80();
    easy_rust_81();
    easy_rust_82();
    easy_rust_83();
    easy_rust_84();
    easy_rust_85();
    easy_rust_86();
    easy_rust_87();
    easy_rust_88();
    easy_rust_89();
    easy_rust_90();
    easy_rust_91();
    easy_rust_92();
    easy_rust_93();
    easy_rust_94();
    easy_rust_95();
    easy_rust_96();
    easy_rust_97();
    easy_rust_98();
    easy_rust_99();
    easy_rust_100();
}
