fn main() {
    const GIANT_INT: i64 = 100_000_000;
    let url = "//waggle-images.s3.amazonaws.com/drive_files/attachments/000/000/343/original/5d3d3d9f1a4f079d6f8bee52ac9b4c73.gcode?1500885304";
    let url2 = format!("{}", url);
    println!("{}", transform_url(url2));

    let file_name_a = "foobar.stl";
    let file_name_b = "fdmprinter.def.json";
    println!("{}", get_file_name(file_name_a.to_string()));
    println!("{}", get_extension(file_name_a.to_string()));
    println!("{}", get_file_name(file_name_b.to_string()));
    println!("{}", get_extension(file_name_b.to_string()));
    println!("{}", addition(1, 2));
    println!("The remainder of {} / {} is {}", 3, 2, (3%2));
    println!("3 to the 3rd power is {}", i32::pow(3, 3));
    // example of integer power of float
    println!("2.5 to the 2nd power is {}", f64::powi(2.5, 2));
    // example of floating point power of float
    println!("2.5 to the power of pi is {}", f64::powf(2.5, std::f64::consts::PI));
    println!("The value of GIANT_INT is {}", GIANT_INT);

    // variable shadowing
    let z = 10;
    println!("The value of z is {}", z);
    let z = z + 20;
    println!("The new value of z is {}", z);
    // in Rust, we are not allowed to mutate a variable type, use shadowing for that
}

fn addition(a: i32, b: i32) -> i32 {
    return a + b;
}

fn transform_url(url: String) -> String {
    let owned_string = format!("http:{}", url);
    let mut q_index = 0;
    for i in owned_string.chars() {
        if i == '?' {
            break;
        } else {
            q_index = q_index + 1
        }
    }
    let concat_string = format!("{}", &owned_string[..q_index]);
    return concat_string;
}

// &str is an immutable reference to a string
fn get_extension(name: String) -> String {
    let mut dot_index = 0;
    for i in name.chars().rev() {
        if i == '.' {
            break;
        } else {
            dot_index = dot_index + 1;
        }
    }
    let dot_index = name.chars().count() - dot_index;
    return name[dot_index..].to_string();
}

// &str is an immutable reference to a string
fn get_file_name(name: String) -> String {
    let mut dot_index = 0;
    for i in name.chars().rev() {
        if i == '.' {
            break;
        } else {
            dot_index = dot_index + 1;
        }
    }
    let dot_index = name.chars().count() - dot_index - 1;
    return name[..dot_index].to_string();
}
