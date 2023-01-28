use log::debug;
use serde::Deserialize;
use serde_json::{Value, from_str, from_value};
use crate::error::{Error, get_error};

/// Возвращает извлечённые по ключу объекты
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

/// Возвращает извлечённый по ключу объект
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

/// Верхний объект извлекается по первому ключу,
/// далее из него извлекаются объекты по второму ключу.
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

/// Получает объект из JSON value
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

/// Извлекает данные ответа из `data` и возвращает JSON value
pub(crate) fn get_value_response(response: String) -> Result<Value, Error> {
    let data: Value = from_str(response.as_str()).unwrap();
    let res = data.as_object().unwrap().get("data").unwrap().clone();

    match res.is_null() {
        false => Ok(res),
        true => Err(get_error(&data)),
    }
}