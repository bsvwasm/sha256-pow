#[cfg(test)]
mod xor_tests {
  extern crate wasm_bindgen_test;
  use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
  // wasm_bindgen_test::wasm_bindgen_test_configure!();
  
  #[wasm_bindgen_test]
  async fn xor_in_xor_out() {
    let a = b"00021e8000000000000000000000000000000000000000000000000000000000";
    let b = b"39bea76fe07b762c8540b5b3d7967faf643e46eeaf6ec0c132a8b1b969734d65";

    let v = sha256_pow::xor(a, b).unwrap();

    assert_eq!(v, [3, 9, 82, 87, 80, 82, 14, 86, 85, 0, 7, 82, 7, 6, 2, 83, 8, 5, 4, 0, 82, 5, 82, 3, 84, 7, 9, 6, 7, 86, 81, 86, 6, 4, 3, 85, 4, 6, 85, 85, 81, 86, 6, 85, 83, 0, 83, 1, 3, 2, 81, 8, 82, 1, 82, 9, 6, 9, 7, 3, 4, 84, 6, 5]);

    let reversed_a = sha256_pow::xor(&v, b).unwrap();

    assert_eq!(reversed_a, a);


    let reversed_b = sha256_pow::xor(&v, a).unwrap();

    assert_eq!(reversed_b, b);
  }
}