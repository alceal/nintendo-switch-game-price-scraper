# Nintendo Switch Game Price Scraper

## Introduction

The Nintendo Switch Game Price Scraper demonstrates web scraping capabilities using Rust. This project exhibits my proficiency in Rust development by creating a web scraping tool that extracts game titles and their prices from a specific webpage and displays the information in the terminal.

## Dependencies

This project relies on the following crates from crates.io:

- `reqwest`: Facilitates making HTTP requests.
- `scraper`: A tool for parsing HTML content.
- `regex`: Used for pattern matching within the scraped content.

## Installation

To utilize the Nintendo Switch Game Price Scraper, follow these installation steps:

```bash
git clone <https://github.com/yourusername/nintendo-switch-game-price-scraper.git>
cd nintendo-switch-game-price-scraper
cargo build --release
```

## Usage

The Nintendo Switch Game Price Scraper is designed to extract game titles and prices from a specific webpage. Here's an example of how to use it:

```bash
cargo run
```

This will execute the scraper and display the game titles and their respective prices in the terminal.
