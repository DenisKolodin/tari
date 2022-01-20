// Copyright 2021. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::{io, net::TcpListener};

use libtor::{LogDestination, LogLevel, TorFlag};
use log::*;
use multiaddr::Multiaddr;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tari_shutdown::ShutdownSignal;
use tempfile::{tempdir, NamedTempFile, TempDir, TempPath};
use tor_hash_passwd::EncryptedKey;

use crate::{exit_codes::ExitCodes, CommsTransport, TorControlAuthentication};

const LOG_TARGET: &str = "common::tor";

pub struct Tor {
    control_port: u16,
    data_dir: String,
    log_destination: String,
    log_level: LogLevel,
    passphrase: Option<String>,
    socks_port: u16,
    temp_dir: Option<TempDir>,
    temp_file: Option<TempPath>,
}

impl Default for Tor {
    fn default() -> Tor {
        Tor {
            control_port: 19_051,
            data_dir: "/tmp/tor-data".into(),
            log_destination: "/tmp/tor.log".into(),
            log_level: LogLevel::Err,
            passphrase: None,
            socks_port: 19_050,
            temp_dir: None,
            temp_file: None,
        }
    }
}

impl Tor {
    /// Returns a new Tor instance with random options.
    /// The data directory, passphrase, and log destination are temporary and randomized.
    /// Two TCP ports will be provided by the operating system.
    /// These ports are used for the control and socks ports, the onion address and port info are still loaded from the
    /// node identity file.
    pub fn initialize() -> Result<Tor, ExitCodes> {
        let mut instance = Tor::default();

        // check for unused ports to assign
        let (socks_port, control_port) = get_available_ports()?;
        instance.socks_port = socks_port;
        instance.control_port = control_port;

        // generate a random passphrase
        let passphrase: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        instance.passphrase = Some(passphrase);

        // data dir
        let temp = tempdir()?;
        let dir = temp.path().to_string_lossy().to_string();
        instance.temp_dir = Some(temp);
        instance.data_dir = dir;

        // log destination
        let temp = NamedTempFile::new()?.into_temp_path();
        let file = temp.to_string_lossy().to_string();
        instance.temp_file = Some(temp);
        instance.log_destination = file;

        Ok(instance)
    }

    /// Override a given Tor comms transport with the control address and auth from this instance
    pub fn update_comms_transport(&self, transport: CommsTransport) -> Result<CommsTransport, ExitCodes> {
        if let CommsTransport::TorHiddenService {
            socks_address_override,
            forward_address,
            auth,
            onion_port,
            tor_proxy_bypass_addresses,
            tor_proxy_bypass_for_outbound_tcp,
            ..
        } = transport
        {
            let control_server_address = format!("/ip4/127.0.0.1/tcp/{}", self.control_port).parse::<Multiaddr>()?;
            let auth = if let Some(ref passphrase) = self.passphrase {
                TorControlAuthentication::Password(passphrase.to_owned())
            } else {
                auth
            };
            let transport = CommsTransport::TorHiddenService {
                control_server_address,
                socks_address_override,
                forward_address,
                auth,
                onion_port,
                tor_proxy_bypass_addresses,
                tor_proxy_bypass_for_outbound_tcp,
            };
            Ok(transport)
        } else {
            let e = format!("Expected a TorHiddenService comms transport, received: {:?}", transport);
            Err(ExitCodes::ConfigError(e))
        }
    }

    /// Run the Tor instance until the shutdown signal is received
    pub async fn run(self, mut shutdown_signal: ShutdownSignal) -> Result<(), ExitCodes> {
        info!(target: LOG_TARGET, "Starting Tor");

        let Tor {
            data_dir,
            socks_port,
            control_port,
            log_level,
            log_destination,
            passphrase,
            ..
        } = self;

        let mut tor = libtor::Tor::new();

        tor.flag(TorFlag::DataDirectory(data_dir.clone()))
            .flag(TorFlag::SocksPort(socks_port))
            .flag(TorFlag::ControlPort(control_port))
            .flag(TorFlag::Hush())
            .flag(TorFlag::LogTo(log_level, LogDestination::File(log_destination)));

        if let Some(secret) = passphrase {
            let hash = EncryptedKey::hash_password(&secret).to_string();
            tor.flag(TorFlag::HashedControlPassword(hash));
        }

        tor.start_background();

        shutdown_signal.wait().await;

        Ok(())
    }
}

/// Attempt to find 2 available TCP ports
fn get_available_ports() -> Result<(u16, u16), io::Error> {
    let localhost = "127.0.0.1";
    let listener1 = TcpListener::bind((localhost, 0))?;
    let port1 = listener1.local_addr()?.port();

    let listener2 = TcpListener::bind((localhost, 0))?;
    let port2 = listener2.local_addr()?.port();

    Ok((port1, port2))
}