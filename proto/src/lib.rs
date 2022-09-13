use once_cell::sync::Lazy;

pub mod message;
pub mod service;

static DESCRIPTOR_POOL: Lazy<prost_reflect::DescriptorPool> = Lazy::new(|| {
    prost_reflect::DescriptorPool::decode(
        include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
    )
    .unwrap()
});
