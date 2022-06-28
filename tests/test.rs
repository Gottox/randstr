use randstr::randstr;

#[test]
fn check_all_uppercase() {
    let mut rand = randstr().upper().len(1024).build();
    assert!(rand.generate().chars().all(|c| c.is_uppercase()));
}

#[test]
fn check_all_lowercase() {
    let mut rand = randstr().lower().len(1024).build();
    assert!(rand.generate().chars().all(|c| c.is_lowercase()));
}

#[test]
fn check_all_digit() {
    let mut rand = randstr().digit().len(1024).build();
    assert!(rand.generate().chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn check_must_digit() {
    let mut rand = randstr().all().must_digit().len(1).build();
    assert!(rand.generate().chars().any(|c| c.is_ascii_digit()));
}
