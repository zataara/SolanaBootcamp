// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)


fn main() {
    let answer: String = current_favorite_course();
    println!("My course is {}", answer);
}

fn current_favorite_course() -> String {
    String::from("Solana")
}
