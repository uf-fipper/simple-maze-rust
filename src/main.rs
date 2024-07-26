use maze::{console_game::ConsoleGame, game::Game};
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let u8array: &[u8] = "aosndfjewnjqkerkjwqnerijwerneafqwefqqweffe".as_bytes();
    let mut seed: <ChaCha8Rng as SeedableRng>::Seed;
    if u8array.len() >= 32 {
        seed = u8array[0..32].try_into().unwrap();
    } else {
        seed = [0; 32];
        seed[..u8array.len()].clone_from_slice(u8array);
    }
    println!("{:?}", seed);
    let mut random = thread_rng();
    random.fill(&mut seed);
    ConsoleGame::new_with_random(10, 20, random)
        .unwrap()
        .run()
        .unwrap();
}
