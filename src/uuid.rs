use std::fmt::{Debug, Formatter};

struct Uuid (u128);


fn generate(version: &str) -> Uuid {
    Uuid { 0: 0 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_creation() {
        let a = Uuid(91812314343222919191914343423234273641);
        println!("{:01$X}", a.0, 32);

        assert_eq!(a.0, 91812314343222919191914343423234273641);
    }
}