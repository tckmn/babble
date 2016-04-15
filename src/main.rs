extern crate babble;
use babble::babble::Babble;

fn main() {
    let (mut stdout, mut stdin) = (::std::io::stdout(), ::std::io::stdin());
    let mut b = Babble::new(&mut stdout, &mut stdin);
    b.run("ARRHZLEZLLZLLZLOZTBTZTBHWZLOZLRZLLZLDZTBIZEPUT".to_string());
    b.run("ONEPVBONEPVAADDPVCPUT".to_string());
    b.run("PVAFIVPVBTWOPVASUBPVCPUT".to_string());
}
