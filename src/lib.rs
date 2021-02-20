use pyo3::prelude::*;
use pyo3::{wrap_pyfunction};
use pyo3::types::{PyString, PyDict, PyFunction, PyList, IntoPyDict};
use std::collections::HashMap;
use regex::Regex;

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

#[pyfunction]
fn is_html_input(dictionary: &PyAny) -> PyResult<bool> {
    //MultiDict type datastructures are used to represent HTML form input,
    // which may have more than one value for each key.
    dictionary.hasattr("getlist")
}

#[pyfunction]
fn parse_html_dict<'a>(dictionary: &'a PyDict, prefix: & PyString) -> PyResult<HashMap<&'a str, Vec<&'a str>>>{
    // Used to support dictionary values in HTML forms.
    //
    // {
    //     'profile.username': 'example',
    //     'profile.email': 'example@example.com',
    // }
    //     -->
    // {
    //     'profile': {
    //         'username': 'example',
    //         'email': 'example@example.com'
    //     }
    // }
    let regex_string = format!(r#"^{}\.(.+)$"#,prefix);
    let reg: Regex = Regex::new(regex_string.as_str()).unwrap();
    let mut ret: HashMap<&str, Vec<&str>> = HashMap::new();
    for (k,v) in dictionary {
        let kk: &str = k.extract()?;
        let mat = reg.captures(kk);
        if mat.is_none() {
            continue
        } else {
            let key = mat.unwrap().get(1).unwrap();
            let value:Vec<&str> = dictionary.get_item(kk).unwrap().extract()?;
            ret.insert(key.as_str(),value);
        }
    }
    Ok(ret)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rest_framework(py: Python, m: &PyModule) -> PyResult<()> {
    // let datastructures = PyModule::import(py, "django.utils.datastructures")?;
    // let multi_value_dict = datastructures.get("MultiValueDict")?;
    // println!("{}",multi_value_dict);
    m.add_function(wrap_pyfunction!(is_html_input, m)?)?;
    m.add_function(wrap_pyfunction!(parse_html_dict, m)?)?;


    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
