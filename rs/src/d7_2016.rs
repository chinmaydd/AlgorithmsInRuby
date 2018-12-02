use util;

fn check_if_supports_tls(input: &str) -> bool {
    // Let us assume the string to be of the form a.b.c.d.a.b.c.d...
    let mut iter = input.chars();

    let mut a: char = iter.next().unwrap();
    let mut b: char = iter.next().unwrap();
    let mut c: char = iter.next().unwrap();
    let mut d: char = iter.next().unwrap();

    for character in iter {
        if a == d && b == c && a != b {
            return true
        } else {
            a = b;
            b = c;
            c = d;
            d = character;
        }
    }
    
    return a == d && b == c && a != b
}

fn process_ip(input: &str) -> bool {
    // Let us assume the string to be of the form a.b.c.d.a.b.c.d...

    let v: Vec<&str> = input.split(|c| c == ']' || c == '[').collect();
    let mut in_bracket = false;
    let mut solved = false;

    for string in v {
        if !in_bracket && check_if_supports_tls(string) {
            solved = true;
        }

        if in_bracket && check_if_supports_tls(string) {
            return false;
        }

        in_bracket = !in_bracket
    }

    solved
}

fn check_if_supports_ssl(_substr: &str, _total_vec: Vec<&str>) -> bool {
    false
}

fn process_ip_for_ssl(input: &str) -> bool {
    // Let us assume that the string is of the form a.b.c.a.b.c...

    let _v: Vec<&str> = input.split(|c| c == ']' || c == '[').collect();

    false
}

pub fn run() {
    let input = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp7");
    
    let split = input.split("\n");
    let vec = split.collect::<Vec<_>>();

    let mut count = 0;

    for ip in vec {
        if process_ip_for_ssl(ip.clone()) {
            count += 1;
        }
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
   use super::process_ip;

   #[test]
   fn test_1() {
       let test_1_str = "abba[mnop]qrst";
       assert_eq!(process_ip(test_1_str), true);
   }

   #[test]
   fn test_2() {
       let test_2_str = "abcd[bddb]xyyx";
       assert_eq!(process_ip(test_2_str), false);
   }

   #[test]
   fn test_3() {
       let test_3_str = "aaaa[qwer]tyui";
       assert_eq!(process_ip(test_3_str), false);
   }

   #[test]
   fn test_4() {
       let test_4_str = "ioxxoj[asdfgh]zxcvbn";
       assert_eq!(process_ip(test_4_str), true);
   }
}
