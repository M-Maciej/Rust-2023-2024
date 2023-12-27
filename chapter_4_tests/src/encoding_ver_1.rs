pub fn num_decodings(s: &str) -> i32 {
    let st = s.as_bytes();

    return match st.len() {
        0 => 1,
        1 => {
            if st[0] == b'0' {
                0
            } else {
                1
            }
        }
        _ => {
            let mut first_joined = 0;
            let mut first_seperate = 1;
            for i in (0..(st.len() - 1)).rev() {
                //println!("iterator {i}");
                let byte = st[i];
                let b_right = st[i + 1];
                match byte {
                    b'0' => {
                        if i == 0 {
                            return 0;
                        }
                        match b_right {
                            b'0' => {
                                return 0;
                            }
                            _ => {
                                first_seperate = first_seperate + first_joined;
                                first_joined = 0;
                            }
                        }
                    }
                    b'1' => match b_right {
                        b'0' => continue,
                        b'1' | b'2' => {
                            if i + 2 != st.len() {
                                if st[i + 2] == b'0' {
                                    first_seperate = first_seperate + first_joined;
                                    first_joined = 0;
                                    continue;
                                }
                            }
                            let joined_temp = first_joined;
                            first_joined = first_seperate;
                            first_seperate = joined_temp + first_seperate;
                        }
                        _ => {
                            let joined_temp = first_joined;
                            first_joined = first_seperate + joined_temp;
                            first_seperate = first_seperate + joined_temp;
                        }
                    },
                    b'2' => match b_right {
                        b'0' => {
                            continue;
                        }
                        b'1' | b'2' => {
                            if i + 2 != st.len() {
                                if st[i + 2] == b'0' {
                                    first_seperate = first_seperate + first_joined;
                                    first_joined = 0;
                                    continue;
                                }
                            }
                            let joined_temp = first_joined;
                            first_joined = first_seperate;
                            first_seperate = joined_temp + first_seperate;
                        }
                        b'7' | b'8' | b'9' => {
                            let joined_temp = first_joined;
                            first_joined = 0;
                            first_seperate = first_seperate + joined_temp;
                        }
                        _ => {
                            let joined_temp = first_joined;
                            first_joined = first_seperate + joined_temp;
                            first_seperate = first_seperate + joined_temp;
                        }
                    },
                    _ => match b_right {
                        b'0' => {
                            return 0;
                        }
                        _ => {
                            first_seperate = first_seperate + first_joined;
                            first_joined = 0;
                        }
                    },
                }
            }
            return first_joined + first_seperate;
        }
    };
}
