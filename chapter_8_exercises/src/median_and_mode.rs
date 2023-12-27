use std::collections::HashMap;

pub fn median(list: &[isize]) -> Option<isize> {
    let mut my_vector = list.to_vec();
    my_vector.sort();
    let length = my_vector.len();
    let median = my_vector.get(length / 2);
    return median.copied();
}
pub fn mode(list: &[isize]) -> Option<isize> {
    let mut occurences = HashMap::new();
    for integer in list.iter() {
        let count = occurences.entry(integer).or_insert(0);
        *count += 1;
    }
    occurences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .copied()
}
pub fn mode2(list: &[isize]) -> Option<isize> {
    let mut occurrences = HashMap::new();
    for &number in list {
        *occurrences.entry(number).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .fold((None, 0), |(max_num, max_count), (num, count)| {
            match count > max_count {
                true => (Some(num), count),
                false => (max_num, max_count),
            }
        })
        .0
}
