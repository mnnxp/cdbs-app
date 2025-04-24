use regex::Regex;

/// Returns arrays of received data, containing pair of header and value.
pub(crate) fn parsing_single(data: &str) -> Vec<(String, String)> {
    let mut lines = data.lines();
    let mut result = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            continue;
        }
        if let Some((header, value)) = parsing_hp(line) {
            result.push((header.to_string(), value.to_string()));
        }
    }
    result
}

pub(crate) fn parsing_hp(text: &str) -> Option<(&str, &str)> {
    let re = r"([^\t\n]*)";
    let mut header = "";
    let mut value = "";
    for m in Regex::new(re).unwrap().find_iter(text) {
        if header.is_empty() {
            header = m.as_str();
            continue;
        }
        value = m.as_str();
        break;
    }
    if header.is_empty() || value.is_empty() {
        return None;
    }
    Some((header, value))
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn parsing_tab() {
        let input_test = r"Param 1	test value 1
Param 2	22,cq,1 ,,,,,,,

Param 3	Vm4 “#$24” mv 25
Param 4	test value 4
Bad 555
Param 5	@31, ,sdv,vdsv
	Bad
Param 6	csdk\tm\nv wewf 30
Param 7	lal so r32 m";
        let output_test = vec![
            (String::from("Param 1"), String::from("test value 1")),
            (String::from("Param 2"), String::from("22,cq,1 ,,,,,,,")),
            (String::from("Param 3"), String::from("Vm4 “#$24” mv 25")),
            (String::from("Param 4"), String::from("test value 4")),
            (String::from("Param 5"), String::from("@31, ,sdv,vdsv")),
            (String::from("Param 6"), String::from(r"csdk\tm\nv wewf 30")),
            (String::from("Param 7"), String::from("lal so r32 m"))
            ];
        // let output_test = (vec![Some("")], vec![vec![Some("")]]);
        let result = parsing_single(input_test);
        assert_eq!(output_test, result)
    }
}
