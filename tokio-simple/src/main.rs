extern crate futures;
extern crate tokio_core;

use futures::{Future, Stream};
use tokio_core::io::{copy, Io};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;


fn main() {

    // create an event loop
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // bind to server socket
    let addr = "127.0.0.1:12345".parse().unwrap();
    let tcpListener = TcpListener::bind(&addr, &handle).unwrap();

    // stream of sockets for incoming connections
    let server = tcpListener.incoming().for_each(|(sock, _)|{

        // split reader and writer
        let (reader, writer) = sock.split();

        // echo and return data
        let bytes_copied = copy(reader, writer);

        // print
        let handle_connection = bytes_copied.map(|amt|{
            println!("wrote {} bytes", amt);
        }).map_err(|err| {
            println!("Error occured: {:?}", err);
        });

        handle.spawn(handle_connection);

        Ok(())

    });

    core.run(server).unwrap();


}