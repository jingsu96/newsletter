## Note

### Chapter 1

#### Inner Development Loop

1. Make a change;
2. Compile the application;
3. Run tests;
4. Run the application;

The speed of your inner development loop is an upper bound on the number of iterations that you can complete in a unit of time.


#### Tools to speed up the inner development loop

1. Faster Linking - LLVM
2. cargo-watch
3. CI: With a collection of automated checks running on every commit - our CI pipeline. If one of the checks fails you cannot merge to main - as simple as that
    3.1 - cargo test (cargo test)
    3.2 - code coverage (tarpaulin)
    3.3 - Linting (clippy)
    3.4 - Formatting (rustfmt)
    3.5 - Security Vulnerabilities (cargo audit)

#### Feature Requirement

```bash
As a ...
I want ...
So that ...
```

Two users stories:

1. As a blog visitor, I want to subscribe to the newsletter, so that I can receive email updates when new content is published on the blog.
2. As a blog author, I want to send a email to all my subscribers, so that I can notify them when new content is published on the blog.

To start:

1. Choose a web framework and get familiar with it.
2. Define our test strategy
3. Choose a crate to interact with the database
4. Define how we want to manage changes to our database schema over time
5. Actually write some queries


Framework

1. actix-web
2. testing
    2.1 embeded test module
    2.2 external tests folder