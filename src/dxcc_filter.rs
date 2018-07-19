//! TODO:
//! auto create regex for each entity
//! add filters for special calls
//! add sanity checks for callsigns length, < 2 < 20
//!
#![allow(warnings)]
extern crate regex;

use regex::Regex;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Entity {
    prefix: &'static str,
    name: &'static str,
    prefixRange: Vec<&'static str>,
}
//FIXME: return entitiy struct
///Takes a callsign and returns a Option<Entity>
pub fn match_call(call: &str) -> Option<&str> {
    let re = Regex::new(r"^(D[A-R])").unwrap();
    re.is_match(call);
    unimplemented!()
}

fn sample_csv() {
    let germany_sample = "DL:Germany:Eu:28:14:+1:53N:13E:DA-DR,Y2-Y9:";
    let germany_newformat = "DL;Germany;DA-DR;Y2-Y9;";
    let germany_prefix = "DA-DR";
}
//FIXME:
fn prefix_to_regex(prefix: &str) -> Option<&str> {
    if prefix.len() == 5 {
        let splitted: Vec<&str> = prefix.split('-').collect();
        println!("{:?}",splitted );
        Some(prefix)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_regex_test() {
        assert_eq!(prefix_to_regex("DA-DR"), Some("^(D[A-R])"));
        assert_eq!(prefix_to_regex("AA-AL"), Some("^(A[A-L])"));
        assert_eq!(prefix_to_regex("8A-8I"), Some("^(8[A-I])"));
    }

    #[test]
    fn match_call_test() {
        assert_eq!(match_call("DD5HT"), Some("Germany"));
        assert_eq!(match_call("DL0IU"), Some("Germany"));
        assert_eq!(match_call("DD5HT"), Some("Germany"));
        assert_eq!(match_call("DD5HT"), Some("Germany"));
        //assert!(!match_call("DD1"));
    }
}
