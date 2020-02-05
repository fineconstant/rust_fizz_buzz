use fizz_buzz::FizzBuzz;

//Binary - use case
fn main() {
    let fizz_buzz = (1..=1_000).map(|x| (x, FizzBuzz::for_number(x)));

    for (x, fb) in fizz_buzz {
        println!("FizzBuzz {}: {}", x, fb);
    }
}
