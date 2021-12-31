// #[cfg(test)]
// mod tests {
//   extern crate wasm_bindgen_test;
//   use wasm_bindgen_test::*;
//   wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
//   // wasm_bindgen_test::wasm_bindgen_test_configure!();
  
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
// }