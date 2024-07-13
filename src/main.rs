mod dummy;
use dummy::file::{File, Read};

fn main() -> Result<(), String> {
    let mut read_buffer = Vec::<u8>::new();
    let file_name = "testfile.txt";
    let file_data = vec![114, 117, 115, 116, 33];
    let mut file = File::new_with_data(&file_name, &file_data);

    file.open()?;
    let file_len = file.read(&mut read_buffer)?;
    file.close()?;
    let text = String::from_utf8_lossy(&read_buffer);
    println!("{:?}", file);
    println!("{} is {} bytes long", file.name(), file_len);
    println!("{}", text);
    Ok(())
}
