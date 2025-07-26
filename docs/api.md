# API Documentation

This document outlines the REST API endpoints provided by this service.

---

## [Base URL](http://localhost:8080/)

```
http://localhost:8080/
```

---

## Endpoints

### `GET /`

**Description**: Returns a welcome message.

**Response**
- `200 OK`

```text
Hello, World!
```

---

### `GET /search?q=<query>`

**Description**: Searches for books and authors via OpenLibrary.

**Query Parameters**:

* `q` (string): The search term (e.g., `"harry+potter"`)

**Response**

* `200 OK`

```json
{
  "books": [
    {
      "title": "Sample Book",
      "author_name": ["Author One"],
      "work_id": "Work ID"
    }
  ],
  "authors": [
    {
      "name": "Author One",
      "work_count": 5
    }
  ]
}
```

---

### `GET /book/{{book_id}}`

**Description**: Retrieves metadata for a book by its OpenLibrary Work ID.

**Path Parameters**:

* `book_id` (string): e.g., `OL123456W`

**Response**

* `200 OK`

```json
{
  "title": "Book Title",
  "description": "This is a description of a book...",
  "author_keys": ["/authors/author_id"]
}
```

---

### `GET /author/{{author_id}}`

**Description**: Retrieves metadata and bio for an author by their OpenLibrary Author ID.

**Path Parameters**:

* `author_id` (string): e.g., `OL98765A`

**Response**

* `200 OK`

```json
{
  "name": "Author One",
  "bio": "Author One is a fictional writer used for documentation examples..."
}
```

---

## Error Format

All errors are returned in a consistent JSON format.

```json
{
  "error": "Description of the error"
}
```

**Common status codes**:

* `400 Bad Request`: Malformed or missing parameters
* `404 Not Found`: Resource not found
* `500 Internal Server Error`: Server-side issue

---

## Notes

* All responses use `Content-Type: application/json` unless otherwise noted.
* This API does not require authentication (yet).
* Future versions may add pagination and filtering options.
