const ID_SIZE: usize = Attribute::<u32>::size_of();
const USERNAME_SIZE: usize = Attribute::<&str>::size_of();
const EMAIL_SIZE: usize = Attribute::<&str>::size_of();
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * 100;

#[derive(Debug)]
struct Page {
}

#[derive(Debug)]
pub(crate) struct Row {
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

#[derive(Debug)]
pub struct Attribute<T> {
    pub value: T
}

impl<T> Attribute<T> {
    fn new(value: T) -> Attribute<T> {
        Attribute::<T> { value }
    }

    fn size(&self) -> usize {
        Self::size_of()
    }

    pub const fn size_of() -> usize {
        std::mem::size_of::<T>()
    }
}

