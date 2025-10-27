mod erroe;
// 將型別重新導出，確保 pub fn p 的簽章不會引用私有型別
pub use crate::erroe::{ AppError, AppResult };

use nom::{ bytes::complete::{ take_till1 } };
use serde_json::json;

pub fn get_json_value(path: &str, date: serde_json::Value) -> AppResult<serde_json::Value> {
  if path == "#" {
    if date.is_array() {
      // get len
      return Ok(json!(date.as_array().unwrap().len()));
    } else {
      return Err(AppError::SyntaxError(format!("{date:?} is not array")));
    }
  }
  match take_till1::<_, _, nom::error::Error<&str>>(|c| c == '.')(path) {
    Ok(r) => {
      let (input, match_result) = r;
      let o = date.get(match_result);
      if o.is_none() {
        return Err(AppError::ParseError(format!("value {match_result} does not exist.")));
      }
      let o = o.unwrap();
      //   println!("input={input}\nm={m}\no={o}");

      if !input.is_empty() {
        let (_, input) = input.split_at(1);
        let o2 = get_json_value(input, o.clone())?;
        // println!("o2={o2}");
        Ok(o2)
      } else {
        Ok(o.clone())
      }
    }
    Err(e) => {
      // 將 nom 錯誤轉成字串回報
      Err(AppError::ParseError(e.to_string()))
    }
  }
}

#[cfg(test)]
mod tests {
  use serde_json::json;

  use super::*;

  #[test]
  fn test_json_path_retrieval() {
    let i = json!({"a":{"b":[0,1,2,3],"a":"a"},"b":"b"});
    let path = "a.b.#";
    let o = get_json_value(path, i).unwrap();
    // println!("{o:#?}");
    assert_eq!(o, json!(4))
  }

  #[test]
  fn validate_json_path_with_non_array() {
    let i = json!({"a":{"b":4,"a":"a"},"b":"b"});
    let path = "a.b.#";
    let o = get_json_value(path, i.clone());
    assert_eq!(
      o,
      Err(
        AppError::SyntaxError(format!("{:?} is not array", i.get("a").unwrap().get("b").unwrap()))
      )
    )
  }
}
