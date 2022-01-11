pub mod petscii {
    pub fn pet2ascii(mut pet: u8) -> u8 {
        //If the PETSCII character is 192-223, subtract 96. Then subtract 32 if the resultant value is 97-122.
        if pet >= 192 && pet < 224 {
            pet -= 96;
            if pet > 96 && pet < 123 {
                pet -= 32;
            }
        //If the PETSCII character is A-Z, make it a-z (PETSCII 97-122, subtract 32)
        } else if pet > 96 && pet < 123 {
            pet -= 32
        //If the PETSCII character is a-z, make it A-Z (PETSCII 65-90, add 32)
        } else if pet > 64 && pet < 91 {
            pet += 32
        } else if pet == 0xa0 {
            pet = 0x20
        }
        return pet;
    }
    pub fn pet2unicode(pet: u8) -> String {
        // bug here?
        return (u16::from(pet) + 0xE000).to_string();
    }
}
