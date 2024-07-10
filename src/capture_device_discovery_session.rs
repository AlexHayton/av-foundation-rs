use objc2::{
    extern_class, msg_send, msg_send_id, mutability::InteriorMutable, rc::Id, ClassType, Encode,
    Encoding,
};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol};

use crate::{
    capture_device::{AVCaptureDevice, AVCaptureDevicePosition, AVCaptureDeviceType},
    media_format::{AVMediaType, AVMediaTypeVideo},
};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceDiscoverySession;

    unsafe impl ClassType for AVCaptureDeviceDiscoverySession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceDiscoverySession {}

impl AVCaptureDeviceDiscoverySession {
    pub fn init(
        device_types: NSArray<AVCaptureDeviceType>,
        media_type: AVMediaType,
        position: AVCaptureDevicePosition,
    ) -> Id<Self> {
        unsafe {
            msg_send_id![
                AVCaptureDeviceDiscoverySession::class(),
                discoverySessionWithDeviceTypes:device_types
                mediaType:media_type
                position:position
            ]
        }
    }

    pub fn devices(&self) -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![self, devices] }
    }
}
