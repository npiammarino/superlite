use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom},
    mem, str,
};

#[derive(Debug)]
pub enum TableError {
    TableIndexError,
    TableFull,
    ByteOverflow,
}
use crate::TableError::*;

pub const USERNAME_MAX: usize = 32;
pub const EMAIL_MAX: usize = 255;

type ColumnId = u32;
type ColumnUsername = [u8; USERNAME_MAX];
type ColumnEmail = [u8; EMAIL_MAX];

const ID_SIZE: usize = mem::size_of::<ColumnId>();
const USERNAME_SIZE: usize = mem::size_of::<ColumnUsername>();
const EMAIL_SIZE: usize = mem::size_of::<ColumnEmail>();
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Debug, Clone, Copy)]
pub struct RowBytes {
    id: ColumnId,
    username: ColumnUsername,
    email: ColumnEmail,
}
#[derive(Debug, Clone)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {
    pub fn build(id: u32, username: String, email: String) -> Row {
        Row {
            id,
            username,
            email,
        }
    }

    fn serialize(self) -> Result<RowBytes, TableError> {
        let mut username = [b'\0'; USERNAME_MAX];
        let chars = self.username.as_bytes();
        if chars.len() > USERNAME_MAX {
            return Err(TableError::ByteOverflow);
        }
        for i in 0..chars.len() {
            username[i] = chars[i];
        }

        let mut email = [b'\0'; EMAIL_MAX];
        let chars = self.email.as_bytes();
        if chars.len() > EMAIL_MAX {
            return Err(TableError::ByteOverflow);
        }
        for i in 0..chars.len() {
            email[i] = chars[i];
        }

        Ok(RowBytes {
            id: self.id,
            username,
            email,
        })
    }
}

impl RowBytes {
    fn deserialize(self) -> Row {
        let username = str::from_utf8(&self.username)
            .expect("expect username")
            .trim_matches('\0');
        let email = str::from_utf8(&self.email)
            .expect("expect email")
            .trim_matches('\0');
        Row {
            id: self.id,
            username: String::from(username),
            email: String::from(email),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    rows: [Option<RowBytes>; ROWS_PER_PAGE],
    bytes: Option<[u8; PAGE_SIZE]>,
}

impl Page {
    fn new() -> Page {
        Page {
            rows: [None; ROWS_PER_PAGE], // move this to heap?
            bytes: None,
        }
    }

    fn from_bytes(bytes: [u8; PAGE_SIZE]) -> Page {
        Page {
            rows: [None; ROWS_PER_PAGE],
            bytes: Some(bytes),
        }
    }
}

#[derive(Debug)]
struct Pager {
    file: File,
    pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Pager {
    fn new(filename: String) -> io::Result<Pager> {
        let f = OpenOptions::new().read(true).write(true).open(filename)?;
        let pages = [None; TABLE_MAX_PAGES]; // note: should move to heap

        Ok(Pager { file: f, pages })
    }

    fn get_page(&mut self, page_index: usize) -> io::Result<Page> {
        match self.pages[page_index] {
            Some(page) => Ok(page),
            None => {
                let mut buff = [0u8; PAGE_SIZE];
                self.file
                    .seek(SeekFrom::Start((page_index * PAGE_SIZE) as u64))?;
                self.file.read_exact(&mut buff)?;

                let new_page = Page::from_bytes(buff);
                self.pages[page_index] = Some(new_page);
                Ok(new_page)
            }
        }
    }
}

#[derive(Debug)]
pub struct Table {
    num_rows: usize,
    // pages: [Option<Page>; TABLE_MAX_PAGES],
    pager: Pager,
}

impl Table {
    pub fn new(filename: String) -> Table {
        Table {
            num_rows: 0,
            // pages: [None; TABLE_MAX_PAGES],
            pager: Pager::new(filename).expect("hopefully this file exists..."),
        }
    }

    fn row_slot(&mut self, row_num: usize) -> (usize, usize) {
        let page_index = (row_num - 1) / ROWS_PER_PAGE;
        let row_index = (row_num - 1) % ROWS_PER_PAGE;
        if self.pager.pages[page_index].is_none() {
            self.pager.pages[page_index].replace(Page::new());
        }

        (page_index, row_index)
    }

    pub fn push_row(&mut self, row: Row) -> Result<(), TableError> {
        if self.num_rows == TABLE_MAX_ROWS {
            return Err(TableFull);
        }

        let (page_index, row_index) = self.row_slot(self.num_rows + 1);
        match row.serialize() {
            Ok(new_row) => {
                let mut page = self.pager.pages[page_index].expect("Created in slot function");
                page.rows[row_index].replace(new_row);

                self.pager.pages[page_index].replace(page);
                self.num_rows += 1;

                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_row(&mut self, row_number: usize) -> Result<Row, TableError> {
        if row_number > self.num_rows {
            return Err(TableIndexError);
        }

        let (page_index, row_index) = self.row_slot(row_number);
        let row_bytes =
            self.pager.pages[page_index].expect("expect page").rows[row_index].expect("expect row");

        Ok(row_bytes.deserialize())
    }
}
