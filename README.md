# rust-tls-handshake
A Secure Server-to-Server Handshake in Rust
🚀 Overview

This project offers a server-to-server communication system leveraging the safety and concurrency features of Rust, paired with the cryptographic strength of OpenSSL. Ensuring your server intercommunications are encrypted and trustworthy.

🌟 Features

- Secure Communication: Establish encrypted communication channels between servers.
- Asynchronous Operations: Utilize asynchronous operations for non-blocking communication, ensuring optimal performance.
- Self-Signed Certificates: For testing and development purposes, easily generate and use self-signed certificates.
- Easy-to-Use: Straightforward setup and usage, making secure communication a breeze.

🔧 Prerequisites

  Rust and Cargo installed on your machine.
  OpenSSL library.

🛠️ Installation

Clone the repository:

    git clone https://github.com/luishsr/rust-tls-handshake.git

Navigate into the project directory:

    cd secure-server-handshake-rust

Build the project:

    cargo build --release

🚴‍♀️ Quickstart

Generate SSL Certificates:

    openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365

Run the server:

    cargo run --bin server

In a separate terminal instance, run the client:

    cargo run --bin client

📖 Documentation

For a deep dive into how everything works, check out the docs/ folder or visit our Wiki page.
🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you'd like to change. Ensure to update tests as appropriate.
📜 License

This project is licensed under the MIT License - see the LICENSE.md file for details.

For any inquiries or discussions, feel free to reach out - luis@luissoares.tech

Give your servers the secret handshake they deserve! 💼🤖🤝🤖
