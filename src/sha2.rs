use alloy_primitives::address;
pub fn sha2(data: &[u8]) -> [u8; 32] {
    let ret = stylus_sdk::call::RawCall::new_static()
        .call(address!("0000000000000000000000000000000000000002"), data)
        .unwrap();
    let mut out = [0; 32];
    out.copy_from_slice(&ret);
    out
}
