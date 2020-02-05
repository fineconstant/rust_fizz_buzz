use fizz_buzz::FizzBuzz;

//Integration tests

#[test]
fn when_1_then_1() {
    let actual = FizzBuzz::for_number(1);
    let expected = "1";
    assert_eq!(actual, expected)
}

#[test]
fn when_2_then_2() {
    let actual = FizzBuzz::for_number(2);
    let expected = "2";
    assert_eq!(actual, expected)
}

#[test]
fn when_3_then_fizz() {
    let actual = FizzBuzz::for_number(3);
    let expected = "Fizz";
    assert_eq!(actual, expected)
}

#[test]
fn when_4_then_4() {
    let actual = FizzBuzz::for_number(4);
    let expected = "4";
    assert_eq!(actual, expected)
}

#[test]
fn when_5_then_5() {
    let actual = FizzBuzz::for_number(5);
    let expected = "Buzz";
    assert_eq!(actual, expected)
}

#[test]
fn when_10_then_5() {
    let actual = FizzBuzz::for_number(10);
    let expected = "Buzz";
    assert_eq!(actual, expected)
}

#[test]
fn when_15_then_fizz_buzz() {
    let actual = FizzBuzz::for_number(15);
    let expected = "FizzBuzz";
    assert_eq!(actual, expected)
}
