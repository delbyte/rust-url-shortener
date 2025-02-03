# Rust URL Shortener

This is a URL shortener built with Rust and Axum for the BlazinglyFast Hackathon. It provides functionality to shorten URLs and generate QR codes for them. It's deployed with Render, on a free plan. Expect longer wait times after inactivity. 

You can visit the site at: https://flashurl-2u1k.onrender.com

## Features

- Shortens long URLs into short ones
- Redirects to the original URL when accessing a short URL, which is shareable to anyone
- Generates a QR code for the URL

## Technologies Used

- **Rust**: Programming language
- **Axum**: Web framework
- **SQLx**: Database interaction with SQLite
- **qrcodegen**: QR Code generation
- **image**: Image handling for QR code generation
- **Render**: For deployment

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/delbyte/rust-url-shortener.git
   cd rust-url-shortener
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## Endpoints

- **POST /shorten**: Shortens a URL (requires JSON body with `long_url`).
- **GET /:short_code**: Redirects to the original URL using the short code.
- **GET /qr**: Generates a QR code for a URL (requires query parameter `url`).

## License

This project is licensed under the MIT License.
