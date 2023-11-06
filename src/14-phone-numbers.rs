fn main() {
    let phone_number_1 = "+1 (223) 456-7890";
    let phone_number_2 = "223 456   7890   ";
    let phone_number_3 = "223456789";
    let phone_number_4 = "22234567890";
    let mut p = PhoneNumber::new(phone_number_1);
    assert_eq!(p.clean_and_validate(), Some("2234567890".to_string()));
    p.set_input(phone_number_2);
    assert_eq!(p.clean_and_validate(), Some("2234567890".to_string()));
    p.set_input(phone_number_3);
    assert_eq!(p.clean_and_validate(), None);
    p.set_input(phone_number_4);
    assert_eq!(p.clean_and_validate(), None);
}

struct PhoneNumber {
    input: String,
}

impl PhoneNumber {
    fn new(input: &str) -> Self {
        PhoneNumber {
            input: String::from(input),
        }
    }

    fn set_input(&mut self, new_input: &str) {
        self.input = String::from(new_input);
    }

    fn clean_and_validate(&mut self) -> Option<String> {
        let mut phone_number: String = self
            .input
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>();
        let mut char_vec: Vec<char> = phone_number.chars().collect();

        if phone_number.len() <= 9 || phone_number.len() >= 12 {
            return None;
        } else {
            if char_vec[0] == '1' && phone_number.len() == 11 {
                phone_number = phone_number[1..].to_string();
                char_vec = phone_number.chars().collect();
            } else if phone_number.len() == 10 {
            } else {
                return None;
            }
            if char_vec[0] != '1' && char_vec[3] != '0' {
                return Some(phone_number);
            }
        }
        None
    }
}
