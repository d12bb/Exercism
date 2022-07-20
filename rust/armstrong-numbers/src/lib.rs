struct Number {
    num: u32,
    len: u32,
    digits: Vec<u32>,
}
pub fn is_armstrong_number(num: u32) -> bool {
    let num = split(num);
    if num.num == num.digits.iter().map(|d| d.pow(num.len) ).sum() {
        return true
    }
    return false
}

fn split(num: u32) -> Number {
    let mut num = num;
    let orig = num;
    let mut digits: Vec<u32> = vec![];

    while num > 0 {
        digits.push(num % 10);
        num = num / 10;
    }

    Number{
        num: orig,
        len: digits.len().try_into().unwrap(),
        digits,
    }
}
