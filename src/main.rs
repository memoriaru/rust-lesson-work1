pub mod mod1 {
    pub fn mod1_print() {
        println!("sub mod1");
        for idx in 65..=122 {
            if idx > 64 && idx < 91 {
                println!("{}", char::from_u32(idx).unwrap().to_ascii_lowercase());
            } else if idx > 96 && idx < 123 {
                println!("{}", char::from_u32(idx).unwrap().to_ascii_uppercase());
            } else {
                println!("{}", char::from_u32(idx).unwrap())
            }
        }
    }

    pub mod mod2 {
        pub fn mod2_print() {
            println!("sub mod2");
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
