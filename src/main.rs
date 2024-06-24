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

fn highest_weight_value (n: u32, b: u8) -> u32 {
    let base: u32 = b.into();

	let mut weight_power = 0;

	while base.pow(weight_power) < n {
		weight_power += 1;
	}

	return weight_power - 1;
}

fn b_to_dec (n: String, b: u8) -> u32 {
    let base: u32 = b.into();

    let mut result = 0;
    let mut counter = 0;

    // Splitting n
    let mut parts: Vec<_> = n.split("").collect::<Vec<_>>();
    parts.retain(|&x| x != "");

    for i in (0..parts.len()).rev() {
        let element = parts[i];

        result += base.pow(counter) * element.parse::<u32>().unwrap();

        counter += 1;
    }

    return result.try_into().unwrap();
}

fn dec_to_b (n: u32, b: u8) -> String {
	let base: u32 = b.into();
	let hwv = highest_weight_value(n, b);

	let mut n_remaining = n.clone();
	let mut result = String::new();

	for i in (0..hwv+1).rev() {
		let val = n_remaining / (base.pow(i));
		if val > 0 {
			n_remaining = n_remaining % (base.pow(i) * val);
		}
		result = format!("{}{}", result, val);
	}

	return result;
}

fn main() {
    stdout().flush().unwrap();
    
    let (mut in_base, mut out_base): (u8, u8);
    let in_value: String;

    loop {
    	simple_print("Source base [2, 16]:\t");
    	in_base = read_u8();

    	if in_base >= 2 && in_base <= 16 {
    		break;
    	}
    }

    simple_print("Source value [1, ∞[:\t");
    in_value = read_string();

    // Using only bases beetwen 
    loop {
    	simple_print("Ouput base [2, 16]:\t");
    	out_base = read_u8();

    	if in_base >= 2 && in_base <= 16 {
    		break;
    	}
    }

    let number_dec = b_to_dec(in_value.clone(), in_base);
    let out_value = dec_to_b(number_dec, out_base);

    println!("{:?} in base({:?}) is {:?} in base({:?})", in_value, in_base, out_base, out_value);
    println!("\t>{:?}", out_value);
}
