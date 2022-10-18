use url::Url;
use wasm_bindgen::prelude::{
  wasm_bindgen,
  JsValue,
};

use crate::{
  coder::{
    self,
    GameOverCoderV01,
    SealedEncodedGameOver,
  },
  game_over,
  web_api,
};

#[wasm_bindgen]
pub struct EncodedGameOver(coder::SealedEncodedGameOver);

#[allow(non_snake_case)]
#[wasm_bindgen]
impl EncodedGameOver
{
  #[wasm_bindgen(constructor)]
  pub fn new(game_over: &web_api::GameOver) -> Result<EncodedGameOver, String>
  {
    coder::SealedEncodedGameOver::new::<GameOverCoderV01, _>(game_over.inner())
      .map(|x| EncodedGameOver(x))
      .map_err(|e| format!("{}", e))
  }

  #[wasm_bindgen]
  pub fn asURLSearchParams(&self) -> String
  {
    serde_urlencoded::to_string(&self.0).unwrap()
  }

  #[wasm_bindgen]
  pub fn decode(url: String, unseen: Vec<JsValue>) -> Result<web_api::GameOver, String>
  {
    let seal = serde_urlencoded::from_str::<SealedEncodedGameOver>(
      Url::parse(&url)
        .map_err(|e| format!("{}", e))?
        .query()
        .ok_or("url has no search query")?,
    )
    .map_err(|e| format!("{}", e))?;

    let unseen = unseen.into_iter().map(|x| x.as_string().unwrap()).collect();

    let game_over: game_over::GameOver<String> =
      (seal, unseen).try_into().map_err(|e| format!("{}", e))?;
    Ok(web_api::GameOver::from(game_over))
  }
}
