// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut result = String::new();
    let mut start = None;
    let mut last = None;

    let n = input.chars().count();

    for (i, c) in input.chars().enumerate() {
        if c != ' ' {
            start = Some(i);
            break;
        }
    }

    for (i, c) in input.chars().rev().enumerate() {
        if c != ' ' {
            last = Some(n - i);
            break;
        }
    }

    match (start, last) {
        (Some(start), Some(last)) => result.push_str(&input[start..last]),
        (Some(start), _) => result.push_str(&input[start..]),
        (_, Some(last)) => result.push_str(&input[..last]),
        _ => result.push_str(input),
    }

    result
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut result = String::from(input);
    result.push_str(" world!");

    result
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut result = String::new();

    let indeces = spot(input);

    match indeces {
        (Some(first), Some(last)) => {
            result.push_str(&input[..first]);
            result.push_str("balloons");
            result.push_str(&input[last + 1..]);
        }
        _ => (),
    }

    result
}

fn spot(input: &str) -> (Option<usize>, Option<usize>) {
    let mut first = None;
    let mut last = None;

    for (i, c) in input.chars().enumerate() {
        if c == 'c' {
            match first {
                Some(_) => (),
                None => {
                    first = Some(i);
                }
            }
        }

        if c == 's' {
            match first {
                Some(_) => last = Some(i),
                None => {}
            }
        }
    }

    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
