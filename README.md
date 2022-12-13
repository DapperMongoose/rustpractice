# rustpractice
Re-implement the silly practice project that I did for golang in rust, for education and fun.

Use a ton of overkill to create a stateful web app with a sqlite db, front end web service, and back-end web service.

N.B. Because all the Rust front-end libraries involve a lot more complexity than I felt warranted for this 
(admittedly already overly complex) toy implementation I'm recycling my go practice project's web client for this one.

Maybe in the future after I do some more reading on WASM with Rust I'll consider going that route.


In order to run this project:
* Open two terminals
* In one terminal cd to the client directory and go run webclient.go to start the web client
* In the second terminal cargo run from the root directory
* Point your browser to localhost:8000 for the client GUI.  The server will run on localhost:8080