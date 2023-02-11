# Fullstack-Rust-web-app-template
This is a test app written using my framework!

# Instructions
To get started clone the repo into your local machine.

Add the wasm32-unknown-unknown rust target.

Install trunk.

Serve the frontend, run the backend.

Must run diesel setup!!!! with a .env file containing the database path

Must generate ssl certificates:

openssl genrsa -out private.pem 2048

openssl rsa -in private.pem -outform PEM -pubout -out public.pem


You should be able to find the frontend demo in a browser at http://localhost:8080/


Further instructions can be found at: https://yew.rs
