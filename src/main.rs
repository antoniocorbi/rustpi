use std::fs::File;
use std::io::prelude::*;

fn compute_stats(v : &Vec<u32>) -> [u32;10]{
    let mut st = [0u32; 10];

    for x in v.iter() {
        st[*x as usize] += 1;
    }

    st
}

fn print_stats(v: [u32;10], count: usize) {
    for i in 0..v.len() {
        println!("[{}]: {}%", i, (100*v[i]) as f64 / count as f64)
    }
}

fn main() -> std::io::Result<()> {
    let fname = "/home/acorbi/projects/rustpi/data/pi-10million.txt";
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // strip last char?
    if fname == "/home/acorbi/projects/rustpi/data/pi-1million.txt" {
        contents = contents[0..contents.len()-1].to_string();
    }
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

    let st = compute_stats(&numbers);
    print_stats(st, numbers.len());
    println!("-----------------------------------------\n{:?}", st);

    Ok(())
}
