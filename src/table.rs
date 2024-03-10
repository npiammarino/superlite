use std::{
    convert::TryInto,
    fs::{File, OpenOptions},
    io::{self, Error, Read, Seek, SeekFrom, Write},
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

type ColumnId = [u8; 4];
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

type PageBytes = [u8; PAGE_SIZE];

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
            id: self.id.to_ne_bytes(),
            username,
            email,
        })
    }
}

impl RowBytes {
    fn new(bytes: [u8; ROW_SIZE]) -> RowBytes {
        let id: ColumnId = bytes[0..USERNAME_OFFSET].try_into().expect("id from bytes");
        let username: ColumnUsername = bytes[USERNAME_OFFSET..EMAIL_OFFSET]
            .try_into()
            .expect("username from bytes");
        let email: ColumnEmail = bytes[EMAIL_OFFSET..ROW_SIZE]
            .try_into()
            .expect("email from bytes");
        RowBytes {
            id,
            username,
            email,
        }
    }

    fn deserialize(self) -> Row {
        let username = str::from_utf8(&self.username)
            .expect("expect username")
            .trim_matches('\0');
        let email = str::from_utf8(&self.email)
            .expect("expect email")
            .trim_matches('\0');
        Row {
            id: u32::from_ne_bytes(self.id),
            username: String::from(username),
            email: String::from(email),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    rows: [Option<RowBytes>; ROWS_PER_PAGE],
    bytes: Option<PageBytes>,
}

impl Page {
    fn new() -> Page {
        Page {
            rows: [None; ROWS_PER_PAGE], // move this to heap?
            bytes: None,
        }
    }

    fn from_bytes(bytes: PageBytes) -> Page {
        Page {
            rows: [None; ROWS_PER_PAGE],
            bytes: Some(bytes),
        }
    }

    fn serialize(&self) -> PageBytes {
        let mut bytes = [0u8; PAGE_SIZE];
        for (i, row) in self.rows.iter().enumerate() {
            let mut cursor = i * ROW_SIZE;
            if let Some(row_bytes) = row {
                for byte in row_bytes.id {
                    bytes[cursor] = byte;
                    cursor += 1;
                }
                cursor = i * ROW_SIZE + USERNAME_OFFSET;
                for byte in row_bytes.username {
                    bytes[cursor] = byte;
                    cursor += 1;
                }
                cursor = i * ROW_SIZE + EMAIL_OFFSET;
                for byte in row_bytes.email {
                    bytes[cursor] = byte;
                    cursor += 1;
                }
            }
        }

        bytes
    }
}

#[derive(Debug)]
struct Pager {
    file: File,
    pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Pager {
    fn new(filename: &String) -> io::Result<Pager> {
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

    fn get_row(&mut self, page_index: usize, row_index: usize) -> io::Result<Row> {
        let page = self.get_page(page_index)?;
        let row_bytes = match page.rows[row_index] {
            Some(row) => row,
            None => {
                let mut buff = [0u8; ROW_SIZE];
                self.file
                    .seek(SeekFrom::Start((row_index * ROW_SIZE) as u64))?;
                self.file.read_exact(&mut buff)?;
                RowBytes::new(buff)
            }
        };
        Ok(row_bytes.deserialize())
    }

    fn flush(&mut self, page_num: usize) -> io::Result<()> {
        match self.pages[page_num] {
            None => {}
            Some(mut page) => {
                self.file
                    .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
                match page.bytes {
                    Some(bytes) => {
                        self.file.write(&bytes)?;
                    }
                    None => {
                        let bytes = page.serialize();
                        page.bytes = Some(bytes);
                        self.file.write(&bytes)?;
                    }
                }
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct Table {
    num_rows: usize,
    filename: String,
    pager: Pager,
}

impl Table {
    pub fn open(filename: String) -> Table {
        Table {
            num_rows: 3,
            pager: Pager::new(&filename).expect("hopefully this file exists..."),
            filename,
        }
    }

    pub fn close(&mut self) -> io::Result<()> {
        for i in 0..self.pager.pages.len() {
            self.pager.flush(i)?;
        }
        Ok(())
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

        Ok(self
            .pager
            .get_row(page_index, row_index)
            .expect("expect row"))
    }
}
