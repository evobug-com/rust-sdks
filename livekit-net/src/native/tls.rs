// Copyright 2026 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! rustls client configs for the native transport.
//!
//! Built here rather than through tokio-tungstenite's and reqwest's own root
//! features: reqwest 0.13 no longer has root-store features (its `rustls`
//! feature hard-wires aws-lc-rs and the platform verifier), and building the
//! websocket config here lets both share one webpki-roots version. Each config
//! uses the process-default crypto provider when one is installed and ring
//! otherwise, the same choice tokio-tungstenite and reqwest 0.12 made.

use std::sync::Arc;

use tokio_rustls::rustls::{
    self,
    crypto::{ring, CryptoProvider},
    ClientConfig, RootCertStore,
};

fn provider() -> Arc<CryptoProvider> {
    CryptoProvider::get_default().cloned().unwrap_or_else(|| Arc::new(ring::default_provider()))
}

fn config(roots: RootCertStore) -> Result<ClientConfig, rustls::Error> {
    Ok(ClientConfig::builder_with_provider(provider())
        .with_safe_default_protocol_versions()?
        .with_root_certificates(roots)
        .with_no_client_auth())
}

/// The websocket config, with the roots tokio-tungstenite's
/// `rustls-tls-native-roots`/`rustls-tls-webpki-roots` features would load.
/// Built per connection, as tokio-tungstenite does, so a changed OS store is
/// picked up on reconnect.
pub(super) fn ws_config() -> Result<Arc<ClientConfig>, String> {
    #[allow(unused_mut)]
    let mut roots = RootCertStore::empty();
    #[cfg(feature = "rustls-tls-native-roots")]
    {
        let rustls_native_certs::CertificateResult { certs, errors, .. } =
            rustls_native_certs::load_native_certs();
        if !errors.is_empty() {
            log::warn!("native root CA certificate loading errors: {errors:?}");
        }
        // Not finding any native roots is not fatal when webpki roots are bundled.
        #[cfg(not(feature = "rustls-tls-webpki-roots"))]
        if certs.is_empty() {
            return Err(format!("no native root CA certificates found (errors: {errors:?})"));
        }
        let total = certs.len();
        let (added, ignored) = roots.add_parsable_certificates(certs);
        log::debug!("Added {added}/{total} native root certificates (ignored {ignored})");
    }
    #[cfg(feature = "rustls-tls-webpki-roots")]
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    config(roots).map(Arc::new).map_err(|e| e.to_string())
}

/// The HTTP config, with the roots reqwest 0.12's root features loaded and its
/// HTTP/1.1-only ALPN (this crate does not enable reqwest's `http2`).
pub(super) fn http_config() -> ClientConfig {
    #[allow(unused_mut)]
    let mut roots = RootCertStore::empty();
    #[cfg(feature = "rustls-tls-webpki-roots")]
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    #[cfg(feature = "rustls-tls-native-roots")]
    roots.add_parsable_certificates(rustls_native_certs::load_native_certs().certs);
    let mut config = config(roots).expect("the crypto provider supports the default TLS versions");
    config.alpn_protocols = vec![b"http/1.1".to_vec()];
    config
}
