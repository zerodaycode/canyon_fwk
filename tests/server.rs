use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};

#[test]
fn it_binds() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    assert_eq!(
        listener.local_addr().unwrap(),
        SocketAddr::V4(
            SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1), 
                8080
            )
        )
    );
}
