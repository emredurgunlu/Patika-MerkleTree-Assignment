#![allow(non_snake_case)]

use merklelib::merkle::merkle_root::merkle_root;
fn main() {
    merkle_root("input0.txt".to_string());
}

// Pass all tests!
#[cfg(test)]
mod tests {
    use merklelib::merkle::merkle_root::merkle_root;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(
            result,
            "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9"
        );
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(
            result,
            "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e"
        );
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(
            result,
            "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e"
        );
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(
            result,
            "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe"
        );
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(
            result,
            "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597"
        );
    }
}
