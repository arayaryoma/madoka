use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use rustls_pemfile::{certs, rsa_private_keys};

pub fn load_certs(path: &Path) -> Result<Vec<Vec<u8>>, io::Error> {
    certs(&mut BufReader::new(File::open(path)?))
}

pub fn load_keys(path: &Path) -> Result<Vec<Vec<u8>>, io::Error> {
    rsa_private_keys(&mut BufReader::new(File::open(path)?))
}
