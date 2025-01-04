use regex::Regex;

// #[derive(Clone, Debug)]
// pub(crate) enum ParsingSplit {
//     Tab,
//     Csv,
//     Custom(String)
// }

/// Returns arrays of received data, the first containing headers and the second containing values.
/// If first is true, only the first row is processed, and the array with values is returned empty.
pub(crate) fn parsing_text(data: &str, first: bool) -> (Vec<Option<&str>>, Vec<Vec<Option<&str>>>){
    let mut lines = data.lines();
    let headers = lines.by_ref().next().map(|head| parsing_line(head)).unwrap_or_default();
    let mut values = Vec::new();
    if headers.is_empty() || first {
        return (headers, values)
    }
    for line in lines.by_ref() {
        if line.is_empty() {
            continue;
        }
        values.push(parsing_line(line));
    }
    (headers, values)
}

pub(crate) fn parsing_line(text: &str) -> Vec<Option<&str>> {
    let re = r"([^\t\n]*)";
    // let re = match split {
    //     ParsingSplit::Tab => r"([^\t\n]*)",
    //     // ("(?:[^"]|"")*"|[^,"\n\r]*)(,|\r?\n|\r)
    //     ParsingSplit::Csv => r#"(?:^|,)(?=[^"]|(")?)"?((?(1)(?:[^"]|"")*|[^,"]*))"?(?=,|$)"#,
    // };
    let mut values = Vec::new();
    for m in Regex::new(re).unwrap().find_iter(text) {
        let value = m.as_str();
        if value.is_empty() {
            values.push(None);
            continue;
        }
        values.push(Some(value));
    }
    values
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn parsing_tab() {
        let input_test = r"[ModificationName]	name2	name3	name4	Name5<span>2</span>	name6
par1	21	31	41	51	61
par2		23	24	2    5	26
par3	Vm4 “#$24” mv 25	@31, ,sdv,vdsv	2   5	26	lal so r32 m
par4	24	25	26	27	28
par5	22,cq,1	,,,,,,,		28	29
par6	26		28d  	csdk\tm\nv wewf	30
par7		28		30	31";
        let output_test = (
            vec![Some("[ModificationName]"), Some("name2"), Some("name3"), Some("name4"), Some("Name5<span>2</span>"), Some("name6")],
            vec![
                vec![Some("par1"), Some("21"), Some("31"), Some("41"), Some("51"), Some("61")],
                vec![Some("par2"), None, Some("23"), Some("24"), Some("2    5"), Some("26")],
                vec![Some("par3"), Some("Vm4 “#$24” mv 25"), Some("@31, ,sdv,vdsv"), Some("2   5"), Some("26"), Some("lal so r32 m")],
                vec![Some("par4"), Some("24"), Some("25"), Some("26"), Some("27"), Some("28")],
                vec![Some("par5"), Some("22,cq,1"), Some(",,,,,,,"), None, Some("28"), Some("29")],
                vec![Some("par6"), Some("26"), None, Some("28d  "), Some(r"csdk\tm\nv wewf"), Some("30")],
                vec![Some("par7"), None, Some("28"), None, Some("30"), Some("31")]
            ]
        );
        // let output_test = (vec![Some("")], vec![vec![Some("")]]);
        let result = parsing_text(input_test, false);
        assert_eq!(output_test, result)
    }

//     #[test]
//     fn parsing_csv() {
//         let input_test = r#"[ModificationName],name2,name3,name4,Name5<span>2</span>,name6
// par1,21,31,41,51,61
// par2,,23,24,2    5,26
// par3,Vm4 “#$24” mv 25,"@31, ,sdv,vdsv",2   5,26,lal so r32 m
// par4,24,25,26,27,28
// par5,"22,cq,1",",,,,,,,",,28,29
// par6,26,,28d  ,csdk\tm\nv wewf,30
// par7,,28,,30,31"#;
//         let output_test = (vec![Some("")], vec![vec![Some("")]]);
//         let result = parsing_text(input_test, ParsingSplit::Csv);
//         assert_eq!(output_test, result)
//     }
}
