use url::Url;
use wasm_bindgen::prelude::{
  wasm_bindgen,
  JsValue,
};

use crate::{
  coder::{
    self,
    SealedEncoded,
    Version00Coding,
  },
  web_api,
};

#[wasm_bindgen]
pub struct Encoded(coder::SealedEncoded);

#[allow(non_snake_case)]
#[wasm_bindgen]
impl Encoded
{
  #[wasm_bindgen(constructor)]
  pub fn new(game_over: &web_api::GameOver) -> Encoded
  {
    Encoded(Version00Coding::encode(game_over.inner()))
  }

  #[wasm_bindgen]
  pub fn asURLSearchParams(&self) -> String
  {
    serde_urlencoded::to_string(&self.0).unwrap()
  }

  #[wasm_bindgen]
  pub fn decode(url: String, unseen: Vec<JsValue>) -> Result<web_api::GameOver, String>
  {
    Ok(web_api::GameOver::from(
      Version00Coding::decode(
        serde_urlencoded::from_str::<SealedEncoded>(
          Url::parse(&url)
            .map_err(|e| format!("{}", e))?
            .query()
            .ok_or(String::from("url is missing search query"))?,
        )
        .map_err(|e| format!("{}", e))?
        .try_into()
        .map_err(|e| format!("{}", e))?,
        unseen.into_iter().map(|x| x.as_string().unwrap()).collect(),
      )
      .map_err(|e| format!("{}", e))?,
    ))
  }
}
