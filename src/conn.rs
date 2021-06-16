use rustls::Certificate;
use std::net::TcpStream;
use std::sync::Arc;
use crate::tls::Conn;
use crate::uri::Uri;

pub static CERTIFICAT_IGC_SANTE: &[u8] = include_bytes!("resources/cert_igc_sante.der");
pub static CERTIFICAT_GIE_AC_SERVEUR: &[u8] = include_bytes!("resources/cert_gie_ac_serveur.der");
pub static CERTIFICAT_GIE_OSI: &[u8] = include_bytes!("resources/cert_gie_osi_sesam_vitale.der");

pub fn get_tls_conn(addr: &Uri) -> Conn<TcpStream> {
    println!("get_tls_conn");
    let mut config = rustls::ClientConfig::new();
    config.key_log = Arc::new(rustls::KeyLogFile::new());
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    config.root_store.add(&Certificate(Vec::from(CERTIFICAT_IGC_SANTE))).unwrap();
    config.root_store.add(&Certificate(Vec::from(CERTIFICAT_GIE_AC_SERVEUR))).unwrap();
    config.root_store.add(&Certificate(Vec::from(CERTIFICAT_GIE_OSI))).unwrap();

    config.ct_logs = Some(&ct_logs::LOGS);

    //config.verifier: Arc::new(vrify::WebPKIVerifier::new()),
    //set_certs_config(&mut config).unwrap();

    use rustls::{ClientSession, StreamOwned};

    let session = ClientSession::new(
        &Arc::new(config),
        webpki::DNSNameRef::try_from_ascii_str(addr.host().unwrap()).unwrap()
    );
    let stream = TcpStream::connect((addr.host().unwrap(), addr.corr_port())).unwrap();
    let stream = StreamOwned::new(session, stream);

    Conn { stream }
}
