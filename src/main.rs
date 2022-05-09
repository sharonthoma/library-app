
#[derive(Debug, Clone, Copy)]
struct Book{
    id: u8,
    author: &'static str,
    title: &'static str,
    available_books: &'static str,
}
#[derive(Debug, Clone, Copy)]
pub struct User{
     id: u8,
     username: &'static str,
     fullname: &'static str,
     address: &'static str,
}
#[derive(Debug, Clone, Copy)]
struct Borrower{
     user_id: u8,
     book_id: u8,
     return_book:bool, 
 }
 #[derive(Debug, Clone)]
struct Library{
    books: Vec<Book>,
    users: Vec<User>,
    borrowers: Vec<Borrower>,
}

impl Library{
    pub fn add_book(&mut self, author: &'static str, title: &'static str, available_books: &'static str){
        let len_: usize = self.books.len();
        let id_= len_ as u8;
        let book: Book= Book { id: id_, author, title, available_books };
        self.books.push(book)
    }
    
    pub fn add_user (&mut self, username: &'static str, fullname: &'static str , address:&'static str ){
      let len_: usize = self.users.len();
      let id_= len_ as u8;
      let user = User{id: id_, username, fullname, address};
      self.users.push(user);
    }

    pub fn delete_book(&mut self, id: u8 ){
        let books = &mut self.books;

        let index = books.iter().position(|x| x.id == id).unwrap();
        books.remove(index);
    }
    
    pub fn get_my_books(&mut self) ->Vec<Option<&'static Book>> {
        // let user: String = String::from("sharon.testnet");
        let borrowers = &mut self.borrowers.clone();
        let me = &mut self.clone();
        let mut books_to_return = Vec::new();
        for n in 0..borrowers.len() {
          let borrower = borrowers.get(n);
          
          match borrower {
            Some(x) => {
              if x.user_id == 1 {
                // let user = self.get_user(x.user_id);
                let book = me.get_book(x.book_id);
                books_to_return.push(book);
              }
            }
            None => {
            }}
          }

          return books_to_return;
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
    
}


fn main() {
    let mut library = Library{ books: Vec::new(), users: Vec::new(), borrowers: Vec::new()  };
    library.add_book("sharon", "principle of physics", "1000");
    library.add_book("Thomas", "complex analysis", "900");
    library.delete_book(1);
    println!("{:#?}", library.books);
}
