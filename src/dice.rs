use rand::distributions::{Distribution, Uniform};


pub fn roll_dice(d: u32) -> u32{
    let mut rng = rand::thread_rng();
    let dice = Uniform::from(1..d);

    dice.sample(&mut rng)
}