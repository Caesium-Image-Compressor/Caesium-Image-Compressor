fn quality_for_album(photos: usize) -> u8 {
    if photos > 200 {
        75
    } else if photos > 40 {
        80
    } else {
        85
    }
}

fn main() {
    let q = quality_for_album(120);
    println!("suggested quality {q}");
}
