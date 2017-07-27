fn main() {
    let url = "//waggle-images.s3.amazonaws.com/drive_files/attachments/000/000/343/original/5d3d3d9f1a4f079d6f8bee52ac9b4c73.gcode?1500885304";
    let url2 = format!("{}", url);
    println!("{}", transform_url(url2));
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
