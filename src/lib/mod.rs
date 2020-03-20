pub mod threadpool;
pub mod router;
pub mod handler;
pub mod httpcode;
pub mod config;
pub mod actionresult;

use crate::lib::threadpool;

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_thread_pull_with_valid_size() {
        let is_valid = match other_threadpool::ThreadPool::new(4) {
            Ok(pool) => true,
            Err(_) => false,    
        };

        assert_eq!(is_valid, true);
    }
}
