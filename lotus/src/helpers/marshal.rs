use fvm_shared::address::Address;
pub use serde::ser;

pub fn marshal_cbor<T>(a: &T) -> Vec<u8>
where
    T: ser::Serialize + ?Sized,
{
    fvm_ipld_encoding::to_vec(a).expect("header serialization cannot fail")
}

pub fn marshal_cbor_address(addr: &Address) -> Vec<u8> {
    marshal_cbor(addr)
}
