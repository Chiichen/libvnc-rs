pub mod config;
use core::net::SocketAddr;
use core::ptr::NonNull;
use std::{ffi::CString, marker::PhantomData};

use libvnc_sys::rfb::{
    _rfbClient, rfbClientCleanup, rfbClientGetClientData, rfbClientSetClientData, rfbGetClient,
    rfbInitClient, HandleRFBServerMessage, WaitForMessage,
};
use log::info;

use self::config::ClientConfig;
pub struct RfbClient(pub NonNull<_rfbClient>); //Ptr's lifecycle is not

impl RfbClient {
    /// Creates a new [`RfbClient`] with given [`ClientConfig`] and [`SocketAddr`]
    pub fn new(config: ClientConfig, addr: SocketAddr) -> Self {
        let client = unsafe {
            rfbGetClient(
                config.bits_per_sample,
                config.samples_per_pixel,
                config.bytes_per_pixel,
            )
        };
        let host_addr = std::ffi::CString::new(addr.ip().to_string()).unwrap(); //What if this CString is dropped at the end of new
        (unsafe { *client }).serverHost = host_addr.as_ptr() as *mut i8;
        unsafe { *client }.serverPort = addr.port().try_into().unwrap();
        Self(NonNull::new(client).unwrap())
    }
    fn from_raw(ptr: *mut _rfbClient) -> Option<RfbClient> {
        NonNull::new(ptr).map(|inner| Self(inner))
    }
    pub fn size(&self) -> (i32, i32) {
        (
            unsafe { self.0.as_ref() }.width,
            unsafe { self.0.as_ref() }.height,
        )
    }
}
pub struct VncClient<T> {
    inner: RfbClient,
    phantom: PhantomData<T>,
}

impl<T: ClientCallbackHandler> VncClient<T> {
    pub fn new(config: ClientConfig, addr: SocketAddr) -> Self {
        let mut inner = RfbClient::new(config, addr);
        unsafe { inner.0.as_mut() }.FinishedFrameBufferUpdate = Some(client_callback::<T>);

        Self {
            inner,
            phantom: PhantomData,
        }
    }
    // pub fn set_callback<F>(&mut self, callback: F)
    // where
    //     F: FnMut(T),
    // {
    //     unsafe { *self.inner.0.as_mut() }.FinishedFrameBufferUpdate = Some(client_callback::<T>);
    // }
    pub fn run(&mut self, handler: T) {
        unsafe {
            rfbClientSetClientData(
                self.inner.0.as_ptr(),
                core::ptr::null_mut(),
                Box::into_raw(Box::new(handler)) as *mut core::ffi::c_void,
            )
        }
        unsafe {
            rfbInitClient(
                self.inner.0.as_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        // std::thread::spawn(||)
        loop {
            let msg = unsafe { WaitForMessage(self.inner.0.as_mut(), 500) };
            if msg < 0 {
                info!("disconnected");
                unsafe { rfbClientCleanup(self.inner.0.as_mut()) };
                break;
            }

            if msg != 0 && unsafe { HandleRFBServerMessage(self.inner.0.as_mut()) } == 0 {
                info!("disconnected");
                unsafe { rfbClientCleanup(self.inner.0.as_mut()) };
                break;
            }
        }
    }

    pub fn size(&self) -> (i32, i32) {
        self.inner.size()
    }
}

unsafe extern "C" fn client_callback<T: ClientCallbackHandler>(client: *mut _rfbClient) {
    let data = rfbClientGetClientData(client, core::ptr::null_mut()) as *mut T;
    data.as_mut()
        .unwrap()
        .update(&RfbClient::from_raw(client).unwrap());
}
pub trait ClientCallbackHandler {
    fn update(&mut self, client: &RfbClient);
}
