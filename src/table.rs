const ID_SIZE: usize = Column::<u32>::size_of();
const ID_OFFSET: usize = 0;

const USERNAME_SIZE: usize = Column::<&str>::size_of();
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;

const EMAIL_SIZE: usize = Column::<&str>::size_of();
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * 100;

#[derive(Debug)]
struct Column<T> {
    value: T,
}

impl<T> Column<T> {
    fn new(value: T) -> Self {
        Column::<T> { value }
    }

    pub const fn size_of() -> usize {
        std::mem::size_of::<T>()
    }

    fn size(&self) -> usize {
        Self::size_of()
    }
}

#[derive(Debug)]
struct Page {
}

#[derive(Debug)]
pub(crate) struct Row<'a> {
    id: Column<u64>,
    username: Column<&'a str>,
    email: Column<&'a str>,
}

impl<'a> Row<'a> {
    pub fn new(id: u64, username: &'a str, email: &'a str) -> Self {
        let id_col = Column{value: id};
        let username_col = Column{value: username};
        let email_col = Column{value: email};
        Row{
            id: id_col,
            username: username_col,
            email: email_col,
        }
    }
}

#[derive(Debug)]
struct Table {
    num_rows: usize,
    pages: Vec<Page>,
}

impl Table {
    fn row_slot(&mut self, row_num: usize) {
        let pages = &mut self.pages;
        let page_num: usize = row_num / ROWS_PER_PAGE;
        let mut page = pages.get(page_num);

        if page.is_none() {
            pages.insert(page_num, Page {});
            page = pages.get(page_num);
        }

        let page = page.unwrap();
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
    }
}
