use std::io::{stdin, stdout, Write};

fn read_string () -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("cannot find user input");

    input = input
        .trim()
        .to_string();

    return input;
}

fn read_u8 () -> u8 {
    let input_str = read_string();
    let output_u8 = input_str.parse::<u8>().unwrap();
    return output_u8;
}

fn simple_print (text: &str) {
    print!("{}", text);
    stdout().flush().unwrap();
}

fn b_to_dec (n: String, b: u8) -> u32 {
    let base: u32 = b.into();

    let mut result = 0;
    let mut counter = 0;

    let mut parts: Vec<_> = n.split("").collect::<Vec<_>>();
    parts.retain(|&x| x != "");

    for i in (0..parts.len()).rev() {
        let element = parts[i];

        println!("{:?}", element.parse::<u32>().unwrap());
        result += base.pow(counter) * element.parse::<u32>().unwrap();

        counter += 1;
    }

    return result.try_into().unwrap();
}

/*fn dec_to_b (n: String, b: u8) {

}*/

fn main() {
    stdout().flush().unwrap();

    simple_print("Source base [2, 255]:\t");
    let in_base = read_u8();

    simple_print("Source value :\t\t");
    let in_value = read_string();

    simple_print("Ouput base [2, 255]:\t");
    let _out_base = read_u8();

    let number_dec = b_to_dec(in_value, in_base);
    println!("{:?}", number_dec);
}
