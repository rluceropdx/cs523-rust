use prompted::input;

fn main() {
    let input_first_row = input!("Provide the first row: ");

    let mut bits_str: String = "*.*..*..".to_string(); // or "10100100".to_string();

    if input_first_row.len() == 8 {
        bits_str = input_first_row;
    } else {
        println!("First row input not valid, will use {} instead", bits_str);
        println!();
    }

    println!("{}", bits_str); // first row

    let mut tf_list: [bool; 8] = bits_to_tf(&bits_str);
    for _i in 1..10 {
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

    let result: [bool; 8] = tf_bits.clone().try_into().expect("Vector has wrong length");
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
