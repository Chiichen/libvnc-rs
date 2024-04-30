pub mod config;
use libvnc_sys::rfb::bindings::{_rfbClient, rfbClientCleanup, rfbGetClient};

use self::config::ClientConfig;
pub struct RfbClient(*mut _rfbClient);

impl Drop for RfbClient {
    fn drop(&mut self) {
        unsafe { rfbClientCleanup(self.0) }
    }
}

impl RfbClient {
    fn new(config: ClientConfig) -> Self {
        Self(unsafe {
            rfbGetClient(
                config.bits_per_sample,
                config.samples_per_pixel,
                config.bytes_per_pixel,
            )
        })
    }
}
pub struct VncClient {}
