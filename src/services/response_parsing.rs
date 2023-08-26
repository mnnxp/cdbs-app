use log::debug;
use serde::Deserialize;
use serde_json::{Value, from_str, from_value};
use crate::error::{Error, get_error};

/// Get objects from response data by key
pub(crate) fn resp_parsing<T>(
    response: String,
    key_word: &str,
) -> Result<Vec<T>, Error>
where
    for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
{
    debug!("Key word: {:?}", key_word);
    get_value_response(response).and_then(|val| {
        let res = from_value(val.get(key_word).unwrap().clone());
        Ok(res.unwrap())
    })
}

/// Get object from response data by key
pub(crate) fn resp_parsing_item<T>(
    response: String,
    key_word: &str,
) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
{
    debug!("Key word: {:?}", key_word);
    get_value_response(response).and_then(|val| {
        let res = from_value(val.get(key_word).unwrap().clone());
        Ok(res.unwrap())
    })
}

/// The top object is extracted by the first key,
/// then next objects are extracted from it by the second key.
pub(crate) fn resp_parsing_two_level<T>(
    response: String,
    first_key: &str,
    second_key: &str,
) -> Result<Vec<T>, Error>
where
    for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
{
    debug!("Key words (1,2): {:?}, {:?}", first_key, second_key);
    get_value_response(response).and_then(|val| {
        let res = from_value(val
            .get(first_key).unwrap().clone()
            .get(second_key).unwrap().clone()
        );
        Ok(res.unwrap())
    })
}

/// Get object from JSON value by key
pub(crate) fn get_from_value<T>(
    value: &Value,
    key_word: &str,
) -> Result<Vec<T>, Error>
where
    for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
{
    debug!("Key word: {:?}", key_word);
    let res = from_value(value.get(key_word).unwrap().clone());
    Ok(res.unwrap())
}

/// Retrieves an object with the key "data" from the response data and returns a JSON value.
pub(crate) fn get_value_response(response: String) -> Result<Value, Error> {
    let data: Value = from_str(response.as_str()).unwrap();
    let res = data.as_object().unwrap().get("data").unwrap().clone();

    match res.is_null() {
        false => Ok(res),
        true => Err(get_error(&data)),
    }
}
