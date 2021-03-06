use std::{net::SocketAddr, sync::Arc, time::Duration};

pub mod client;
mod config;
mod error;
pub mod server;
use config::Config;
use error::Error;

use bytes::{Bytes, BytesMut};
use quiche::ConnectionId;

const MAX_DATAGRAM_SIZE: usize = 1350;

pub struct Connection {
    inner: std::pin::Pin<Box<quiche::Connection>>,
}

impl Connection {
    pub fn connect(cfg: Arc<Config>, to: SocketAddr) -> Result<Self, Error> {
        let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
        // Configure connection
        config.set_application_protos(b"\x0ahq-interop\x05hq-29\x05hq-28\x05hq-27\x08http/0.9")?;
        config.set_max_idle_timeout(5000);
        config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
        config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
        config.set_initial_max_data(10_000_000);
        config.set_initial_max_stream_data_bidi_local(1_000_000);
        config.set_initial_max_stream_data_bidi_remote(1_000_000);
        config.set_initial_max_stream_data_uni(1_000_000);
        config.set_initial_max_streams_bidi(100);
        config.set_initial_max_streams_uni(100);
        config.set_disable_active_migration(true);
        config.enable_early_data();
        config.load_verify_locations_from_file(&cfg.auth.ca_cert_file)?;
        config.load_cert_chain_from_pem_file(&cfg.auth.cert_file)?;
        config.load_priv_key_from_pem_file(&cfg.auth.key_file)?;

        let scid = ConnectionId::from_ref(&[0xba, 16]);

        Ok(Connection {
            inner: quiche::connect(None, &scid, to, &mut config)?,
        })
    }

    pub async fn accept(
        scid: &ConnectionId<'_>,
        odcid: Option<&ConnectionId<'_>>,
        from: SocketAddr,
        cfg: Arc<Config>,
    ) -> Result<Self, Error> {
        let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
        // Configure connection
        config.set_application_protos(b"\x0ahq-interop\x05hq-29\x05hq-28\x05hq-27\x08http/0.9")?;
        config.set_max_idle_timeout(5000);
        config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
        config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
        config.set_initial_max_data(10_000_000);
        config.set_initial_max_stream_data_bidi_local(1_000_000);
        config.set_initial_max_stream_data_bidi_remote(1_000_000);
        config.set_initial_max_stream_data_uni(1_000_000);
        config.set_initial_max_streams_bidi(100);
        config.set_initial_max_streams_uni(100);
        config.set_disable_active_migration(true);
        config.enable_early_data();
        config.load_verify_locations_from_file(&cfg.auth.ca_cert_file)?;
        config.load_cert_chain_from_pem_file(&cfg.auth.cert_file)?;
        config.load_priv_key_from_pem_file(&cfg.auth.key_file)?;

        Ok(Connection {
            inner: quiche::accept(scid, odcid, from, &mut config)?,
        })
    }

    pub fn create_stream(&mut self) -> Result<u64, Error> {
        unimplemented!()
    }

    pub fn send_to_stream(&mut self, stream_id: u64, bytes: Bytes) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn recv_from_stream(&mut self, stream_id: u64, bytes: &mut BytesMut) -> Result<(), Error> {
        unimplemented!()
    }

    fn timeout(&self) -> Option<Duration> {
        self.inner.timeout()
    }

    fn on_timeout(&mut self) {
        self.inner.on_timeout()
    }
}
