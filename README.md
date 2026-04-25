# Personal Library

A command-line personal library manager written in **Rust**.  
Add, delete, and view your book collection, with automatic persistence to a JSON file.

## Features

- **Add a book** – Enter the title, author, reading status (Read / Reading / Unread), and optionally the publication year
- **Delete a book** – Remove a book by its number shown in the library list
- **Show all books** – Display your entire collection with details
- **Persistent storage** – All data is saved to a `library.json` file, so your library survives between sessions

## How it Works

Under the hood, the application uses [`serde`](https://crates.io/crates/serde) and [`serde_json`](https://crates.io/crates/serde_json) to serialize and deserialize Rust structs into JSON, making file I/O clean and reliable.

## Usage

### Run the app

``bash
cargo run``

## Interactive Menu

When you launch the program, you’ll see a menu with four options:

1. Add a new book
2. Delete a book
3. Show the list of books
4. Save and quit

1. Adding a Book

You’ll be prompted for:

    Title (required)

    Author (required)

    State – choose between Read, Reading, Unread (required)

    Year – publication year (optional, press Enter to skip)

After filling the details, the book is immediately added to the library.

2. Deleting a Book

The app displays all books with an index number.
Enter the number of the book you want to delete.
The deletion is permanent after confirmation.

3. Showing the Library

Displays the full list of saved books, each with its number, title, author, status, and year (if provided).

4. Save and Quit

Writes the current library to library.json and exits the program.
The file is automatically loaded next time you run the app.

## DATA FILE

All your books are stored in a file named library.json in the same directory as the executable.
You can manually inspect or back up this file if needed.
