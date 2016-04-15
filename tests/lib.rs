#![feature(box_syntax)]

extern crate babble;
use babble::babble::Babble;

fn run(code: String) -> String {
    let mut stdout: Vec<u8> = Vec::new();
    let mut stdin = ::std::io::empty();
    {
        let mut b = Babble::new(&mut stdout, &mut stdin);
        b.run(code);
    }
    String::from_utf8(stdout).unwrap()
}

#[test]
fn test_hello_world() {
    assert_eq!(String::from("Hello, World!"), run(String::from("ARRHZLEZLLZLLZLOZTBTZTBHWZLOZLRZLLZLDZTBIZEPUT")));
}
