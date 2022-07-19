use alignment::Alignment;
use background::Background;

mod dice;
mod background;
mod alignment;

fn main() {
    let player = setup_hero("Suel".to_string(), "Druid".to_string(), "human".to_string());
    player.print_hero();

}


struct Hero {
    name: String,
    alignment: Alignment,
    background: Background,
    race: String,
    class: String,
    strength: u32,
    dexterity: u32,
    constitution: u32,
    intelligence: u32,
    wisdom: u32,
    charisma: u32
}
/* 
impl Hero{
    fn new(name: String, race: Race, class: Class, background: Background) -> Hero {
    }
}*/

fn setup_hero(name: String, class: String, race: String) -> Hero {
    Hero {
        name: name.to_string(),
        alignment: Alignment::ChaoticEvil,
        background: Background::Acolyte,
        race: race.to_string(),
        class: class.to_string(),
        strength: dice::roll_dice(6),
        dexterity: dice::roll_dice(6),
        constitution: dice::roll_dice(6),
        intelligence: dice::roll_dice(6),
        wisdom: dice::roll_dice(6),
        charisma: dice::roll_dice(6)
    }
}

impl Hero{
    fn print_hero(&self){
        println!("name: \"{}\"", self.name);
        println!("class: \"{}\"", self.class);
        println!("race: \"{}\"", self.race);
        println!("alignment \"{:?}\"", self.alignment);
        println!("background \"{:?}\"", self.background);
        println!("strength: \"{}\"", self.strength);
        println!("dexterity: \"{}\"", self.dexterity);
        println!("constitution: \"{}\"", self.constitution);
        println!("intelligence: \"{}\"", self.intelligence);
        println!("wisdom: \"{}\"", self.wisdom);
        println!("charisma: \"{}\"", self.charisma);
    }

    fn sheet(&self){
        println!("Name: {}", self.name);
        println!("Race: {}", self.race);
    }
}
