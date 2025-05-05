pub struct P12;

impl P12 {
    pub fn other_v1_solve(num: i32) -> String {
        let mut mutable_number = num;
        let mut return_value = String::new();
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let romans = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        for i in 0..values.len() {
            while mutable_number >= values[i] {
                dbg!(&mutable_number, &values[i]);
                return_value.push_str(romans[i]);
                mutable_number -= values[i];
            }
        }
        return_value
    }
}
