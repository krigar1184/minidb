const ID_SIZE: usize = Attribute::<u64>::size_of();
const ID_OFFSET: usize = 0;

const USERNAME_SIZE: usize = Attribute::<&str>::size_of();
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;

const EMAIL_SIZE: usize = Attribute::<&str>::size_of();
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * 100;

struct SerializationError {
}

#[derive(Debug)]
struct Attribute<T> {
    value: T,
}

impl<T> Attribute<T> {
    fn new(value: T) -> Self {
        Attribute::<T> { value }
    }

    pub const fn size_of() -> usize {
        std::mem::size_of::<T>()
    }

    fn size(&self) -> usize {
        Self::size_of()
    }
}

impl Attribute<u64> {
    fn serialize(&self, dst: *mut u8) {
        unsafe {
            std::ptr::copy_nonoverlapping(self.value.to_ne_bytes().as_ptr(), dst.add(ID_OFFSET), self.size());
        }
    }

    fn deserialize(&mut self, src: *const u8) {
        let dst: *mut u8 = self.value.to_ne_bytes().as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, self.size());
        }
    }
}


impl Attribute<&str> {
    fn serialize(&self, dst: *mut u8) {
        unsafe {
            std::ptr::copy_nonoverlapping(self.value.as_ptr(), dst.add(ID_OFFSET), self.size());
        }
    }

    fn deserialize(&mut self, src: *const u8) {
        let dst: *mut u8 = self.value.as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, self.size());
        }
    }
}

#[derive(Debug)]
struct Page {
}

#[derive(Debug)]
pub(crate) struct Row<'a> {
    id: Attribute<u64>,
    username: Attribute<&'a str>,
    email: Attribute<&'a str>,
    offset: usize,
}

impl<'a> Row<'a> {
    pub fn new(id: u64, username: &'a str, email: &'a str) -> Self {
        let id_col = Attribute::new(id);
        let username_col = Attribute::new(username);
        let email_col = Attribute::new(email);
        Row{
            id: id_col,
            username: username_col,
            email: email_col,
            offset: 0,
        }
    }

    fn serialize(&self, dst: *mut u8) -> std::result::Result<(), SerializationError> {
        unsafe {
            self.id.serialize(dst.offset(ID_OFFSET as isize));
            self.username.serialize(dst.offset(USERNAME_OFFSET as isize));
            self.email.serialize(dst.offset(EMAIL_OFFSET as isize));
        }
        Ok(())
    }

    fn deserialize(&mut self, src: *const u8) {
        unsafe {
            self.id.deserialize(src.add(ID_OFFSET));
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
