use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("/home/acorbi/projects/rustpi/data/pi-1million.txt")?;
    //    let mut file = File::open("/home/acorbi/projects/rustpi/data/small.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents[0..contents.len()-1].to_string();
    //assert_eq!(contents, "Hello, world!");

    println!("Contents length: {}", contents.len());

    let mut count = 0;
    let numbers : Vec<u32> = contents
        .chars()
        .map(|c|{
            count += 1;
            //print!("[{}]", c);
            c.to_digit(10).unwrap()
            //    c
        })
        .collect();

    //println!("count= {}", count);
    //println!("'7'*2 es = {}", '7'.to_digit(10).unwrap() * 2);

    println!("{:?}", numbers);

    Ok(())
}
