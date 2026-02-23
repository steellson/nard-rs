mod throw;

use crate::throw::throw::Throw;

fn main() {
    for s in 0..=20 {
        let step = Throw::new();
        println!("Step #{}: First {}, Second: {}, Jackpot: {}", s, step.first, step.second, step.is_jackpot)
    }
}
