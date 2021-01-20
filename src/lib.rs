mod helpers;
mod session;

// pub use network_manager::*;
pub use session::PowerManager;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
