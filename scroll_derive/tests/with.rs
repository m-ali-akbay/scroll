use scroll::{BE, Pread, Pwrite, ctx::SizeWith};
use scroll_derive::{Pread, Pwrite, SizeWith};

#[derive(Debug)]
pub struct RemoteType {
    pub foo: u32,
}

#[derive(Debug, Pwrite, Pread, SizeWith)]
pub struct WrapperType {
    pub foo: u64,
}

impl From<&RemoteType> for WrapperType {
    fn from(rt: &RemoteType) -> Self {
        WrapperType { foo: rt.foo as u64 }
    }
}

impl From<&WrapperType> for RemoteType {
    fn from(wt: &WrapperType) -> Self {
        RemoteType { foo: wt.foo as u32 }
    }
}

#[derive(Debug, Pwrite, Pread, SizeWith)]
pub struct WithRemoteType {
    #[scroll(with = WrapperType)]
    pub remote: RemoteType,
}

#[test]
fn test_with_remote_type() {
    assert_eq!(WithRemoteType::size_with(&BE), 8);

    let wr = WithRemoteType {
        remote: RemoteType { foo: 0x04030201 },
    };
    let mut bytes = vec![0u8; 8];
    let size = bytes.pwrite_with(&wr, 0, BE).unwrap();
    assert_eq!(size, 8);
    assert_eq!(&bytes[..], &[
        0x00, 0x00, 0x00, 0x00,
        0x04, 0x03, 0x02, 0x01,
    ]);

    let wr: WithRemoteType = bytes.pread_with(0, BE).unwrap();
    assert_eq!(wr.remote.foo, 0x04030201);
}
