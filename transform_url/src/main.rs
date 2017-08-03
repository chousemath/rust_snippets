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

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess is {}", guess);

    // example of a double precision floating point number in Rust
    let x1 = 2.0;
    println!("The size of x1 is {} bytes", std::mem::size_of_val(&x1));

    // example of a single precision floating point number in Rust
    let x2: f32 = 125.2;
    println!("The size of x2 is {} bytes", std::mem::size_of_val(&x2));

    // Rust has two primitive compound types, tuples and arrays
    // example of a tuple
    let yy: (&str, u32) = ("hadouken", 9000);
    println!("The size of yy is {} bytes", std::mem::size_of_val(&yy));
    // use pattern matching to destructure a tuple
    let (yy1, yy2) = yy;
    println!("The first value of yy is {}", yy1);
    println!("The first value of yy is {}", yy.0);
    println!("The second value of yy is {}", yy2);
    println!("The second value of yy is {}", yy.1);

    // arrays in Rust have fixed length
    // arrays are useful when you want stack allocation rather than heap allocation
    // an array is a single chunk of memory allocated on the stack
    let arr_a = [1, 2, 3, 4, 5];
    let mut counter = 0;
    println!("The length of arr_a is {}", arr_a.len());
    for i in &arr_a {
        println!("element {} of arr_a => {}", counter, i);
        counter += 1;
    }

    println!("First value of arr_a = {}", arr_a[0]);

    // Rust panic is when the program exits with an error

    say_hello();
    let (wisdom_a, wisdom_b) = spit_wisdom(55);
    println!("{}", wisdom_a);
    println!("{}", wisdom_b);

    let wall_thickness = 2.0;
    let wall_line_width_0 = 0.2;
    let wall_line_width_x = 0.2;
    let line_count = wall_line_count(wall_thickness, wall_line_width_0, wall_line_width_x);
    println!("The wall line count when the 0th wall is {}, xth wall is {}, and desired wall thickness is {}, is {}", wall_line_width_0, wall_line_width_x, wall_thickness, line_count);
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

// Rust doesn't care where you define your functions
fn say_hello() {
    println!("Hello!");
}

fn spit_wisdom(x: u32) -> (String, String) {
    let value_of_x = format!("The value of x is {}", x);
    let size_of_x  = format!("The size of x is {} bytes", std::mem::size_of_val(&x));
    return (value_of_x, size_of_x);
}

fn wall_line_count(wall_thickness: f64, wall_line_width_0: f64, wall_line_width_x: f64) -> i64 {
    // magic_spiralize else max(1, round((wall_thickness - wall_line_width_0) / wall_line_width_x) + 1) if wall_thickness != 0 else 0"
    if wall_thickness == 0.0 { return 0; }
    return std::cmp::max(1, (((wall_thickness - wall_line_width_0) / wall_line_width_x).round() as i64) + 1);
}
