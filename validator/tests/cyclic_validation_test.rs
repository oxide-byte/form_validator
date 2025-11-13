use validator::prelude::*;
use validator::validators::Email;

#[derive(validator::Validate)]
struct User<'a> {
    #[validate(Email)]
    email: String,
    members: Vec<&'a User<'a>>,
}

impl<'a> User<'a> {
    pub fn update_members(&mut self, members: Vec<&'a User<'a>>) {
        self.members = members;
    }
}

#[test]
fn validate_good_members() {
    let good_child = User {
        email: "good_child@test.com".into(),
        members: vec![],
    };

    let root = User {
        email: "root@example.com".into(),
        members: vec![&good_child],
    };

    assert!(root.validate().is_ok());
}

#[test]
fn validate_bad_members() {
    let bad_child = User {
        email: "not-an-email".into(),
        members: vec![],
    };

    let root = User {
        email: "root@example.com".into(),
        members: vec![&bad_child],
    };

    assert!(root.validate().is_ok());
}

#[test]
fn validate_cyclic_good_members() {
    let mut root = User {
        email: "root@example.com".into(),
        members: vec![],
    };
    let rp = &root as *const _;
    unsafe {
        root.update_members(vec![&*rp]);
    }
    assert!(root.validate().is_ok());
}

#[test]
fn validate_cyclic_bad_members() {
    let mut root = User {
        email: "not-an-email".into(),
        members: vec![],
    };
    let rp = &root as *const _;
    unsafe {
        root.update_members(vec![&*rp]);
    }
    assert!(root.validate().is_err());
}