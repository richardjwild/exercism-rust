pub fn is_leap_year(year: i32) -> bool {
    return year.divisible_by(400) ||
        (year.divisible_by(4) && year.not_divisible_by(100))
}

trait Divisible {
    fn divisible_by(&self, divisor: i32) -> bool;
    fn not_divisible_by(&self, divisor: i32) -> bool;
}

impl Divisible for i32 {

    fn divisible_by(&self, divisor: i32) -> bool {
        self % divisor == 0
    }

    fn not_divisible_by(&self, divisor: i32) -> bool {
        self % divisor != 0
    }
}