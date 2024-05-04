const HEADER_OFFSET: usize = 100;

const HEADER_STRING: &str = "SQLite format 3\0";
const PAGE_SIZE: u16 = 4096;
const RESERVED: u8 = 0;
const NUMBER_OF_PAGES: u32 = std::u32::MAX - 1;

type Page = Vec<u8>;
type Cell = Vec<u8>;
// include enum with header sections with size and offset? https://www.sqlite.org/fileformat2.html
// found only on page one of the database

fn build_db_header() -> Vec<u8> {
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

fn build_page_header(page_one: bool, page_type: u8, interior: bool) -> Vec<u8> {
    let mut header: Vec<u8> = if page_one { build_db_header() } else { vec![] };
    // values are big-endian

    // The page type -- u8
    // A value of 2 (0x02) means the page is an interior index b-tree page.
    // A value of 5 (0x05) means the page is an interior table b-tree page.
    // A value of 10 (0x0a) means the page is a leaf index b-tree page.
    // A value of 13 (0x0d) means the page is a leaf table b-tree page.
    header.extend_from_slice(&[page_type]);

    // The start of the first freeblock of the page (0 if full) -- u16
    // how to init?
    header.extend_from_slice(&[0, 1]);

    // The number of cells on the page -- u16
    // should always be 1 unless rootpage of table containing no rows (so init at 0?)
    header.extend_from_slice(&[0, 0]);

    // The start of cell content area (0 is 65536) -- u16
    // cells grow from bottom up?  Only root page of table with no rows can have 0 cells (so init at 0?)
    header.extend_from_slice(&[0, 0]);

    // The number of fragmented free bytes in the cell content area (leftovers) -- u8
    // this value should never exceed 60
    header.extend_from_slice(&[0, 0]);

    // The rightmost pointer (for interior b tree pages only) -- u32
    // how to init?
    if interior {
        // do the thing
    }

    header
}

fn build_page(page_type: u8) -> Page {}

fn build_tl_cell(rowid: i64, payload: Vec<u8>) -> Cell {
    let mut cell: Vec<u8> = vec![];

    cell.extend_from_slice(&payload.len().to_be_bytes());
    cell.extend_from_slice(&rowid.to_be_bytes());
    cell.extend_from_slice(payload.as_slice());

    // add overflow page number if needed

    cell
}

fn build_ti_cell(rowid: i64, left_child: u32) -> Cell {
    let mut cell: Vec<u8> = vec![];

    cell.extend_from_slice(&left_child.to_be_bytes);
    cell.extend_from_slice(&rowid.to_be_bytes());

    cell
}

fn build_il_cell(rowid: i64, payload: Vec<u8>) -> Cell {
    let mut cell: Vec<u8> = vec![];

    cell.extend_from_slice(&payload.len().to_be_bytes);
    cell.extend_from_slice(payload.as_slice);

    cell
}

fn build_ii_cell(payload: Vec<u8>, left_child: u32) -> Cell {
    let mut cell: Vec<u8> = vec![];

    cell.extend_from_slice(&left_child.to_be_bytes());
    cell.extend_from_slice(&payload.len().to_be_bytes());
    cell.extend_from_slice(payload.as_slice);

    // add overflow page number if needed
}

fn build_cell(
    page_type: u8,
    payload: Vec<u8>,
    rowid: Option<i64>,
    left_child: Option<u32>,
) -> Vec<u8> {
    let mut cell: Vec<u8> = vec![];

    match page_type {
        // Table leaf
        0x02 => {
            // returns usize but expressed as varint in docs, is this accurate?
            cell.extend_from_slice(&payload.len().to_be_bytes());
            cell.extend_from_slice(&rowid.expect("requires row id").to_be_bytes());
            cell.extend_from_slice(payload.as_slice());
            // overflow page (need some calculation here when structs are built)
        }
        // Table interior
        0x05 => {}
        // Index leaf
        0x0a => {}
        // Index interior
        0x0d => {}
    }

    cell
}

fn main() {
    let header = build_db_header();
    println!("Header length: {}", header.len());
    println!("Header bytes: {:02X?}", header);
}
