#![allow(unused)]
pub mod config;

use core::{marker::PhantomData, net::SocketAddr, ptr::NonNull};
use std::ffi::CString;

use libvnc_sys::rfb::bindings::{_rfbScreenInfo, rfbGetScreen};

use crate::utils::argv;

use self::config::ServerConfig;

pub struct RfbServer(NonNull<_rfbScreenInfo>);

impl RfbServer {
    pub fn new(config: ServerConfig, addr: SocketAddr, args: Vec<String>) -> Self {
        let argv = argv::Argv::new(args);
        let ptr = unsafe {
            rfbGetScreen(
                argv.get_argc() as *mut i32,
                argv.get_argv() as *mut *mut i8,
                config.width,
                config.height,
                config.bits_per_sample,
                config.samples_per_pixel,
                config.bytes_per_pixel,
            )
        };
        let buffer_size =
            TryInto::<usize>::try_into(config.width * config.height * config.bytes_per_pixel)
                .unwrap();
        (unsafe { *ptr }).frameBuffer = vec![0; buffer_size].as_ptr() as *mut i8;
        let host_addr = CString::new(addr.ip().to_string()).unwrap(); //What if this CString is dropped at the end of new
        (unsafe { *ptr }).httpDir = host_addr.as_ptr() as *mut i8;
        (unsafe { *ptr }).port = addr.port() as i32;
        Self(NonNull::new(ptr).unwrap())
    }
}
pub struct VncServer<T> {
    inner: RfbServer,
    phantom: PhantomData<T>,
}

impl<T> VncServer<T> {
    pub fn new(config: ServerConfig, addr: SocketAddr, args: Vec<String>) -> Self {
        Self {
            inner: RfbServer::new(config, addr, args),
            phantom: PhantomData,
        }
    }
}
