mod thread_pool;
mod worker;

pub use thread_pool::{ThreadPool, ThreadPoolError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
