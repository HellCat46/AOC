fn main() {
    let data = std::fs::read_to_string("data/document.txt").unwrap();
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut tot = 0;
    let lines = data.split("\n");
    for line in lines {
        let mut f: u32 = 10;
        let mut l: u32 = 10;

        struct Digit {
            idx: usize,
            num: u32,
        }
        let mut arr: Vec<Digit> = Vec::new();
        for (num, digit) in NUMS.iter().enumerate() {
            match line.find(digit) {
                Some(idx) => {
                    arr.push(Digit {
                        idx,
                        num: num as u32 + 1,
                    });
                    match line.rfind(digit) {
                        Some(ridx) => {
                            if ridx != idx {
                                arr.push(Digit {
                                    idx: ridx,
                                    num: num as u32 + 1,
                                })
                            }
                        }
                        None => todo!(),
                    }
                }
                None => continue,
            }
        }
        arr.sort_by(|a, b| a.idx.partial_cmp(&b.idx).unwrap());

        let mut first_idx: Option<usize> = None;
        let mut last_idx: Option<usize> = None;
        for (idx, ch) in line.chars().enumerate() {
            if f == 10 {
                match ch.to_digit(10) {
                    Some(value) => {
                        f = value;
                        first_idx = Some(idx);
                    }
                    None => continue,
                };
            }
            match ch.to_digit(10) {
                Some(value) => {
                    l = value;
                    last_idx = Some(idx)
                }
                None => continue,
            };
        }

        if arr.len() > 0 && (first_idx == None || first_idx > Some(arr[0].idx)) {
            f = arr[0].num;
        }
        if f == 10 {
            continue;
        };
        if arr.len() > 0 && (last_idx == None || last_idx < Some(arr[arr.len() - 1].idx)) {
            l = arr[arr.len() - 1].num;
        }

        tot += (f * 10) + l;
    }
    println!("\n{}", tot);
}
