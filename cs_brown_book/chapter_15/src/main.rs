#![allow(unused)]
fn main() {
    use std::borrow::Cow;

    fn modify_weirdly(input: &mut Cow<'_, [i32]>) {
        for i in 0..input.len() {
            let v = input.get(i);
            if let Some(_) = v {
                // Clones into a vector if not already owned.
                input.to_mut().pop();
            }
        }
    }

    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    modify_weirdly(&mut input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [4, -3, -1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    modify_weirdly(&mut input);
    println!("{:?} {:?}", slice, input);

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    modify_weirdly(&mut input);
}
