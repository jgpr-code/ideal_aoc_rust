use std::fmt::Display;

// the return type for parts sometime its Numbers sometimes its Strings
#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    Num(i128),
    #[allow(dead_code)]
    Str(String),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Answer::Num(i) => write!(f, "{}", i),
            Answer::Str(s) => write!(f, "{}", s),
        }
    }
}

#[macro_export]
macro_rules! regx {
    ($re:literal) => {{
        static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            // println!("initializing regex {}", $re);
            regex::Regex::new($re).unwrap()
        });
        &RE
    }};
}
pub use regx;

#[cfg(test)]
#[macro_use]
pub mod test_utils {
    use std::fs;
    pub fn read_from_file(filename: &str) -> String {
        println!("reading {}", filename);
        fs::read_to_string(filename)
            .unwrap_or_else(|msg| panic!("error reading {}: {}", filename, msg))
    }
    #[macro_export]
    macro_rules! extract_day_from_path {
        () => {{
            let path = module_path!();
            let re = $crate::regx!(r"day\d{2}");
            let m = re
                .find(path)
                .expect("macro is only valid inside paths containing 'day\\d{2}' pattern");
            m.as_str()
        }};
    }
    #[macro_export]
    macro_rules! local_file {
        ($file:literal) => {
            LazyLock::new(|| {
                read_from_file(&format!(
                    "src/{}/{}",
                    $crate::extract_day_from_path!(),
                    $file
                ))
            })
        };
    }
    // re-export macro such that test_utils::local_file path can be used.
    pub use crate::local_file;
}
