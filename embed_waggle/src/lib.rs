#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn say(x: String) {
    println!("{}", x);
}

#[no_mangle]
pub extern "C" fn get_string(x: String) -> String {
    return x;
}

#[no_mangle]
pub extern "C" fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

#[no_mangle]
pub extern "C" fn sub(x: i32, y: i32) -> i32 {
    return x - y;
}

#[no_mangle]
pub extern "C" fn div(x: f32, y: f32) -> f32 {
    return x / y;
}

#[no_mangle]
pub extern "C" fn fact(x: i32) -> i32 {
    if x < 0 {
        return 0;
    } else if x == 0 {
        return 1;
    } else {
        return x * fact(x - 1);
    }
}

#[no_mangle]
pub extern "C" fn fact2(x: i32) -> i32 {
    if x <= 0 {
        return 0;
    } else {
        let upper_limit = x + 1;
        let mut product = 1;
        for i in 1..upper_limit {
            product = product * i;
        }
        return product;
    }
}

#[no_mangle]
pub extern "C" fn transform_url(url: String) -> String {
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
