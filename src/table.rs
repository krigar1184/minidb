use std::mem::align_of;

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

#[derive(Debug)]
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

    fn serialize(&self, dst: *mut T) {
        unsafe {
            let offset = dst.align_offset(align_of::<T>());
            std::ptr::copy_nonoverlapping(
                &self.value,
                dst.add(offset).cast::<T>(),
                1,
            );
        }
    }

}

impl Attribute<u64> {
    fn deserialize(src: *const u8) -> Self {
        unsafe {
            let dst: *mut u8 = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(src, dst, Self::size_of());
            let value: u64 = std::mem::transmute(dst);

            Self::new(value)
        }
    }
}


impl Attribute<String> {
    fn deserialize(src: *const u8) -> Self {
        unsafe {
            let dst: *mut u8 = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(src, dst, Self::size_of());
            let value: String = String::from_raw_parts(dst, 1000, 1000);

            Self::new(value)
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
            dbg!(ID_OFFSET, USERNAME_OFFSET, EMAIL_OFFSET);
            self.id.serialize(dst.add(ID_OFFSET).cast::<u64>());
            self.username.serialize(dst.add(USERNAME_OFFSET).cast::<&str>());
            self.email.serialize(dst.add(EMAIL_OFFSET).cast::<&str>());
        }
        Ok(())
    }

    fn deserialize(&mut self, src: *const u8) {
        unsafe {
            let id = Attribute::<u64>::deserialize(src.add(ID_OFFSET));
            let username = Attribute::<String>::deserialize(src.add(USERNAME_OFFSET));
            let email = Attribute::<String>::deserialize(src.add(EMAIL_OFFSET));
        }
    }
}

#[derive(Debug)]
struct Table {
    num_rows: usize,
    pages: Vec<Page>,
}

impl Table {
    fn row_slot(&mut self, row_num: usize) -> usize {
        let pages = &mut self.pages;
        let page_num: usize = row_num / ROWS_PER_PAGE;
        let mut page = pages.get(page_num);

        if page.is_none() {
            pages.insert(page_num, Page {});
            page = pages.get(page_num);
        }

        let mut page = page.unwrap();
        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        byte_offset
    }
}

#[test]
fn test_serialize() {
    unsafe {
        let mut t = Table{num_rows: 0, pages: vec![]};
        let r = Row::new(666, "username", "email");
        let p = std::ptr::addr_of_mut!(t).add(t.row_slot(0)).cast::<u8>();
        r.serialize(p).unwrap();

        dbg!(p, p.cast::<u64>().read());
        assert_eq!(p.cast::<u64>().read(), r.id.value);

        let p2 = p.add(ID_SIZE).cast::<&str>();
        assert_eq!(p2.read(), r.username.value);

        let p3 = p.add(ID_SIZE + USERNAME_SIZE).cast::<&str>();
        p3.read();
    }
}
