use std::{fs::OpenOptions, mem, str};

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
}

impl Page {
    fn new() -> Page {
        Page {
            rows: [None; ROWS_PER_PAGE],
        }
    }
}

// struct Pager {
//     file: OpenOptions,
//     pages: [Option<Page>; TABLE_MAX_PAGES],
// }

// impl Pager {
//     fn open(filename: String) -> Result<Pager, ()> {
//         let f = OpenOptions::new().read(true).write(true).open(filename)?;
//         let pages = [None; TABLE_MAX_PAGES];

//         Pager { file: f, pages }
//     }
// }

#[derive(Debug)]
pub struct Table {
    num_rows: usize,
    pages: [Option<Page>; TABLE_MAX_PAGES],
    // pager: Pager,
}

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: [None; TABLE_MAX_PAGES],
        }
    }

    fn row_slot(&mut self, row_num: usize) -> (usize, usize) {
        let page_index = (row_num - 1) / ROWS_PER_PAGE;
        let row_index = (row_num - 1) % ROWS_PER_PAGE;
        if self.pages[page_index].is_none() {
            self.pages[page_index].replace(Page::new());
        }

        (page_index, row_index)
    }
    // pub fn open() -> Table {
    //     let filename = String::new();
    //     let pager = Pager::open(filename);

    //     Table { num_rows: 0, pager }
    // }

    pub fn push_row(&mut self, row: Row) -> Result<(), TableError> {
        if self.num_rows == TABLE_MAX_ROWS {
            return Err(TableFull);
        }

        let (page_index, row_index) = self.row_slot(self.num_rows + 1);
        match row.serialize() {
            Ok(new_row) => {
                let mut page = self.pages[page_index].expect("Created in slot function");
                page.rows[row_index].replace(new_row);

                self.pages[page_index].replace(page);
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
            self.pages[page_index].expect("expect page").rows[row_index].expect("expect row");

        Ok(row_bytes.deserialize())
    }
}
