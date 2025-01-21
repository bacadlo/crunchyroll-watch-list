# crunchyroll-watch-list

A Rust program that fetches a <a href="https://www.crunchyroll.com/" target="blank"> Crunchyroll </a> user's watch history and displays them in a CSV file. 

Implemented using <a href="https://github.com/crunchy-labs/crunchyroll-rs" target="blank"> crunchyroll-rs </a> API.

## Installation

1. Make sure you have Rust installed on your system. If not, you can install it from [rustup.rs](https://rustup.rs/)

2. Clone this repository: 

3. Create a new file called `.env` in the root of the project and add your Crunchyroll credentials as follows:

CRUNCHYROLL_EMAIL=your_email
CRUNCHYROLL_PASSWORD=your_password
HISTORY_LIMIT=20

3. Install dependencies:
   
   cargo build
   

4. Run the program:

   cargo run
   



