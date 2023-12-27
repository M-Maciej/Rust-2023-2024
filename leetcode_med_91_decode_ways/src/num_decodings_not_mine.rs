//author: rony0000013
//https://leetcode.com/problems/decode-ways/solutions/4454444/rust-0ms-2-05mb-one-liner-solution/
//https://leetcode.com/rony0000013/

pub fn num_decodings_not_mine(s: String) -> i32 {
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
