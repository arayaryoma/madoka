use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use rustls_pemfile::{certs, rsa_private_keys};
use webpki::types::{CertificateDer, PrivateKeyDer};

pub fn load_certs(path: &Path) -> io::Result<Vec<CertificateDer<'static>>> {
    certs(&mut BufReader::new(File::open(path)?)).collect()
}

pub fn load_keys(path: &Path) -> io::Result<PrivateKeyDer<'static>> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
        .next()
        .unwrap()
        .map(Into::into)
}
