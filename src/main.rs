extern crate babble;
use babble::babble::Babble;

fn main() {
    let mut b = Babble::new();
    b.run("ARRHZLEZLLZLLZLOZTBTZTBHWZLOZLRZLLZLDZTBIZEPUT".to_string());
    b.run("ONEPVBONEPVAADDPVCPUT".to_string());
    b.run("PVAFIVPVBTWOPVASUBPVCPUT".to_string());
}
