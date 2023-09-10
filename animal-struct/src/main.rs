#[derive(Debug)]
enum Location {
    Ground,
    Water,
    Air,
}

#[derive(Debug)]
enum Animal_species {
    Bird,
    Mammal,
    Amphibian,
    Creepy,
    Aquatic(Aquatic_species),
}

#[derive(Debug)]
enum Aquatic_species{
    Crab,
    Octopus,
    Fish,
    Clam,
}

#[derive(Debug)]
struct Animal{
    animal_species: Animal_species,
    name: String,
    location: Location,
}

impl Animal{
    fn show(&self){
        match &self.animal_species{
            Animal_species::Aquatic(x) => println!("{:?} is a {:?} and live in {:?}",self.name,x,self.location),
            _ => println!("{:?} is a {:?} and live in {:?}",self.name,self.animal_species,self.location),
        }
        
    }
}



fn main() {
    let lion = Animal {
        animal_species:Animal_species::Mammal,
        name: String::from("Lion"),
        location: Location::Ground,
    };


    let clown_fish = Animal {
        animal_species: Animal_species::Aquatic(Aquatic_species::Fish),
        name: String::from("Clown Fish"),
        location: Location::Water,
    };
    //println!("{:?}",lion);
    lion.show();
    clown_fish.show();
}
