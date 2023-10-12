use rustls::{ClientConfig, ServerConfig, NoClientAuth, Certificate};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::TlsConnector;
use webpki::DNSNameRef;
use std::sync::Arc;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use std::io::BufReader;
use std::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn load_certs() -> Vec<Certificate> {
    let cert_file = File::open("localhost.crt").expect("Cannot open certificate file");
    let mut cert_reader = BufReader::new(cert_file);

    certs(&mut cert_reader).unwrap_or_else(|_| {
        panic!("Failed to read certificate");
    })
}

fn load_private_key() -> rustls::PrivateKey {
    let key_file = &mut BufReader::new(File::open("localhost.key").unwrap());
    let keys = pkcs8_private_keys(key_file).unwrap_or_else(|_| panic!("Failed to load keys"));

    if keys.is_empty() {
        panic!("No keys found!");
    }
    keys[0].clone()
}

async fn handle_client(mut tls_stream: tokio_rustls::server::TlsStream<TcpStream>) {
    // Reads data from the TLS stream
    let mut data = vec![0; 100];
    tls_stream.read(&mut data).await.unwrap();
    if &data[..5] == b"HELLO" {
        tls_stream.write_all(b"HELLO_ACK").await.unwrap();
        println!("Received HELLO from client. Sent HELLO_ACK.");
    }
    // You can add more exchanges here.
}

async fn run_server() {
    let mut server_config = ServerConfig::new(NoClientAuth::new());
    server_config.set_single_cert(load_certs(), load_private_key()).unwrap();
    let server_config = Arc::new(server_config);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let acceptor = TlsAcceptor::from(server_config);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let tls_stream = acceptor.accept(stream).await.unwrap();
        tokio::spawn(handle_client(tls_stream));
    }
}

async fn run_client() {
    let mut config = ClientConfig::new();
    let dns_name = DNSNameRef::try_from_ascii_str("localhost").unwrap();

    let cert = load_certs();
    config.root_store.add(&cert[0]).unwrap();

    let connector = TlsConnector::from(Arc::new(config));
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    let mut tls_stream = connector.connect(dns_name, stream).await.unwrap();

    // Read an Write Data
    tls_stream.write_all(b"HELLO").await.unwrap();
    let mut buffer = vec![0; 100];
    tls_stream.read(&mut buffer).await.unwrap();
    if &buffer[..9] == b"HELLO_ACK" {
        println!("Received HELLO_ACK from server.");
    }

}

#[tokio::main]
async fn main() {

    tokio::spawn(run_server());

    // Giving server some time to start.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    run_client().await;

}

