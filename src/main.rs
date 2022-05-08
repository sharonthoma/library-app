use std::{string, vec};
use near_sdk::{near_bindgen, env};
#[derive(Debug)]
struct Books{
    id: u8,
    author: String,
    title: String,
    available_books: String,
}
pub struct User{
     id: u8,
     sername:String,
     fullname:String,
     address:String,
}
struct Borrower{
     User_id: u8,
     book_id: u8,
     //return_book:"True",
     
     
 }
struct Library{
    books: Vec<Books>,
    users: Vec<User>,
    borrowers: Vec<Borrower>,
}
//impl Borrower{
  //pub fn Borrow_books(&mut self,User_id: u8, book_id: u8 ){
    
  //}
//}

impl Library{
    pub fn add_book(&mut self, author: String, title: String, available_books: String){
        let len_: usize = self.books.len();
        let id_= len_ as u8;
        let book: Books= Books { id: id_, author: author, title: title, available_books: available_books };
        self.books.push(book)
    }
    pub fn delete_book(&mut self, id: u8 ){
        let books = &mut self.books;

        let index = books.iter().position(|x| x.id == id).unwrap();
        books.remove(index);
    }
    pub fn get_my_books(&mut self) -> Vec<&Books> {
        let account_id = env::signer_account_id();
        let user: String = String::from(account_id);
        let mut books_to_return: Vec<&Books> = Vec::new();
        for n in 0..self.books.len() {
          let book_ = self.books.get(n);
          match book_ {
            Some(x) => {
              if x.id == user {
                books_to_return.push(x);
              }
            }
            None => {
            }}
      }
    }
    //pub fn update_my_books =>(&mut self, id: u8){
       // let books = &mut self.books;
    
          
        }
    
      //return books_to_return;
     // }
}


fn main() {
    let mut books = Books { books: Vec::new() };
    books.add_book("sharon".to_string(), "principle of physics".to_string(), "1000".to_string());
    books.add_book("Thomas".to_string(), "complex analysis".to_string(), "900".to_string());
    books.delete_book(1);
    println!("{:#?}", books.books);
}
