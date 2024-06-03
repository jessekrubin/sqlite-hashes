use digest::Digest;
#[cfg(feature = "hex")]
use hex::ToHex as _;

#[derive(Debug, Default)]
pub enum HashState<T> {
    #[default]
    Created,
    Started,
    HasValues(T),
}

impl<T: Digest + Clone> HashState<T> {
    #[inline]
    pub fn add_null(&mut self) {
        if let Self::Created = self {
            *self = Self::Started;
        }
    }

    #[inline]
    pub fn add_value(&mut self, value: &[u8]) {
        match self {
            Self::Created | Self::Started => {
                let mut hasher = T::new();
                hasher.update(value);
                *self = Self::HasValues(hasher);
            }
            Self::HasValues(hasher) => {
                hasher.update(value);
            }
        }
    }

    #[inline]
    #[cfg(feature = "window")]
    pub fn calc(&self) -> Option<Vec<u8>> {
        match self {
            Self::Created | Self::Started => None,
            Self::HasValues(hasher) => Some(hasher.clone().finalize().to_vec()),
        }
    }

    #[inline]
    #[cfg(all(feature = "window", feature = "hex"))]
    pub fn calc_hex(&self) -> Option<String> {
        match self {
            Self::Created => None,
            Self::Started => Some(String::new()),
            Self::HasValues(hasher) => Some(hasher.clone().finalize().to_vec().encode_hex_upper()),
        }
    }

    #[inline]
    #[cfg(all(feature = "window", feature = "int"))]
    pub fn calc_int(&self) -> Option<i64> {
        match self {
            Self::Created => None,
            Self::Started => Some(0),
            Self::HasValues(hasher) => Some(i64::from_be_bytes(
                hasher
                    .clone()
                    .finalize()
                    .to_vec()
                    .try_into()
                    .unwrap_or_default(),
            )),
        }
    }

    #[inline]
    pub fn finalize(self) -> Option<Vec<u8>> {
        match self {
            Self::Created | Self::Started => None,
            Self::HasValues(hasher) => Some(hasher.finalize().to_vec()),
        }
    }

    #[inline]
    #[cfg(feature = "hex")]
    pub fn finalize_hex(self) -> Option<String> {
        match self {
            Self::Created => None,
            Self::Started => Some(String::new()),
            Self::HasValues(hasher) => Some(hasher.finalize().to_vec().encode_hex_upper()),
        }
    }

    #[inline]
    #[cfg(feature = "int")]
    pub fn finalize_int(self) -> Option<i64> {
        match self {
            Self::Created => None,
            Self::Started => Some(0),
            Self::HasValues(hasher) => Some(
                hasher
                    .finalize()
                    .to_vec()
                    .iter()
                    .fold(0, |acc, &x| acc * 256 + x as i64),
            ),
        }
    }
}
