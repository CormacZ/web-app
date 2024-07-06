# Web Application with Rust and Assembly

This project demonstrates a web application using Rust for both frontend (via WebAssembly) and backend, with assembly language integration for performance-critical tasks.

## Directory Structure

- `frontend/`: Rust frontend using Yew framework.
- `backend/`: Rust backend using Actix-Web framework.
- `assembly/`: Assembly language files for specific tasks (e.g., SHA3-512 hashing).

## Setup

1. Clone the repository.
2. Navigate to `frontend/` and `backend/` directories separately.
3. Run `cargo build` and `cargo run` in each directory to build and start the frontend and backend servers, respectively.

## Usage

- Access the web application frontend via `http://localhost:8080` (default Actix-Web port).
- Explore how Rust and assembly integrate for performance optimizations in backend tasks.

## Notes

- Adjust dependencies and implementations based on specific project requirements.
- Assembly (`sha3.asm`) integration assumes low-level optimizations or specific tasks where assembly is beneficial.
