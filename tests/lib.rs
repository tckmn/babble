#![feature(box_syntax)]

extern crate babble;

// a utility function to run code as a string and return its STDOUT
fn run(code: String) -> String {
    let mut stdout: Vec<u8> = Vec::new();
    let mut stdin = ::std::io::empty();
    let mut b = babble::Babble::new();
    b.run_with_io(code, &mut stdout, &mut stdin);
    String::from_utf8(stdout).unwrap()
}

#[test]
fn test_literals() {
    assert_eq!(String::from("Hello, World!"), run(String::from("ARRHZLEZLLZLLZLOZTBTZTBHWZLOZLRZLLZLDZTBIZEPUT")));
    assert_eq!(String::from("27"), run(String::from("NUMABCZPUT")));
    assert_eq!(String::from("-1980"), run(String::from("NUMZDEFZPUT")));
    assert_eq!(String::from("437/654"), run(String::from("NUMZZGHIZJKLZPUT")));
    assert_eq!(String::from("-871/1088"), run(String::from("NUMZZZMNOZPQRZPUT")));
}

#[test]
fn test_math() {
    assert_eq!(String::from("3"), run(String::from("ONEPVBTWOPVAADDPVCPUT")));
    assert_eq!(String::from("3"), run(String::from("FIVPVBTWOPVASUBPVCPUT")));
    assert_eq!(String::from("25"), run(String::from("FIVSVARVAMULPUT")));
    assert_eq!(String::from("2"), run(String::from("TENPVBFIVPVADIVPVCPUT")));
    assert_eq!(String::from("32"), run(String::from("TWOPVBFIVPVAPOWPVCPUT")));
}

#[test]
fn test_control_flow() {
    assert_eq!(String::from("10987654321"), run(String::from("ONEXPSTENXPRBLKXPRPUTSUBENDWHL")));
    assert_eq!(String::from("10"), run(String::from("ONEXPSTENXPRBLKXPRPUTSUBENDXIF")));
}
