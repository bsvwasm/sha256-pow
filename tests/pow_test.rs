#[cfg(test)]
mod tests {
  extern crate wasm_bindgen_test;
  use sha256_pow::ProofOfWork;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
  // wasm_bindgen_test::wasm_bindgen_test_configure!();
  
//   #[wasm_bindgen_test]
//   async fn easy_mine_target() {
//     let target = "0021e8000000000000000000000000000000000000000000000000000000000".to_string();
//     println!("mining to target {}...", &target);

//     let output = sha256_pow::mine(b"Hello, PoW!".to_vec(), target.clone(), None).await.unwrap();


//     println!("finished mining!");
//     assert_eq!(sha256_pow::verify(&output, &target), true);
//   }

//   #[wasm_bindgen_test]
//   async fn medium_mine_target() {
//     let target = "000021e80000000000000000000000000000000000000000000000000000000".to_string();
//     println!("mining to target {}...", &target);

//     let output = sha256_pow::mine(b"Hello, PoW!".to_vec(), target.clone(), None).await.unwrap();


//     println!("finished mining!");
//     assert_eq!(sha256_pow::verify(&output, &target), true);
//   }

//   #[wasm_bindgen_test]
//   async fn easy_mine_target_random() {
//     let target = "0021e8000000000000000000000000000000000000000000000000000000000".to_string();
//     println!("mining to target {}...", &target);

//     let output = sha256_pow::mine_random(b"Hello, PoW!".to_vec(), target.clone()).await.unwrap();


//     println!("finished mining!");
//     assert_eq!(sha256_pow::verify(&output, &target), true);
//   }

//   #[wasm_bindgen_test]
//   async fn medium_mine_target_random() {
//     let target = "000021e80000000000000000000000000000000000000000000000000000000".to_string();
//     println!("mining to target {}...", &target);

//     let output = sha256_pow::mine_random(b"Hello, PoW!".to_vec(), target.clone()).await.unwrap();


//     println!("finished mining!");
//     assert_eq!(sha256_pow::verify(&output, &target), true);
//   }

  #[wasm_bindgen_test]
  async fn verify_mine_target_from_json() {
    let target = "0021e8000000000000000000000000000000000000000000000000000000000".to_string();
    println!("mining to target {}...", &target);

    let output = sha256_pow::mine_random(b"Hello, PoW!".to_vec(), target.clone()).await.unwrap();

    assert_eq!(sha256_pow::verify(&output, &target), true);

    let pow_json = output.to_json().unwrap();

    let reconstructed_pow = ProofOfWork::from_json(pow_json).unwrap();

    assert_eq!(output.get_template(), reconstructed_pow.get_template(), "Templates didnt match after reconstruction");
    assert_eq!(output.get_nonce(), reconstructed_pow.get_nonce(), "Nonces didnt match after reconstruction");
    assert_eq!(output.get_hash(), reconstructed_pow.get_hash(), "Hashes didnt match after reconstruction");

    println!("finished mining!");
    assert_eq!(sha256_pow::verify(&reconstructed_pow, &target), true, "Failed to verify after JSON conversion");
  }
}