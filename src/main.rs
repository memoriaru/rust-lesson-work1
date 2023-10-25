pub mod mod1 {
    pub fn mod1_print() {
        for idx in (65..=122).rev() {
            println!("{}", char::from_u32(idx).unwrap())
        }
    }

    pub mod mod2 {
        pub fn mod2_print() {
            for idx in 65..=122 {
                println!("{}", char::from_u32(idx).unwrap())
            }
        }
    }
}


fn main() {
    use mod1::mod1_print;
    use mod1::mod2::mod2_print;

    mod1_print();
    mod2_print();
}
