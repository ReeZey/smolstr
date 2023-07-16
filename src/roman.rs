//stolen from some random dude on the internet

pub struct Roman {
    value: u32,
}

const NUMERAL_MAP: [(u32, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl From<u32> for Roman {
    fn from(value: u32) -> Roman {
        Roman { value: value }
    }
}

impl ToString for Roman {
    /// Supports overlining for large numerals

    fn to_string(&self) -> String {
        let mut value = self.value;

        let mut builder = String::new();

        // Check for thousands of M-V

        for &(number, numeral) in NUMERAL_MAP[..11].iter() {
            let number = number * 1000;

            let quotient = value / number;

            builder.push_str(&repeat(&overline(numeral), quotient as usize));

            value -= number * quotient;
        }

        for &(number, numeral) in NUMERAL_MAP.iter() {
            let quotient = value / number;

            builder.push_str(&repeat(numeral, quotient as usize));

            value -= number * quotient;
        }

        builder
    }
}

fn repeat(s: &str, amount: usize) -> String {
    std::iter::repeat(s).take(amount).collect()
}

fn overline(s: &str) -> String {
    // Combining overline: \u{0305}

    s.chars().map(|c| format!("{}\u{0305}", c)).collect()
}
