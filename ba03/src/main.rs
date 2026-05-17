fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        return;
    }

    let sorted = merge_sort(&args, 0, args.len() - 1);

    for item in sorted {
        println!("{}", item);
    }
}

fn merge_sort<T: AsRef<str>>(input: &[T], start: usize, end: usize) -> Vec<String> {
    if start >= end {
        return vec![input[start].as_ref().to_string()];
    }
    let mid = (start + end) / 2;
    let left_arr = merge_sort(input, start, mid);
    let right_arr = merge_sort(input, mid + 1, end);

    let res = merge(left_arr, right_arr);

    return res;
}

fn merge(left_arr: Vec<String>, right_arr: Vec<String>) -> Vec<String> {
    let mut iter_left = left_arr.into_iter().peekable();
    let mut iter_right = right_arr.into_iter().peekable();

    let mut res = Vec::new();

    while iter_left.peek().is_some() && iter_right.peek().is_some() {
        if iter_left.peek() < iter_right.peek() {
            res.push(iter_left.next().unwrap());
        } else {
            res.push(iter_right.next().unwrap());
        }
    }

    while let Some(item) = iter_left.next() {
        res.push(item);
    }

    while let Some(item) = iter_right.next() {
        res.push(item);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted_input_stays_sorted() {
        let input: Vec<&str> = "a b c".split(" ").collect();

        let sorted = merge_sort(&input, 0, input.len() - 1);

        assert_eq!(sorted, vec!["a", "b", "c"]);
    }

    #[test]
    fn reversed_input_gets_sorted() {
        let input: Vec<&str> = "e d c b a".split(" ").collect();

        let sorted = merge_sort(&input, 0, input.len() - 1);

        assert_eq!(sorted, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn uppercase_comes_before_lowercase() {
        let input: Vec<&str> = "A a A a A a".split(" ").collect();

        let sorted = merge_sort(&input, 0, input.len() - 1);

        assert_eq!(sorted, vec!["A", "A", "A", "a", "a", "a"]);
    }

    #[test]
    fn words_with_punctuation_sorted_alphabetically() {
        let input: Vec<&str> = "hello, world, this is a programm".split(" ").collect();

        let sorted = merge_sort(&input, 0, input.len() - 1);

        assert_eq!(
            sorted,
            vec!["a", "hello,", "is", "programm", "this", "world,"]
        );
    }
}
