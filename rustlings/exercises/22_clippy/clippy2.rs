fn main() {
    let mut res = 42;
    let option = Some(12);
    // Fixed the Clippy lint.
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}