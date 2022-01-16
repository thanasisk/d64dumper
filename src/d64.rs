pub mod d64 {
    pub struct DirEntry {
        pub track: u8,
        pub sector: u8,
        pub ftype: u8,
        pub ftrack: u8,
        pub fsector: u8,
        pub low_sz: u8,
        pub high_sz: u8,
        // can be made private + methods
        pub sector_sz: usize,
        pub byte_sz: usize,
        pub dname: String,
        pub dlen: usize,
    }
    impl DirEntry {
        pub fn new() -> DirEntry {
            DirEntry {
                track: 0,
                sector: 0,
                ftype: 0,
                ftrack: 0,
                fsector: 0,
                low_sz: 0,
                high_sz: 0,
                sector_sz: 0,
                byte_sz: 0,
                dname: String::new(),
                dlen: 0x15 - 0x05,
            }
        }
        pub fn print(&self) {
            print!("{}\t{}\t", self.track, self.sector);
            match self.ftype {
                0x00 => print!("Scratched\t"),
                0x80 => print!("DEL\t"),
                0x81 => print!("SEQ\t"),
                0x82 => print!("PRG\t"),
                0x83 => print!("USR\t"),
                0x84 => print!("REL\t"),
                _ => print!("undefined!\t"),
            }
            print!("{} {} ", self.ftrack, self.fsector);
            print!("{} {}\t", self.sector_sz, self.byte_sz);
            println!("{}", self.dname);
        }
    }
}
