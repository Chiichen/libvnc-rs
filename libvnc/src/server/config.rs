pub struct ServerConfig {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) bits_per_sample: i32,
    pub(crate) samples_per_pixel: i32,
    pub(crate) bytes_per_pixel: i32,
}

impl ServerConfig {
    /// Creates a new [`ServerConfig`] with given bits_per_sample, samples_per_pixel and bytes_per_pixel
    pub fn new(
        width: i32,
        height: i32,
        bits_per_sample: i32,
        samples_per_pixel: i32,
        bytes_per_pixel: i32,
    ) -> Self {
        Self {
            width,
            height,
            bits_per_sample,
            samples_per_pixel,
            bytes_per_pixel,
        }
    }
}

impl Default for ServerConfig {
    /// Default value
    ///
    /// ## Value
    ///
    /// - `bits_per_sample` =  8,
    /// - `samples_per_pixel` = 3,
    /// - `bytes_per_pixel` = 4,
    fn default() -> Self {
        Self {
            width: 400,
            height: 300,
            bits_per_sample: 8,
            samples_per_pixel: 3,
            bytes_per_pixel: 4,
        }
    }
}
