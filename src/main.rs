pub mod d64;
pub mod petscii;

use crate::d64::d64::DirEntry;
use crate::petscii::petscii::pet2ascii;
use std::env;
use std::fs;
use std::process::exit;
use std::str;

fn main() {
    println!("D64 Dumper");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "help" {
        usage()
    }
    match args[1].as_str() {
        "version" => println!("0.1"),
        "list" => {
            if args.len() < 3 {
                usage()
            } else {
                let ref dir = &args[2];
                list(dir.to_string());
            }
        }
        "dump" => {}
        _ => usage(),
    }
    std::process::exit(0);
}
fn usage() {
    println!("λελ");
}

fn list(fname: String) {
    // -> Result<Vec<u8>, io::Error> {
    println!("Processing {}", fname);
    if !str::to_lowercase(fname.as_str()).ends_with(".d64") {
        println!("not a .d64 file");
        std::process::exit(-1);
    }
    let mut image = fs::read(fname).expect("problem reading file");
    let bam_offset = 0x16500;
    let direntry_offset = bam_offset + 256;
    let dentries_max = 0x08;
    let mut dentries = Vec::<DirEntry>::new();
    for i in 0..dentries_max {
        dentries.push(parse_direntry(&mut image, direntry_offset + (0x20 * i)));
    }
}

fn get_dname(image: &mut Vec<u8>, start_offset: usize, end_offset: usize) -> String {
    let dname = &mut image[start_offset..end_offset];
    for p in dname.iter_mut() {
        *p = pet2ascii(*p)
    }
    str::from_utf8(&dname).unwrap().trim_end().to_string()
}
fn parse_direntry(image: &mut Vec<u8>, start_offset: usize) -> DirEntry {
    let mut ret = DirEntry::new();
    ret.track = image[start_offset];
    ret.sector = image[start_offset + 1];
    ret.ftype = image[start_offset + 2];
    print!("{}\t{}\t", ret.track, ret.sector);
    match ret.ftype {
        0x00 => print!("Scratched\t"),
        0x80 => print!("DEL\t"),
        0x81 => print!("SEQ\t"),
        0x82 => print!("PRG\t"),
        0x83 => print!("USR\t"),
        0x84 => print!("REL\t"),
        _ => println!("undefined!\t"),
    }
    ret.ftrack = image[start_offset + 3];
    ret.fsector = image[start_offset + 4];
    print!("{} {} ", ret.ftrack, ret.fsector);
    ret.low_sz = image[start_offset + 0x1E];
    ret.high_sz = image[start_offset + 0x1F];
    ret.sector_sz = usize::from(ret.low_sz) + (usize::from(ret.high_sz) * 256);
    ret.byte_sz = (usize::from(ret.low_sz) + (usize::from(ret.high_sz) * 256)) * 254;
    print!("{} {}\t", ret.sector_sz, ret.byte_sz);
    ret.dname = get_dname(image, start_offset + 0x05, start_offset + 0x05 + ret.dlen);
    println!("{}", ret.dname);
    return ret;
}
