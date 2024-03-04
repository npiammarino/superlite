#[derive(Debug)]
pub enum TableError {
    TableIndexError,
}
use crate::TableError::*;

const PAGE_SIZE: usize = 10;

#[derive(Debug, Clone)]
pub struct Row {
    id: usize,
    username: String,
    email: String, // could make special type for this
}

impl Row {
    pub fn build(id: usize, username: String, email: String) -> Row {
        Row {
            id,
            username,
            email,
        }
    }
}

#[derive(Debug)]
pub struct Page {
    // need to serialize eventually
    max_size: usize,
    rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Table {
    num_rows: usize,
    pages: Vec<Page>, // need to serialize eventually
}

impl Table {
    pub fn new() -> Table {
        Table {
            num_rows: 0,
            pages: Vec::new(),
        }
    }

    pub fn push_row(&mut self, row: Row) {
        if self.num_rows % PAGE_SIZE == 0 {
            self.pages.push(Page {
                max_size: PAGE_SIZE,
                rows: vec![],
            });
        }

        let page_index = self.num_rows / PAGE_SIZE;
        self.pages[page_index].rows.push(row);
        self.num_rows += 1;
    }

    pub fn get_row(&self, row_number: usize) -> Result<Row, TableError> {
        if row_number + 1 > self.num_rows {
            return Err(TableIndexError);
        }

        let (page_index, row_index) = (row_number / PAGE_SIZE, row_number % PAGE_SIZE);
        Ok(self.pages[page_index].rows[row_index].clone())
    }
}
