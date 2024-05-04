use libvnc_sys::rfb::bindings::rfbPixelFormat;

pub struct RfbPixelFormat {
    pub bits_per_pixel: u8,
    pub depth: u8,
    pub big_endian: Option<u8>,
    pub true_colour: u8,
    pub red_max: u16,
    pub green_max: u16,
    pub blue_max: u16,
    pub red_shift: u8,
    pub green_shift: u8,
    pub blue_shift: u8,
}

impl From<RfbPixelFormat> for rfbPixelFormat {
    fn from(value: RfbPixelFormat) -> Self {
        let RfbPixelFormat {
            bits_per_pixel,
            depth,
            big_endian,
            true_colour,
            red_max,
            green_max,
            blue_max,
            red_shift,
            green_shift,
            blue_shift,
        } = value;
        Self {
            bitsPerPixel: bits_per_pixel,
            depth: depth,
            bigEndian: big_endian.unwrap_or(0),
            trueColour: true_colour,
            redMax: red_max,
            greenMax: green_max,
            blueMax: blue_max,
            redShift: red_shift,
            greenShift: green_shift,
            blueShift: blue_shift,
            pad1: 0,
            pad2: 0,
        }
    }
}

pub struct ClientConfig {
    pub(crate) bits_per_sample: i32,
    pub(crate) samples_per_pixel: i32,
    pub(crate) bytes_per_pixel: i32,
}

impl ClientConfig {
    /// Creates a new [`ClientConfig`] with given bits_per_sample, samples_per_pixel and bytes_per_pixel
    pub fn new(bits_per_sample: i32, samples_per_pixel: i32, bytes_per_pixel: i32) -> Self {
        Self {
            bits_per_sample,
            samples_per_pixel,
            bytes_per_pixel,
        }
    }
}

impl Default for ClientConfig {
    /// Default value
    ///
    /// ## Value
    ///
    /// - `bits_per_sample` =  8,
    /// - `samples_per_pixel` = 3,
    /// - `bytes_per_pixel` = 4,
    fn default() -> Self {
        Self {
            bits_per_sample: 8,
            samples_per_pixel: 3,
            bytes_per_pixel: 4,
        }
    }
}
