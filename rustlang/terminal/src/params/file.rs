use fairspec_library::fairspec_metadata::IntegrityType;

#[fireargs::option(default = HashType::Sha256)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl From<HashType> for IntegrityType {
    fn from(hash_type: HashType) -> Self {
        match hash_type {
            HashType::Md5 => IntegrityType::Md5,
            HashType::Sha1 => IntegrityType::Sha1,
            HashType::Sha256 => IntegrityType::Sha256,
            HashType::Sha512 => IntegrityType::Sha512,
        }
    }
}
