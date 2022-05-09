#[derive(Debug, Clone, Copy)]
struct Book {
    id: u8,
    author: &'static str,
    title: &'static str,
    available_books: &'static str,
}
#[derive(Debug, Clone, Copy)]
pub struct User {
    id: u8,
    username: &'static str,
    fullname: &'static str,
    address: &'static str,
}
#[derive(Debug, Clone, Copy)]
struct Borrower {
    id: u8,
    user_id: u8,
    book_id: u8,
    book_returned: bool,
}
#[derive(Debug, Clone)]
struct Library {
    books: Vec<Book>,
    users: Vec<User>,
    borrowers: Vec<Borrower>,
}

impl Library {
    pub fn add_book(
        &mut self,
        author: &'static str,
        title: &'static str,
        available_books: &'static str,
    ) {
        let len_: usize = self.books.len();
        let id_ = len_ as u8;
        let book: Book = Book {
            id: id_,
            author,
            title,
            available_books,
        };
        self.books.push(book)
    }

    pub fn add_user(
        &mut self,
        username: &'static str,
        fullname: &'static str,
        address: &'static str,
    ) {
        let len_: usize = self.users.len();
        let id_ = len_ as u8;
        let user = User {
            id: id_,
            username,
            fullname,
            address,
        };
        self.users.push(user);
    }

    pub fn borrow_book(&mut self, user_id: u8, book_id: u8) {
        // Check if the book and the user are available before borrowing
        let book = self.books.iter().find(|book| book.id == book_id);
        let user = self.users.iter().find(|user| user.id == user_id);
        if book.is_none() || user.is_none() {
            println!("Book or user not found");
            return; // Return if book or user not found
        } else {
            let book = book.unwrap();
            let user = user.unwrap();
            let id_ = self.borrowers.len() as u8;
            let borrow = Borrower {
                id: id_,
                user_id: user.id,
                book_id: book.id,
                book_returned: false,
            };
            self.borrowers.push(borrow);
        }
    }

    pub fn get_borrowed_books(&mut self, user_id: u8) -> Vec<Option<Book>> {
        let mut borrowed_books = Vec::new();
        for borrower in self.borrowers.clone().iter() {
            if borrower.user_id == user_id {
                let book = self
                    .books
                    .clone()
                    .into_iter()
                    .find(|book_| book_.id == borrower.book_id);
                borrowed_books.push(book);
            }
        }
        borrowed_books
    }

    pub fn delete_book(&mut self, id: u8) {
        let books = &mut self.books;

        let index = books.iter().position(|x| x.id == id).unwrap();
        books.remove(index);
    }

    pub fn get_book(&mut self, book_id: u8) -> Option<&Book> {
        let me: &Vec<Book> = &self.books;
        let book = me.into_iter().find(|book_| book_.id == book_id);
        return book;
    }

    pub fn get_user(&mut self, user_id: u8) -> Option<&User> {
        let me: &Vec<User> = &self.users;
        let user = me.into_iter().find(|user_| user_.id == user_id);
        return user;
    }

    // Delete a user
    pub fn delete_user(&mut self, id: u8) {
        let users = &mut self.users;
        let index = users.iter().position(|x| x.id == id).unwrap();
        users.remove(index);
    }

    pub fn delete_borrower(&mut self, id: u8) {
        let borrowers = &mut self.borrowers;
        let index = borrowers.iter().position(|x| x.id == id).unwrap();
        borrowers.remove(index);
    }

    // Return a book
    pub fn return_book(&mut self, id: u8, user_id: u8) {
        let borrowers = &mut self.borrowers;
        let index = borrowers
            .iter()
            .position(|x| x.id == id && x.user_id == user_id)
            .unwrap();
        borrowers[index].book_returned = true;
    }
}

fn main() {
    let mut library = Library {
        books: Vec::new(),
        users: Vec::new(),
        borrowers: Vec::new(),
    };
    library.add_book("sharon", "principle of physics", "1000");
    library.add_book("Thomas", "complex analysis", "900");
    println!("Books - {:#?}", library.books);
    library.add_user("sharon", "sharon", "sharon");
    println!("Users - {:#?}", library.users);
    library.borrow_book(0, 1);
    println!("Borrowers - {:#?}", library.borrowers);
    println!(
        "User 1 borrowed books - {:#?}",
        library.get_borrowed_books(0)
    );

    library.return_book(0, 0);

    println!("Borrowers - {:#?}", library.borrowers);
}
