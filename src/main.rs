use std::clone;

fn main() {
    let mut book=Book{
        title : "Fatih Harbiye".to_string(),
        author : "Peyami".to_string(),
        page_count : 5,
    };

    let cloned_book = book.clone();

    let magazine = Magazine{
        title : "GQ".to_string(),
        issue : 15,
        topic : "education".to_string(),
    };
    
    let publications = vec![Publication::Book(book), Publication::Magazine(magazine)];
    print_publications(publications);
}
enum Publication {
    Book(Book),
    Magazine(Magazine),
}

#[derive(Debug , Clone)]
struct Book {
    title: String,
    author: String,
    page_count: u32,
}
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}
fn print_publications(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(ref book) => {
                println!(
                    "Book: {} author: {}, {} pages",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(ref magazine) => {
                println!(
                    "Magazine: {} - Issue: {}, Topic: {}",
                    magazine.title, magazine.issue, magazine.topic
                );
            }
        }
    }
}
