pub mod ch_10_tests;

fn main() {
    println!("Hello, world!");
    let x = ch_10_tests::Point::new(10.5, 10.3);
    let distance = x.distance_from_origin();
    println!("{distance}");

    let mut string = String::from("something");
    let string2: &String;
    {
        let c = "asdasdaasdasds";
        string2 = ch_10_tests::make_larger_or_equal(&mut string, c);
    }
    println!("{string2}");
    println!("{string}");

    println!("{string}");
    string.push_str("sssss");
    println!("{string}");
}
