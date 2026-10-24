use crate::models::descriptor::Descriptor;

pub fn copy_descriptor(descriptor: &Descriptor) -> Descriptor {
    descriptor.clone()
}
