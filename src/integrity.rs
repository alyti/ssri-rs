use crate::hash::Hash;
use crate::algorithm::Algorithm;
use crate::builder::Builder;
use std::fmt;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct Integrity {
    pub hashes: Vec<Hash>
}

impl fmt::Display for Integrity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hashes.iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>()
            .join(" "))
    }
}

impl std::str::FromStr for Integrity {
    type Err = ParseIntegrityError;

    fn from_str(s: &str) -> Result<Integrity, Self::Err> {
        let hashes = String::from(s)
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<Hash>, Self::Err>>()?;
        Ok(Integrity { hashes })
    }
}

impl Integrity {
    pub fn from<B: AsRef<[u8]>>(data: B, algorithm: Algorithm) -> Integrity {
        let mut builder = Builder::new();
        builder.algorithm(algorithm);
        builder.input(&data);
        builder.result()
    }
    pub fn concat(&self, other: Integrity) -> Self {
        let mut hashes = [self.hashes.clone(), other.hashes.clone()].concat();
        hashes.sort_unstable();
        hashes.dedup();
        Integrity { hashes }
    }
    pub fn check<B: AsRef<[u8]>>(&self, data: B) -> Result<Algorithm, ParseIntegrityError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ParseIntegrityError {}
impl fmt::Display for ParseIntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse Subresource Integrity string")
    }
}
impl Error for ParseIntegrityError {}

#[cfg(test)]
mod tests {
    use super::Hash;
    use super::Algorithm;
    use super::Integrity;

    #[test]
    fn parse() {
        let sri: Integrity = "sha1-deadbeef=".parse().unwrap();
        assert_eq!(
            sri.hashes.get(0).unwrap(),
            &Hash {
                algorithm: Algorithm::Sha1,
                digest: String::from("deadbeef=")
            }
        )
    }
}
