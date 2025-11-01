pub fn is_armstrong_number(num: u32) -> bool {
    //todo!("true if {num} is an armstrong number");
    let power = num.to_string().len() as u32;
    let arm_num = num.to_string().chars()
    .map(|c| {let digit = c.to_digit(10).unwrap() as u64;
        digit.pow(power)})
    .sum::<u64>();
    arm_num == (num as u64)
}
