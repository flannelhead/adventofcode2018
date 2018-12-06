use std::fs::File;
use std::io::prelude::*;

fn reduce(data: &String) -> String {
    return data.chars().fold(String::new(), |mut res, new_char| {
        match res.chars().last() {
            Some(chr) => if chr != new_char && chr.eq_ignore_ascii_case(&new_char) {
                res.pop();
                return res
            },
            None => ()
        }
        res.push(new_char);
        res
    });
}

fn main() -> std::io::Result<()> {
    let mut datafile = File::open("data/5_1")?;
    let mut data = String::new();
    datafile.read_to_string(&mut data)?;
    data = data.trim().to_string();

    let range = 'a' as u8 .. 'z' as u8;
    let lengths = range.map(|c| {
        let chr = c as char;
        let fixed_data = data.replace(|chr1: char| chr1.eq_ignore_ascii_case(&chr), "");
        let reduced = reduce(&fixed_data);
        reduced.len()
    });

    let reduced = reduce(&data);
    println!("{}", reduced.len());
    println!("{:?}", lengths.min());

    Ok(())
}
