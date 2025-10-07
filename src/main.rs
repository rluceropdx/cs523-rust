fn main() {
    let mut input_first_row = std::env::args().nth(1).unwrap_or("*.*..*..".to_string()); // Default first row
    let input_total_rows: String = std::env::args().nth(2).unwrap_or("10".to_string()); // total generations to build

    let total_rows: u8 = match input_total_rows.parse() {
        Ok(number) => number,
        Err(error) => {
            println!("oops, not a number: {}", error);
            10
        }
    };

    if input_first_row.len() != 8 {
        input_first_row = "*.*..*..".to_string();
        println!(
            "First row input not valid, will use {} instead",
            input_first_row
        );
    }

    println!("Will calculate {} generations/rows", total_rows);
    println!();
    println!("{}", input_first_row); // first row

    let mut tf_list: [bool; 8] = bits_to_tf(&input_first_row);
    for _i in 1..total_rows {
        tf_list = calc_next_generation(tf_list);
        println!();
    }
}

fn calc_next_generation(tf_list: [bool; 8]) -> [bool; 8] {
    let result: [bool; 8] = [
        rule110([tf_list[7], tf_list[0], tf_list[1]]),
        rule110([tf_list[0], tf_list[1], tf_list[2]]),
        rule110([tf_list[1], tf_list[2], tf_list[3]]),
        rule110([tf_list[2], tf_list[3], tf_list[4]]),
        rule110([tf_list[3], tf_list[4], tf_list[5]]),
        rule110([tf_list[4], tf_list[5], tf_list[6]]),
        rule110([tf_list[5], tf_list[6], tf_list[7]]),
        rule110([tf_list[6], tf_list[7], tf_list[0]]),
    ];
    result
}

fn bits_to_tf(bits_str: &str) -> [bool; 8] {
    let mut tf_bits: Vec<bool> = Vec::new();
    for bit_char in bits_str.chars() {
        if bit_char == '*' || bit_char == '1' {
            tf_bits.push(true);
        } else if bit_char == '.' || bit_char == '0' {
            tf_bits.push(false);
        }
    }

    let result: [bool; 8] = tf_bits.clone().try_into().expect(
        "String cannot be converted to bits of true/false. Please check contents or length.",
    );
    result
}

fn rule110(bits: [bool; 3]) -> bool {
    let result = match bits {
        [true, true, true] => false,
        [true, true, false] => true,
        [true, false, true] => true,
        [true, false, false] => false,
        [false, true, true] => true,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
    };
    if result {
        print!("*")
    } else {
        print!(".")
    };
    result
}

// tests below
#[cfg(test)]
mod tests {
    use super::*; // import outer scope functions
    use std::panic;

    #[test]
    fn test_rule110_tft() {
        assert_eq!(rule110([true, false, true]), true);
        assert_eq!(rule110([true, false, false]), false);
        assert_eq!(rule110([false, false, false]), false);
    }

    #[test]
    fn test_bits_to_tf() {
        let test_result1 = panic::catch_unwind(|| {
            bits_to_tf("12345678");
        });
        assert!(
            test_result1.is_err(),
            "String cannot be converted to bits of true/false. Please check contents or length."
        );

        let test_result2 = panic::catch_unwind(|| {
            bits_to_tf("*.*.*.");
        });
        assert!(
            test_result2.is_err(),
            "String cannot be converted to bits of true/false. Please check contents or length."
        );

        assert_eq!(
            bits_to_tf("10101010"),
            [true, false, true, false, true, false, true, false]
        );
        assert_eq!(
            bits_to_tf("*.*.*.*."),
            [true, false, true, false, true, false, true, false]
        );
    }

    #[test]
    fn test_calc_next_generation() {
        assert_eq!(
            calc_next_generation([true, false, true, false, false, true, false, false]),
            [true, true, true, false, true, true, false, true]
        );
        assert_eq!(
            calc_next_generation([true, true, true, false, true, true, false, true]),
            [false, false, true, true, true, true, true, true]
        );
    }
}
