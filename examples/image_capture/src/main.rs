use std::{
    env::current_dir,
    fs::create_dir,
    net::{Ipv4Addr, SocketAddrV4},
};

use image::{ImageBuffer, Rgba};
use libvnc::client::{config::ClientConfig, ClientCallbackHandler, RfbClient, VncClient};
use log::{error, info};
struct CallbackHandler(ImageBuffer<Rgba<u8>, Vec<u8>>);
impl ClientCallbackHandler for CallbackHandler {
    fn update(&mut self, client: &RfbClient) {
        info!("updating");
        let (width, height) = (
            client.size().0.try_into().unwrap(),
            client.size().1.try_into().unwrap(),
        );
        if (self.0.width() * self.0.height())
            != (TryInto::<u32>::try_into(client.size().0).unwrap()
                * TryInto::<u32>::try_into(client.size().1).unwrap())
        {
            self.0 = ImageBuffer::new(
                client.size().0.try_into().unwrap(),
                client.size().1.try_into().unwrap(),
            );
        }
        let output_path = current_dir().unwrap().join("image_output");
        if !output_path.exists() {
            if let Err(e) = create_dir(&output_path) {
                error!(
                    "Faild to create output dir {} with error {}",
                    output_path.display(),
                    e
                );
            }
        }
        let len = self.0.len();
        let image_path = output_path.join("vnc_screen_shot.bmp");
        let r = image::save_buffer(
            &image_path,
            unsafe { core::slice::from_raw_parts(client.framebuffer_ptr(), len) },
            width,
            height,
            image::ExtendedColorType::Rgba8,
        );

        if let Err(e) = r {
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
    let config = ClientConfig::new(8, 3, 4);
    let mut client = VncClient::new(
        config,
        core::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5900)),
    );
    info!("start");
    client.run(CallbackHandler(ImageBuffer::new(
        client.size().0.try_into().unwrap(),
        client.size().1.try_into().unwrap(),
    )));
}

#[test]
fn test_image() {
    //! An example of generating julia fractals.
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
