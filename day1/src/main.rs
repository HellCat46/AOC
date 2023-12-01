fn main() {
    let data : String = std::fs::read_to_string("data/document.txt").unwrap();
    let mut tot = 0;
    let lines = data.split("\n");
    for line in lines {
        let mut f : u32 = 10;
        let mut l : u32 = 10 ;
        for ch in line.chars(){
            if f == 10 {
                match ch.to_digit(10) {
                    Some(value) => f = value,
                    None => continue,
                };
            }
            match ch.to_digit(10) {
                Some(value) => l = value,
                None => continue,
            };
        }
        if f == 10 { continue};
        tot += (f*10)+l;
    }
    println!("{}", tot);
}
