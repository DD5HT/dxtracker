//! TODO:
//! auto create regex for each entity
//! add filters for special calls
//! add sanity checks for callsigns length, < 2 < 20
//!
#![allow(warnings)]
extern crate regex;

use regex::Regex;

struct Entity {
    prefix: &'static str,
    name: &'static str,
    prefixRange: Vec<&'static str>,
}

fn match_call(call: &str) -> bool {
    let re = Regex::new(r"^(D[A-R])").unwrap();
    re.is_match(call)
}

fn sample_csv() {
    let germany_sample = "DL:Germany:Eu:28:14:+1:53N:13E:DA-DR,Y2-Y9:";
    let germany_newformat = "DL;Germany;DA-DR;Y2-Y9;";
    let germany_prefix = "DA-DR";
}

fn prefix_to_regex(prefix: &str) -> &str {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_regex_test() {
        assert_eq!(prefix_to_regex("DA-DR"), "^(D[A-R])");
        assert_eq!(prefix_to_regex("AA-AL"), "^(A[A-L])");
        assert_eq!(prefix_to_regex("8A-8I"), "^(8[A-I])");
    }

    #[test]
    fn match_call_test() {
        assert!(match_call("DD5HT"));
        assert!(match_call("DL0IU"));
        assert!(match_call("DK0CD"));
        assert!(match_call("DM5EE"));
        assert!(match_call("DO3BLABA"));
        assert!(!match_call("KK1DD"));
        assert!(!match_call("D1DLA"));
        //assert!(!match_call("DD1"));
    }
}
