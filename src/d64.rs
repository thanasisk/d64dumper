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
            }
        }
    }
}
