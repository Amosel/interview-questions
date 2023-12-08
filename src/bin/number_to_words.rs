// s,tens,hundreds, thousands, millions, billions

fn get_below_thousand(num: i32) -> String {
    if num == 0 {
        return "Zero".to_string();
    }
    let below_20 = [
        "",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eightteen",
        "Nineteen",
    ];
    let tens = [
        "", "", "Twenty", "Thirty", "Fourty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninty",
    ];

    if num < 20 {
        return below_20[num as usize].to_string();
    } else if num < 100 {
        let teens_s = if num % 10 == 0 {
            "".to_string()
        } else {
            format!(" {}", below_20[num as usize % 10])
        };

        return format!("{}{}", tens[num as usize / 10], teens_s);
    } else if num < 1000 {
        let below_thousands = num - ((num / 100) * 100);
        let below_thousands = if below_thousands == 0 {
            "".to_string()
        } else {
            format!(" {}", get_below_thousand(below_thousands))
        };
        return format!(
            "{} Hundred{}",
            below_20[num as usize / 100],
            below_thousands
        );
    } else {
        panic!()
    }
}

pub fn number_to_words(num: i32) -> String {
    if num < 1000 {
        get_below_thousand(num)
    } else {
        let mut result = String::new();
        let billion = num / 1_000_000_000;
        let million = (num / 1_000_000) % 1_000;
        let thousand = (num / 1_000) % 1_000;
        let remainder = num % 1_000;

        if billion > 0 {
            result.push_str(&format!("{} Billion ", get_below_thousand(billion)));
        }
        if million > 0 {
            result.push_str(&format!("{} Million ", get_below_thousand(million)));
        }
        if thousand > 0 {
            result.push_str(&format!("{} Thousand ", get_below_thousand(thousand)));
        }
        if remainder > 0 {
            result.push_str(&get_below_thousand(remainder));
        }

        result.trim().to_string()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(number_to_words(0), "Zero".to_string());
        assert_eq!(number_to_words(9), "Nine".to_string());
        assert_eq!(number_to_words(90), "Ninty".to_string());
        assert_eq!(number_to_words(91), "Ninty One".to_string());
        assert_eq!(number_to_words(991), "Nine Hundred Ninty One".to_string());
        assert_eq!(number_to_words(100), "One Hundred".to_string());
        assert_eq!(number_to_words(101), "One Hundred One".to_string());
        assert_eq!(number_to_words(1001), "One Thousand One".to_string());
        assert_eq!(number_to_words(1991), "One Thousand Nine Hundred Ninty One".to_string());
        assert_eq!(number_to_words(1_356_124), "One Million Three Hundred Fifty Six Thousand One Hundred Twenty Four".to_string());
    }
}
