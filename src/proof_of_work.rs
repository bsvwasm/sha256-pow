use primitive_types::U256;
use rand::Rng;
use serde::*;
use sha2::Digest;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
    #[serde(with = "hex")]
    hash: Vec<u8>,
    #[serde(with = "hex")]
    template: Vec<u8>,
    nonce: u64
}

#[wasm_bindgen]
impl ProofOfWork {
    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    pub fn get_template(&self) -> Vec<u8> {
        self.template.clone()
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }

    pub fn from_json(json_string: &str) -> Result<ProofOfWork, JsValue> {
        match serde_json::from_str(json_string) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&format!("{:#?}", e))),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum POWErrors {
    InvalidTarget,
    CantFindTarget,
}

impl Into<JsValue> for POWErrors {
    fn into(self) -> JsValue {
        JsValue::from_str(&format!("{:#?}", self))
    }
}

#[wasm_bindgen]
/// Takes a template vector to be hashed and a target difficulty u256 in hex string form and will mine until it finds a hash that matches the desired difficulty
pub async fn mine(template: Vec<u8>, target_hex: String, offset: Option<u64>) -> Result<ProofOfWork, JsValue> {
    console_error_panic_hook::set_once();
    let target_u256 = U256::from_str_radix(&target_hex, 16).unwrap();

    let offset = match offset {
        Some(v) => v,
        None => 0,
    };

    for nonce in offset..u64::MAX {
      match get_proof(&template, nonce, &target_u256) {
        Some(v) => return Ok(v),
        None => ()
      }
    }

    Err(POWErrors::CantFindTarget.into())
}

#[wasm_bindgen]
/// Takes a template vector to be hashed and a target difficulty u256 in hex string form and will mine until it finds a hash that matches the desired difficulty
pub async fn mine_random(template: Vec<u8>, target_hex: String) -> Result<ProofOfWork, JsValue> {
    console_error_panic_hook::set_once();
    let target_u256 = U256::from_str_radix(&target_hex, 16).unwrap();

    let mut rng = rand::thread_rng();

    loop {
        let nonce: u64 = rng.gen();

        match get_proof(&template, nonce, &target_u256) {
          Some(v) => return Ok(v),
          None => ()
        }
    }
}

pub fn get_proof(template: &[u8], nonce: u64, target_u256: &U256) -> Option<ProofOfWork> {
  let mut hasher = sha2::Sha256::new();
  hasher.update(&template);
  hasher.update(&nonce.to_le_bytes());
  let hash = hasher.finalize();
  let hash_bytes = hash.as_slice();
  let hash_value = U256::from_little_endian(hash_bytes);

  if &hash_value < target_u256 {
      return Some(ProofOfWork {
          hash: hash_bytes.to_vec(),
          template: template.to_vec(),
          nonce,
      });
  }

  None
}

#[wasm_bindgen]
pub fn verify(pow: &ProofOfWork, target_hex: &str) -> bool {
    console_error_panic_hook::set_once();

    let target_u256 = U256::from_str_radix(&target_hex, 16).unwrap();
    let mut hasher = sha2::Sha256::new();
    hasher.update(&pow.get_template());
    hasher.update(&pow.get_nonce().to_le_bytes());
    let hash = hasher.finalize();
    let hash_bytes = hash.as_slice();
    let hash_value = U256::from_little_endian(hash_bytes);

    if hash_bytes != pow.get_hash() {
        return false;
    }

    if hash_value > target_u256 {
        return false;
    }

    true
}
