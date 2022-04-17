use std::cmp::min;

fn diff_common_prefix(text1: &String, text2: &String) -> usize {
    // Qucik check for common null cases.
    if text1.is_empty() || text2.is_empty() || text1.chars().nth(0) != text2.chars().nth(0) {
        return 0;
    }

    // Binary search
    let mut pointer_min = 0;
    let mut pointer_max = min(text1.len(), text2.len());
    let mut pointer_mid = pointer_max;
    let mut pointer_start = 0;

    while pointer_min < pointer_mid {
        if &text1[pointer_start..pointer_mid] == &text2[pointer_start..pointer_mid] {
            pointer_min = pointer_mid;
            pointer_start = pointer_mid;
        } else {
            pointer_max = pointer_mid;
        }
        pointer_mid =
            ((pointer_max - pointer_min) as f64 / 2.0 + pointer_min as f64).floor() as usize;
    }
    return pointer_mid;
}

fn compare(text1: &String, text2: &String) -> bool {
    text1.eq(text2)
}

fn main() {
    let text1 = String::from("Apples are a fruit.");
    let text2 = String::from("Bananas are also fruit.");
    let diff = compare(&text1, &text2);

    println!("Text 1: {}", &text1);
    println!("Text 2: {}", &text2);
    println!("Diff  : {}", &diff);
}

#[cfg(test)]
mod tests {
    use crate::{compare, diff_common_prefix};

    #[test]
    fn equality_check() {
        let text1 = String::from("Apples are a fruit.");
        let text2 = String::from("Bananas are also fruit.");
        let text3 = String::from("林檎は果物です。🍎🍏");
        let text4 = String::from("バナナも果物です。🍌");
        assert_eq!(compare(&text1, &text1), true);
        assert_eq!(compare(&text1, &text2), false);
        assert_eq!(compare(&text3, &text3), true);
        assert_eq!(compare(&text3, &text4), false);
    }

    #[test]
    fn prefix_check() {
        let text1 = String::from("The cat in the hat.");
        let text2 = String::from("The furry cat in the hat.");
        let text3 = String::from("Is that the furry cat?");
        let empty = String::from("");
        assert_eq!(diff_common_prefix(&empty, &text2), 0);
        assert_eq!(diff_common_prefix(&text1, &empty), 0);
        assert_eq!(diff_common_prefix(&empty, &empty), 0);
        assert_eq!(diff_common_prefix(&text1, &text3), 0);
        assert_eq!(diff_common_prefix(&text1, &text1), text1.len());
        assert_eq!(diff_common_prefix(&text1, &text2), 4);

        // let text4 = String::from("今日は晴れです。");
        // let text5 = String::from("今日は雨です。");
        // assert_eq!(diff_common_prefix(&text4, &text5), 4);
    }
}
