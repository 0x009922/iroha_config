use iroha_primitives::addr::{SocketAddr, SocketAddrHost};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::ops::Deref;

pub fn ugly_schematic_find_addr(base_port: u16) -> Option<DefaultPanic<SocketAddr>> {
    let addr = find_port(base_port)
        .map(|port| {
            SocketAddr::Host(SocketAddrHost {
                host: "127.0.0.1".into(),
                port,
            })
        })
        .unwrap_or_else(|| panic!("could not find available port (base = {base_port})"));
    Some(DefaultPanic(addr))
}

pub fn find_port(base: u16) -> Option<u16> {
    (base..u16::MAX).find(|port| {
        // FIXME: should we check on another host?
        if let Ok(_) = TcpListener::bind(("127.0.0.1", *port)) {
            true
        } else {
            false
        }
    })
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DefaultPanic<T>(pub T);

impl<T> Deref for DefaultPanic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Default for DefaultPanic<T> {
    fn default() -> Self {
        unreachable!()
    }
}
