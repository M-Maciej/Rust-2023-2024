mod encoding_ver_1;
fn main() {
    let mut x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    println!("{}", x);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);

    let mut z = 5;
    let z_ref = &mut *x;
    println!("{z} {z_ref}");
    (*z_ref) += 1;
    println!("{x}");
    let z_ref = &mut *x;
    println!("{z} {z_ref}");
    *z_ref = z;
    z += 1;
    println!("{z} {z_ref}");
    println!("{x}");
    println!("przerwa");

    println!("{}", encoding_ver_1::num_decodings("2101"));
    println!("{}", num_decodings("2101".to_string()));

    let array = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(item) = iterator.next() {
        println!("Item: {}", item);
    }
}

fn num_decodings(s: String) -> i32 {
    s.chars()
        .enumerate()
        .fold([1, 0], |a, (i, c)| {
            [
                if c != '0' { a[0] + a[1] } else { 0 },
                if i > 0
                    && (s.chars().nth(i - 1) == Some('1')
                        || (s.chars().nth(i - 1) == Some('2') && c < '7'))
                {
                    a[0]
                } else {
                    0
                },
            ]
        })
        .iter()
        .sum()
}
