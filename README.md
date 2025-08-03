# NoSQL Injection Lesson in Rust 🦀

This project demonstrates a NoSQL injection vulnerability in a Rust application using the `actix-web` framework and MongoDB. It provides both a vulnerable implementation to exploit and a secure, mitigated version to show best practices for prevention.

## Lesson Summary

NoSQL injection is a vulnerability where an attacker can inject malicious query fragments into a NoSQL database query. This attack targets the database, but it can sometimes be more dangerous than traditional SQL injection, as certain NoSQL databases allow for the execution of procedural language (like JavaScript), potentially leading to server-side code execution.

This lesson walks through performing a NoSQL injection to bypass authentication and then demonstrates how proper input sanitization serves as the primary mitigation strategy.

-----

## Project Setup

1.  **Install Prerequisites.** You will need:

      * **Rust**: Get it from [rustup.rs](https://rustup.rs/).
      * **MongoDB**: For local development, it's recommended to [install MongoDB with Homebrew](https://www.mongodb.com/docs/manual/tutorial/install-mongodb-on-os-x/) or run it via [Docker](https://hub.docker.com/_/mongo).

2.  **Clone and Configure the Project.**

    ```bash
    git clone <repository-url>
    cd nosql-injection-lesson
    ```

3.  **Start MongoDB.**
    Ensure your MongoDB server is running. If using Homebrew on macOS, for example:

    ```bash
    # (One-time setup if you haven't done it before)
    brew tap mongodb/brew
    brew install mongodb-community

    # Start the service
    brew services start mongodb-community
    ```

4.  **Populate the Database.**
    Run the `populate_db` binary to seed the database with initial data. This command must be run from the project's root directory.

    ```bash
    cargo run --bin populate_db
    ```

-----

## Running the Application

From the project's root directory, run the main web server application.

```bash
cargo run --bin nosql-injection-lesson
```

The server will be running at `http://127.0.0.1:8080`.

-----

## Interacting with the Application

All `curl` commands should be run from a separate terminal window while the server is running.

### Vulnerability Demonstration

1.  **Authentication Bypass**: We will log in as `philippe` without his password by injecting the `$ne` (not equal) operator into the JSON payload.

    ```bash
    curl -X POST -H "Content-Type: application/json" \
    -d '{ "username": "philippe", "password": { "$ne": null } }' \
    http://127.0.0.1:8080/vulnerable/login
    ```

    **Result**: You will receive an HTML page showing Philippe's plants, proving the login bypass was successful.

2.  **JavaScript Injection in Search**: We will exploit the `$where` operator in the vulnerable search to inject JavaScript that always evaluates to `true`, dumping all records.

    ```bash
    curl -X POST -d "'; return true; //" http://127.0.0.1:8080/vulnerable/search
    ```

    **Result**: You will see a list of **all** plants from all users, not just a specific search result.

### Mitigation Demonstration

Now, run attacks against the `/secure` endpoints to see them fail.

1.  **Secure Login Attempt**:

    ```bash
    curl -X POST -d "username=philippe" -d "password[\$ne]=null" http://127.0.0.1:8080/secure/login
    ```

    **Result**: The server returns a `400 Bad Request` error because it cannot parse the malicious payload into the expected simple string format. The attack is stopped.

2.  **Secure Search Attempt**:

    ```bash
    curl -X POST -d "'; return true; //" http://127.0.0.1:8080/secure/search
    ```

    **Result**: The server returns an empty list because it is safely searching for a plant with the literal name `'; return true; //'`.

-----

## Key Takeaways

  * NoSQL databases are vulnerable to injection attacks, similar to their SQL counterparts.
  * The impact of NoSQL injection can be severe, potentially allowing for arbitrary code execution within the database, not just data manipulation.
  * The primary and most effective defense is to always **sanitize and validate user-supplied input**. Never trust that user input will be well-formed. Avoid dangerous operators like `$where` when dealing with user data.