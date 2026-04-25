// serde converts Rust data types (structs, enums, etc.) into a Json and other string formats.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

const FILE_PATH: &str = "library.json"; // defines where the library data will be stored.

#[derive(Debug, Serialize, Deserialize, Clone)] // This command tells the compiler to generate code to convert the struct to and from JSON.

// Struct of the book.
struct Book {
    title: String,
    author: String,
    year: Option<u32>,     // Can replace with Option<u32> if you want the year to be optional.
    state: String, // State of the book (Read, Reading, Unread or Want to read).
}

#[derive(Debug, Serialize, Deserialize)]

// Struct of the library, which contains a vector of books.
struct Library {
    books: Vec<Book>,
}
// Implementation of the library struct, which includes a method to create a new library.
impl Library {
    fn new() -> Self {
        Library { books: Vec::new() } // Initializes an empty library.
    }

    // function to add a book to the library.

    fn add_book(&mut self, title: String, author: String, year: Option<u32>, state: String) {
        let book = Book {
            title,
            author,
            year,
            state,
        };
        self.books.push(book); // Adds the new book to the library's vector of books.
        println!("Book added successfully!");
    }

    // function to delete a book from the library by its index number.

    fn delete_book(&mut self, index: usize) -> Result<(), String> {
        if index == 0 || index > self.books.len() {
            return Err("Invalid book number".to_string()); // if index number is out of range, return an error message.
        }
        let removed = self.books.remove(index - 1); // removes the book at the specified index.
        println!(
            "Book '{}' by {} removed successfully!",
            removed.title, removed.author
        ); // prints a success message with the title and author of the removed book.
        Ok(())
    }

    // shows all the books in the library.

    fn show(&self) {
        if self.books.is_empty() {
            println!(" Your library is empty!");
            return;
        }
        println!("Your Library:");
        for (i, book) in self.books.iter().enumerate() {
            let year_str = match book.year {
                Some(y) => format!("({y})"),
                None => String::new(),
            };
            println!("  {}. {} by {}{}", i + 1, book.title, book.author, year_str);
        }
    }

    // saves the library data to a json file.

    fn save_to_file(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize library");
        fs::write(FILE_PATH, json)?;
        Ok(())
    }

    // loads the library data from a json file.

    fn load_from_file() -> Self {
        match fs::read_to_string(FILE_PATH) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(lib) => {
                    println!("Library loaded successfully!");
                    lib
                }
                Err(_) => {
                    println!("Failed to parse library data. Starting with an empty library.");
                    Library::new()
                }
            },
            Err(_) => {
                println!("No existing library found. Starting with an empty library.");
                Library::new()
            }
        }
    }
}

// clears terminal.

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

// menu interface

fn print_menu() {
    println!("\n--- Personal Library ---");
    println!("1. Add a book");
    println!("2. Delete a book");
    println!("3. Show all books");
    println!("4. Save & Quit");
    print!("Choose an option: ");
    let _ = io::stdout().flush();
}

// handles input from the user.

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // Loads library.

    let mut library = Library::load_from_file();
    loop {
        clear_screen();
        print_menu();
        let choice = read_line();

        match choice.as_str() {
            "1" => {
                // Add a book
                println!("\n--- Add a New Book ---");
                print!("Title: ");
                let _ = io::stdout().flush();
                let title = read_line();

                print!("Author: ");
                let _ = io::stdout().flush();
                let author = read_line();

                print!("State (Read, Reading, Unread, Want to read): ");
                let _ = io::stdout().flush();
                let state = read_line();

                print!("Year (optional, press Enter to skip): ");
                let _ = io::stdout().flush();
                let year_str = read_line();
                let year: Option<u32> = if year_str.is_empty() {
                    None
                } else {
                    year_str.parse().ok()
                };

                library.add_book(title, author, year, state);
                // Automatically save after modification
                if let Err(e) = library.save_to_file() {
                    eprintln!("❌ Could not save library: {e}");
                }
                println!("\nPress Enter to continue...");
                read_line();
            }
            "2" => {
                // Delete a book
                if library.books.is_empty() {
                    println!("\nYour library is empty. Nothing to delete.");
                    println!("\nPress Enter to continue...");
                    read_line();
                    continue;
                }

                library.show();
                print!("\nEnter the number of the book to delete: ");
                let _ = io::stdout().flush();
                let input = read_line();

                match input.parse::<usize>() {
                    Ok(num) => {
                        if let Err(err) = library.delete_book(num) {
                            println!("❌ {err}");
                        } else {
                            // Save after deletion
                            if let Err(e) = library.save_to_file() {
                                eprintln!("❌ Could not save library: {e}");
                            }
                        }
                    }
                    Err(_) => {
                        println!("❌ Please enter a valid number.");
                    }
                }
                println!("\nPress Enter to continue...");
                read_line();
            }
            "3" => {
                // Show all books
                library.show();
                println!("\nPress Enter to continue...");
                read_line();
            }
            "4" => {
                // Save & quit
                if let Err(e) = library.save_to_file() {
                    eprintln!("❌ Could not save library: {e}");
                } else {
                    println!("💾 Library saved. Goodbye!");
                }
                break;
            }
            _ => {
                println!("❌ Unknown option. Please choose 1‑4.");
                println!("\nPress Enter to continue...");
                read_line();
            }
        }
    }
}
