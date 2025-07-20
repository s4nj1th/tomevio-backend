<div align="center">
<h1>Tomevio Backend</h1>
</div>

This repo contains the backend for **Tomevio**, a book tracker platform. It is built in Rust using the Axum framework to deliver a REST API that powers both [the website](https://github.com/s4nj1th/tomevio-website) and the [mobile app](https://github.com/s4nj1th/tomevio-mobile-app).

---

## Features

- **Book Search**: Search for books using the OpenLibrary API.
- **RESTful API**: Provides endpoints for managing books, users, and tracking data.
- **Scalable Architecture**: Built with modular routes and a focus on performance.
- **Rust Ecosystem**: Leverages modern Rust libraries like Axum, Tokio, and Reqwest.

---

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- [OpenLibrary API](https://openlibrary.org/developers/api) for book data

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/s4nj1th/tomevio-backend.git
   cd tomevio-backend
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. The server will be running at `http://localhost:8080`.

---

## API Endpoints

### `GET /`
- **Description**: Returns a welcome message.
- **Response**: `"Hello, World!"`

### `GET /search?q=<query>`
- **Description**: Searches for books using the OpenLibrary API.
- **Query Parameters**:
  - `q` (string): The search query.
- **Response**:
  ```json
  [
    {
      "title": "Book Title",
      "author_name": ["Author 1", "Author 2"]
    }
  ]
  ```

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.