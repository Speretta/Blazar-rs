use crate::util::uid::UUID;

#[test]
fn creation_uuid() {
    assert!(
        UUID::from_be_bytes([0; 16]).to_string().as_str() == "00000000-0000-0000-0000-000000000000"
    )
}
