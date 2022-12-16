use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\nUsage: cleanpi filename");
        //dbg!(&args);

        return Ok(());
    }

    let fname = &args[1];
    println!("Start cleaning {fname}...");
    clean_file(fname)?;

    Ok(())
}

#[allow(unreachable_code)]
fn clean_file(fname: &str) -> std::io::Result<()> {
    let file = File::open(fname)?;
    let outfname = format!("{fname}.out");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    //dbg!("Output goes to {}", outfname);

    let slcontents = if contents.chars().nth(0) == Some('3') && contents.chars().nth(1) == Some('.')
    {
        &contents[2..]
    } else {
        &contents[..]
    };

    let mut allok = true;
    for c in slcontents.chars() {
        if c < '0' || c > '9' {
            println!("Wrong char: [{c}]");
            allok = false;
        }
    }

    if allok {
        let mut outfile = match File::create(&outfname) {
            Err(why) => panic!("couldn't create {}: {}", outfname, why),
            Ok(file) => file,
        };
        outfile.write_all(slcontents.as_bytes());
    }

    Ok(())
}
