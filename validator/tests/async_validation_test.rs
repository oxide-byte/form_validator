#![cfg(feature = "async")]
use validator::prelude::*;
use validator::validators::{MaxLength, MinLength};

fn run_async<F: core::future::Future<Output = ()>>(fut: F) {
    validator::futures::executor::block_on(fut)
}

#[test]
fn min_length_async() {
    run_async(async {
        let v = MinLength::new(5);
        assert!(v.validate_async(&"12345".to_string()).await.is_ok());
        let err = v
            .validate_async(&"1234".to_string())
            .await
            .unwrap_err();
        assert_eq!(err.code, "min_length");
        assert_eq!(err.params.get("limit").map(|s| s.as_ref()), Some("5"));
        assert_eq!(err.params.get("len").map(|s| s.as_ref()), Some("4"));
    })
}

#[test]
fn max_length_async() {
    run_async(async {
        let v = MaxLength::new(10);
        assert!(v
            .validate_async(&"0123456789".to_string())
            .await
            .is_ok());
        let err = v
            .validate_async(&"0123456789123".to_string())
            .await
            .unwrap_err();
        assert_eq!(err.code, "max_length");
        assert_eq!(err.params.get("limit").map(|s| s.as_ref()), Some("10"));
        assert_eq!(err.params.get("len").map(|s| s.as_ref()), Some("13"));
    })
}