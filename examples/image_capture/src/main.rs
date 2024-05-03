use std::{
    env::current_dir,
    net::{Ipv4Addr, SocketAddrV4},
};

use image::{ImageBuffer, Rgb};
use libvnc::client::{config::ClientConfig, ClientCallbackHandler, RfbClient, VncClient};
use log::{error, info};
struct CallbackHandler(ImageBuffer<Rgb<u8>, Vec<u8>>);
impl ClientCallbackHandler for CallbackHandler {
    fn update(&mut self, client: &RfbClient) {
        info!("updating");
        if (self.0.width() * self.0.height())
            != (TryInto::<u32>::try_into(client.size().0).unwrap()
                * TryInto::<u32>::try_into(client.size().1).unwrap())
        {
            self.0 = ImageBuffer::new(
                client.size().0.try_into().unwrap(),
                client.size().1.try_into().unwrap(),
            );
        }

        let len = self.0.len();
        self.0.copy_from_slice(unsafe {
            core::slice::from_raw_parts(client.0.as_ref().frameBuffer as *const u8, len)
        });
        let image_path = current_dir().unwrap().join("vnc_screen_shot.jpg");
        if let Err(e) = self.0.save(&image_path) {
            error!(
                "failed to save image to {} with error {}",
                image_path.display(),
                e
            );
        } else {
            info!("saved image to {}", image_path.display());
        }
    }
}
fn main() {
    env_logger::init();
    info!("start");
    let config = ClientConfig::new(8, 3, 3);
    let mut client = VncClient::new(
        config,
        core::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5900)),
    );
    client.run(CallbackHandler(ImageBuffer::new(
        client.size().0.try_into().unwrap(),
        client.size().1.try_into().unwrap(),
    )));
}
