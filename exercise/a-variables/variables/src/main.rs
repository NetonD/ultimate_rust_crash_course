const START_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {

    START_MISSILES = 1_000_000_000;
    let unused;
    let (mut missiles, ready) = (START_MISSILES, READY_AMOUNT); 

    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles = missiles - ready);
}

