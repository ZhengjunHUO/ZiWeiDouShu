use crate::interactive::birthday_from_prompt;
use crate::structs::Birthday;

pub fn build_palais() {
    let b = Birthday::from_gregorian(birthday_from_prompt());
    println!("{}", b);
}
