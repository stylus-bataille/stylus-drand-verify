use alloy_primitives::address;
pub fn sha2(data: &[u8], out: &mut [u8; 32]) {
    let ret = stylus_sdk::call::RawCall::new_static().call(address!("0000000000000000000000000000000000000002"), data).unwrap();
    out.copy_from_slice(&ret);
}
