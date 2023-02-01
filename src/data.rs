use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub enum Data {
    Int(i64),
    Str(String),
    Array(Vec<Data>),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Int({})", i),
            Self::Str(s) => write!(f, "Str({})", s),
            Self::Array(v) => write!(f, "Arr({:?})", v),
        }
    }
}

impl Data {
    pub fn new(input: String) -> Option<Data> {
        println!("input: {}", input);
        if Data::is_int(input.as_str()) {
            Some(Data::Int(Data::try_parse_int(input.as_str())))
        } else if Data::is_str(input.as_str()) {
            Some(Data::Str(input))
        } else {
            Some(Data::Array(
                input
                    .replace(&['[', ']'][..], "")
                    .split(",")
                    .fold(vec![], |mut acc, v| {
                        acc.push(Data::new(v.to_string()).unwrap());
                        acc
                    }),
            ))
        }
    }

    fn is_int(input: &str) -> bool {
        let re = Regex::new(r"^\d+$").unwrap();
        re.is_match(input)
    }

    fn is_str(input: &str) -> bool {
        let re = Regex::new(r"^(\w+)|(\d+)$").unwrap();
        re.is_match(input)
    }

    fn try_parse_int(input: &str) -> i64 {
        input.parse::<i64>().unwrap()
    }
}
