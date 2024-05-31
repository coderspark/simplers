use std::*;

pub fn input<S: Into<Option<&'static str>>>(s: S) -> String {
    let s_ref = s.into();

    if let Some(s_str) = s_ref {
        println!("{}", s_str);
    }

    let mut val = String::new();
    std::io::stdin().read_line(&mut val).expect("Something went wrong with the input.");

    return val[0..val.len() - 1].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{}", input(""));
    }
}
