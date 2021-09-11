fn main() {
    triangle();
}

fn triangle() {
    for i in 0..5 {
        for _ in 0..5 - i {
            print!("#");
        }
        println!();
    }
}
