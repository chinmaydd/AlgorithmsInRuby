#[macro_use]
extern crate lazy_static;
extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

mod util;
mod first;
mod second;
mod third;
mod fourth;
mod fifth;

fn main() {
    // first::run();
    second::run();
    // third.run();
    // fourth.run();
    fifth::run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
