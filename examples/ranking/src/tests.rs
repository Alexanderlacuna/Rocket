use rocket::local::blocking::Client;

fn test(uri: String, expected: String) {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get(&uri).dispatch();
    assert_eq!(response.into_string(), Some(expected));
}

#[test]
fn test_hello() {
    for &(name, age) in &[("Mike", 22), ("Michael", 80), ("A", 0), ("a", 127)] {
        test(format!("/hello/{}/{}", name, age),
            format!("Hello, {} year old named {}!", age, name));
    }
}

#[test]
fn test_failing_hello_hi() {
    // Invalid integers.
    for &(name, age) in &[("Mike", 1000), ("Michael", 128), ("A", -800), ("a", -200)] {
        test(format!("/hello/{}/{}", name, age),
            format!("Hi {}! Your age ({}) is kind of funky.", name, age));
    }

    // Non-integers.
    for &(name, age) in &[("Mike", "!"), ("Michael", "hi"), ("A", "blah"), ("a", "0-1")] {
        test(format!("/hello/{}/{}", name, age),
            format!("Hi {}! Your age ({}) is kind of funky.", name, age));
    }
}
