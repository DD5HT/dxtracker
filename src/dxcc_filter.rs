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
    let re = prefix_to_regex("DA-DR").unwrap();
    re.is_match(call);
    Some("Germany")
}

fn sample_csv() {
    let germany_sample = "DL:Germany:Eu:28:14:+1:53N:13E:DA-DR,Y2-Y9:";
    let germany_newformat = "DL;Germany;DA-DR;Y2-Y9;";
    let germany_prefix = "DA-DR";
}

///Takes an exact formated prefix range with the following pattern:
///"DA-DR" and converts it to a regex: "^(D[A-R])"
///```
/// use dxtracker::dxcc_filter::*;
/// assert_eq!(prefix_to_regex("DA-DR").unwrap().as_str(), "^(D[A-R])");
///```
pub fn prefix_to_regex(prefix: &str) -> Option<Regex> {
    if prefix.len() == 5 {
        let prefix_regex = format!(
            r"^({}[{}-{}])",
            prefix.get(0..1).unwrap(),
            prefix.get(1..2).unwrap(),
            prefix.get(4..5).unwrap()
        );
        match Regex::new(prefix_regex.as_ref()) {
            Ok(fix) => Some(fix),
            Err(e) => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_regex_test() {
        assert_eq!(prefix_to_regex("DA-DR").unwrap().as_str(), "^(D[A-R])");
        assert_eq!(prefix_to_regex("AA-AL").unwrap().as_str(), "^(A[A-L])");
        assert_eq!(prefix_to_regex("8A-8I").unwrap().as_str(), "^(8[A-I])");
        assert!(prefix_to_regex("MALLFROMATED").is_none());
    }

    #[test]
    fn match_call_test() {
        assert_eq!(match_call("DD5HT"), Some("Germany"));
        assert_eq!(match_call("DL0IU"), Some("Germany"));
        assert_eq!(match_call("DD5HT"), Some("Germany"));
        assert_eq!(match_call("DR5DT"), Some("Germany"));
        //assert!(!match_call("DD1"));
    }
}
