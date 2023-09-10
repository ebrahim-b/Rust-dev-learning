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
    Aquatic,
}

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



fn main() {
    let lion = Animal {
        animal_species:Animal_species::Mammal,
        name: String::from("Lion"),
        location: Location::Ground,
    };

    println!("{:?}",lion);
}
