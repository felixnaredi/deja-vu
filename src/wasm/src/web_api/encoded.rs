use url::Url;
use wasm_bindgen::prelude::{
  wasm_bindgen,
  JsValue,
};

use crate::{
  coder::{
    self,
    Version00Coding,
  },
  web_api,
};

#[wasm_bindgen]
pub struct Encoded(coder::Encoded);

#[wasm_bindgen]
impl Encoded
{
  #[wasm_bindgen]
  pub fn version(&self) -> String
  {
    self.0.version().into()
  }

  #[wasm_bindgen]
  pub fn data(&self) -> String
  {
    self.0.data().into()
  }

  #[wasm_bindgen]
  pub fn checksum(&self) -> u64
  {
    self.0.checksum()
  }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
impl Encoded
{
  #[wasm_bindgen(constructor)]
  pub fn new(history: &web_api::History) -> Encoded
  {
    Encoded(Version00Coding::encode(history.inner()))
  }

  #[wasm_bindgen]
  pub fn asURLSearchParams(&self) -> String
  {
    serde_urlencoded::to_string(&self.0).unwrap()
  }

  #[wasm_bindgen]
  pub fn decode(url: String, unseen: Vec<JsValue>) -> Result<web_api::History, String>
  {
    Ok(web_api::History::from(
      Version00Coding::decode(
        serde_urlencoded::from_str(
          Url::parse(&url)
            .map_err(|e| format!("{}", e))?
            .query()
            .ok_or(String::from("error - url is missing search query"))?,
        )
        .map_err(|e| format!("{}", e))?,
        unseen.into_iter().map(|x| x.as_string().unwrap()).collect(),
      )
      .map_err(|e| format!("{}", e))?,
    ))
  }
}
