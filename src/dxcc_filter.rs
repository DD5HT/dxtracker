//! TODO:
//! auto create regex for each entity
//! add filters for special calls
//! add sanity checks for callsigns length, < 2 < 20
//!
#![allow(warnings)]

use csv::Reader;
use regex::Regex;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Entity {
    prefix: String,
    name: String,
    prange: Vec<String>,
}

//FIXME: iterate over Entity vector
///Takes a callsign and returns a Option<Entity>
pub fn match_call(call: &str) -> Option<Entity> {
    let dxcc = Entity {
        prefix: "DL".to_string(),
        name: "Germany".to_string(),
        prange: vec!["DA-DR".to_string(), "Y2-Y9".to_string()],
    };

    let re = prefix_to_regex(dxcc.prange[0].as_ref()).unwrap();
    if re.is_match(call) {
        Some(dxcc)
    } else {
        None
    }
}

//FIXME: read in CSV to struct.
fn sample_csv() {
    let germany = "\
    prefix,name,prange
    DL,Germany,DA-DR,Y2-Y9
    ";
    let mut rdr = Reader::from_reader(germany.as_bytes());
    let mut iter = rdr.deserialize();

    if let Some(result) = iter.next() {
        let record: Entity = result.unwrap();
        println!("{:?}", record);
    }
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
            prefix.get(0..1).unwrap(), //unwrap is safe here
            prefix.get(1..2).unwrap(), //since we check the length
            prefix.get(4..5).unwrap(), //before slicing the string
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
        let germany = Some(Entity {
            prefix: "DL".to_string(),
            name: "Germany".to_string(),
            prange: vec!["DA-DR".to_string(), "Y2-Y9".to_string()],
        });
        assert_eq!(match_call("DD5HT"), germany);
        assert_eq!(match_call("DL0IU"), germany);
        assert_eq!(match_call("DD5HT"), germany);
        assert_eq!(match_call("DR5DT"), germany);
        assert_eq!(match_call("CAKE"), None);
        //assert!(!match_call("DD1"));
    }
}
