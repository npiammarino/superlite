const HEADER_OFFSET: usize = 100;

const HEADER_STRING: &str = "SQLite format 3\0";
const PAGE_SIZE: u16 = 4096;
const RESERVED: u8 = 0;
const NUMBER_OF_PAGES: u32 = std::u32::MAX - 1;

// include enum with header sections with size and offset? https://www.sqlite.org/fileformat2.html

fn build_header() -> Vec<u8> {
    let mut header: Vec<u8> = vec![];

    // magic string
    header.extend_from_slice(HEADER_STRING.as_bytes());
    // pagesize
    header.extend_from_slice(&PAGE_SIZE.to_be_bytes());
    // write version
    header.push(1);
    // read version
    header.push(1);
    // reserved space (end of each page)
    header.push(RESERVED);
    // maximum embedded payload fraction
    header.push(64);
    // minimum embedded payload fraction
    header.push(32);
    // leaf payload fraction
    header.push(32);
    // file change counter (inits to 0)
    header.extend_from_slice(&[0; 4]);
    // size of database file in pages (in header size)
    header.extend_from_slice(&[0, 0, 0, 100]);
    // first free page number
    header.extend_from_slice(&[0, 0, 0, 1]);
    // total number free pages
    header.extend_from_slice(&NUMBER_OF_PAGES.to_be_bytes());
    // schema cookie
    header.extend_from_slice(&[0; 4]);
    // schema format number
    header.extend_from_slice(&[0, 0, 0, 1]);
    // default page cache size (suggested, not sure if using this, set to 0 for now?)
    header.extend_from_slice(&[0; 4]);
    // auto-vacuum
    header.extend_from_slice(&[0; 4]);
    // text encoding (1 for utf8)
    header.extend_from_slice(&[0, 0, 0, 1]);
    // user verssion (not used by sqlLite)
    header.extend_from_slice(&[0; 4]);
    // incremental-vacuum
    header.extend_from_slice(&[0; 4]);
    // application format id (0 for plain sqlLite)
    header.extend_from_slice(&[0; 4]);
    //reserved for expansion
    header.extend_from_slice(&[0; 20]);
    // version valid for number (dont know what to do with this yet)
    header.extend_from_slice(&[0; 4]);
    // sqlLite version number (fill in later)
    header.extend_from_slice(&[0; 4]);

    header
}

fn main() {
    let header = build_header();
    println!("Header length: {}", header.len());
    println!("Header bytes: {:02X?}", header);
}
