<div align="center">
  <h1>Tomevio Backend</h1>
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/tomevioDark.svg">
    <source media="(prefers-color-scheme: light)" srcset="assets/tomevioLight.svg">
    <img alt="Tomevio Backend: The Rust-powered engine for your literary journey"
         src="assets/tomevioLight.svg"
         width="50%">
  </picture>
  <p>High-performance backend for tracking, discovering, and organizing books</p>
</div>

## About

**Tomevio Backend** is the robust foundation powering the Tomevio ecosystem - a free and open-source platform for book enthusiasts. Crafted in **Rust** with the **Axum** framework, it delivers a lightning-fast REST API that seamlessly supports both the [web interface](https://github.com/s4nj1th/tomevio-website) and [mobile experience](https://github.com/s4nj1th/tomevio-mobile-app).

## Key Features

- **Comprehensive Book Search** - Instant access to millions of titles through OpenLibrary API integration
- **Detailed Metadata** - Retrieve complete book information including covers, descriptions, and publication details
- **Author Profiles** - Access biographies, bibliographies, and related works
- **Blazing Fast Performance** - Built with `axum`, `tokio`, and `reqwest` for optimal efficiency
- **Modular Design** - Clean, maintainable architecture with clear separation of concerns

## Quick Start

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (included with Rust)

### Installation & Setup
```bash
# Clone the repository
git clone https://github.com/your-org/booktrack-backend.git
cd booktrack-backend

# Build the project
cargo build

# Launch the server
cargo run
```
The API will be available at `http://localhost:8080`

## API Documentation
Explore our comprehensive API reference in [docs/api.md](docs/api.md) to integrate with Tomevio's powerful features.

## How to Contribute
We welcome contributions from the open-source community! To help improve Tomevio:

1. Ensure your code is formatted with `cargo fmt`
2. Follow Rust best practices and conventions
3. Thoroughly document new features and endpoints
4. Submit well-described pull requests

## License
Tomevio Backend is open-source software released under the [MIT License](LICENSE).

## Maintainer
### Sanjith
- [GitHub](https://github.com/s4nj1th) | [Twitter](https://x.com/s4nj1th) | [Email](mailto:sanjith.develops@gmail.com)
