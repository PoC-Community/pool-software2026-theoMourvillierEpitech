pub fn get_even_numbers(numbers: &[i32]) -> String {
    let mut even_numbers: Vec<i32> = Vec::new(); {
        let mut index = 0;
        loop {
            if index == numbers.len(){
                break;
            }
            if numbers[index] % 2 == 0 {
                even_numbers.push(numbers[index]);
            }
            index +=1;
        }
    }


    even_numbers.sort();

    let mut result = String::new();
    let mut index = 0;

    loop {
        if index == even_numbers.len() {
            break;
        }
        result.push_str(&even_numbers[index].to_string());
             if index < even_numbers.len() - 1 {
                result.push_str(" - ");
        }
        index += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_even_numbers_positive() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(get_even_numbers(&numbers), "2 - 4 - 6 - 8 - 10");
    }
    #[test]
    fn test_get_even_numbers_negative() {
        let numbers = vec![-1, -2, -4, -7, -25];
        assert_eq!(get_even_numbers(&numbers), "-4 - -2");
    }
    #[test]
    fn test_get_even_numbers_mixed() {
        let numbers = vec![-10, -3, 12, 0, -8, 14, 25];
        assert_eq!(get_even_numbers(&numbers), "-10 - -8 - 0 - 12 - 14");
    }
}