extern crate babble;

fn main() {
    let mut b = babble::babble::Babble::new();
    b.run("ARRHZLEZLLZLLZLOZTBTZTBHWZLOZLRZLLZLDZTBIZEPUT".to_string());
    b.run("ONEPVBONEPVAADDPVCPUT".to_string());
    b.run("PVAFIVPVBTWOPVASUBPVCPUT".to_string());
}
