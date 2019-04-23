#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_lang::contract;
use ink_core::storage;
use ink_core::env::println;
use ink_core::memory::format;

contract! {
    struct Incrementer {
        value: storage::Value<u64>,
    }

    impl Deploy for Incrementer {
        fn deploy(&mut self, init_value: u64) {
            self.value.set(init_value)
        }
    }

    impl Incrementer {
        pub(external) fn get(&self) -> u64 {
            // ACTION: Use `println` to print the value of `self.value`
            // ACTION: Return `self.value`
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::Incrementer;

    #[test]
    fn incrementer_works() {
        let mut contract = Incrementer::deploy_mock(5);
        assert_eq!(contract.get(), 5);
    }
}
