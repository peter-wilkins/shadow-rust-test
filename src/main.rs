extern crate futures;
extern crate futures_cpupool;
extern crate tokio;
extern crate tokio_io;

use futures::{Future, Stream};
use futures::future::Executor;
use futures_cpupool::CpuPool;
use std::env;
use std::io::Write;
use std::io::stdout;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::{TcpListener, TcpStream};
use tokio_io::io;

#[no_mangle]
pub extern "C" fn server() {
    println!("Server listening for connection");

    // create the cpu pool
    let pool = CpuPool::new_num_cpus();

    // parse the listen address
    let addr = "0.0.0.0:12345".parse::<SocketAddr>().unwrap();

    // create the listening socket
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");

    // create the server
    let server = listener.incoming().for_each(|socket| {
        println!("accepted connection from {:?}", socket.peer_addr().unwrap());

        // create a future that writes "world" to the socket
        let writer = io::write_all(socket, "world\n")
            .then(|_res| {
                println!("wrote 'world' {:?}", _res.is_ok());
                let _ = stdout().flush();
                Ok(())
            });

        // execute it as a concurrent task
        pool.execute(writer).unwrap();

        Ok(())
    });

    server.wait().unwrap();
}

#[no_mangle]
pub extern "C" fn client(host: &String) {
    println!("Client connecting to: {}", host);

    // create the cpu pool
    let pool = CpuPool::new_num_cpus();

    // parse the address to connect to
    let addr = match host.to_socket_addrs().unwrap().next() {
        Some(a) => a,
        None => panic!("Failed to resolve hostname"),
    };

    // create the client connection
    let client = TcpStream::connect(&addr).and_then(|socket| {
        println!("connected to {:?}", socket.peer_addr().unwrap());

        // create a future that writes "hello" to the socket
        let writer = io::write_all(socket, "hello\n")
            .then(|_res| {
                println!("wrote 'hello' {:?}", _res.is_ok());
                let _ = stdout().flush();
                Ok(())
            });

        // execute it as a concurrent task
        pool.execute(writer).unwrap();

        Ok(())
    });

    client.wait().unwrap();
}

/* This is the real entry point for both the exe and the dylib */
#[no_mangle]
pub extern "C" fn start() {
    if env::args().count() > 1 {
        client(&env::args().nth(1).unwrap())
    } else {
        server()
    }
}

/* This is only used when running the code standalone */
#[allow(dead_code)]
fn main() {
    start()
}
