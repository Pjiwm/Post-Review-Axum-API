# Post Review Axum API
Axum is a web application framework that focuses on ergonomics and modularity for Rust.
This project was made to try out Axum in Rust to make a functional restful api.

The API lets you do CRUD actions on Posts and Reviews.
The API contains a generic controller, connection to mongoDB, authentication via jsonwebtokens and encryption for passwords.
## Goals for this project
- [x] Connect to a database
- [x] CRUD requests
- [x] several models
- [x] encrypt passwords of users
- [x] use jsonwebtokens for requests
- [x] middleware
- [x] a model with complex datatypes (time, lists e.g.)
- [x] only objects can be deleted/edited by the author
