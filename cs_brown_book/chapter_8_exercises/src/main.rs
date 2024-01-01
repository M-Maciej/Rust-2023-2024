use chapter_8_exercises::department_text_interface;
use chapter_8_exercises::median_and_mode;
use chapter_8_exercises::pig_latin;
fn main() {
    let list_of_integers = [
        1, 2, 2, 1, 3, 4, 3, 5, 3, 5, 215, 15, 14, 13, 1, 3, 21, 32, 2, 3, 34, 2, 23, 4, 3, 5, 3,
        53, 3, 4, 3, 4, 3, 2, 3, 34, 2, 4, 3, 2, 34, 34, 34, 34, 34, 34, 234, 5234, 5345, 3535, 3,
        4, 3, 2, 3, 3, 4, 3, 3, 4, 3, 4, 34, 4, 3, 34, 5, 4, 5, 45, 4, 3, 23, 3, 4, 3, 3, 4, 3, 34,
        5, 45, 5, 4, 54, 5, 45, 4, 5, 4, 54, 5, 5, 45, 5, 45, 5, 4, 5, 4, 5, 5, 6, 56, 56, 5, 6,
        56, 5, 65, 6, 56, 6, 56, 56, 5, 65, 6, 56, 56, 56, 56, 56, 56, 5, 56, 5, 65, 65, 65, 65,
        65, 6, 56, 56, 56, 5, 65, 65, 6, 56, 56, 56, 56, 5, 65, 6,
    ];
    let median = median_and_mode::median(&list_of_integers);
    let mode1 = median_and_mode::mode(&list_of_integers);
    let mode2 = median_and_mode::mode2(&list_of_integers);
    assert_eq!(mode1, mode2);
    if let Some(median_value) = median {
        println!("Median: {}", median_value);
    }
    if let Some(mode_value) = mode1 {
        println!("Mode: {}", mode_value);
    }

    let some_text="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let some_latin = pig_latin::to_latin_text(&some_text);
    println!("{some_latin}");

    department_text_interface::run_interface();
}
