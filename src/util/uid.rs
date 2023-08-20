use std::fmt::{Display, Debug};


pub struct UUID{
    most: u64,
    least: u64
}


impl UUID{
    pub fn to_be_bytes(&self) -> [u8; 16]{
        let mut be_bytes = [0; 16];
        self.most.to_be_bytes().into_iter().chain(self.least.to_be_bytes()).enumerate().for_each(|(i, x)|  {
            assert!(i<be_bytes.len(), "Corrupted UUID bytes");
            be_bytes[i] = x;
        });
        be_bytes
    }

    pub fn from_be_bytes(bytes: [u8; 16]) -> Self{
        let most = u64::from_be_bytes(bytes[..8].try_into().unwrap());
        let least = u64::from_be_bytes(bytes[8..].try_into().unwrap());
        UUID { most, least }
    }
}


impl Display for UUID{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let be_bytes = self.to_be_bytes();
        write!(f, "{:x}{:x}{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}", be_bytes[0], be_bytes[1], be_bytes[2], be_bytes[3], be_bytes[4], be_bytes[5], be_bytes[6], be_bytes[7], be_bytes[8], be_bytes[9], be_bytes[10], be_bytes[11], be_bytes[12], be_bytes[13], be_bytes[14], be_bytes[15])
    }
}

impl Debug for UUID{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}



impl From<[u64; 2]> for UUID{
    fn from(value: [u64; 2]) -> Self { 
        UUID { most: value[0], least: value[0]}
    }
}
