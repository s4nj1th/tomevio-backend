<div align="center">
  <h1>Tomevio Backend</h1>
  <p>A fast, modular backend for a book tracking platform.</p>
</div>



This repo contains the backend for **Tomevio**, a FOSS book tracker platform. It is built in **Rust** using the **Axum** framework to deliver a REST API that powers both the [website frontend](https://github.com/s4nj1th/tomevio-website) and the [mobile app](https://github.com/s4nj1th/tomevio-mobile-app).



## Features

- **Book Search**: Search for books and authors using the OpenLibrary API.
- **Book Details**: Fetch metadata like title, description, and authors.
- **Author Info**: Get author names and bios using OpenLibrary.
- **Modular Architecture**: Clean separation of routes and logic.
- **Powered by Rust**: Leverages `axum`, `tokio`, and `reqwest` for speed and safety.



## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-org/booktrack-backend.git
   cd booktrack-backend
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the server:

   ```bash
   cargo run
   ```

4. The server will be available at `http://localhost:8080`.



## API Documentation

See [docs/api.md](docs/api.md) for detailed information on all API endpoints.



## Contributing

Pull requests and issues are welcome!
Please:

* Run `cargo fmt` before submitting
* Follow idiomatic Rust conventions
* Document any new routes or features



## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.



## Maintainers

### Sanjith
* [Github](https://github.com/s4nj1th)
* [Twitter (X)](https://x.com/s4nj1th)
* [Email](mailto:sanjith.develops@gmail.com)
