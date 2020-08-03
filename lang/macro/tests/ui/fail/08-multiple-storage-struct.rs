use liquid_lang as liquid;

#[liquid::contract(version = "0.1.0")]
mod noop {
    #[liquid(storage)]
    struct Noop_1 {}

    #[liquid(storage)]
    struct Noop_2 {}

    #[liquid(methods)]
    impl Noop_1 {
        pub fn constructor(&mut self) {}

        pub fn noop(&self) {}
    }

    #[liquid(methods)]
    impl Noop_2 {
        pub fn constructor(&mut self) {}

        pub fn noop(&self) {}
    }
}

fn main() {}
