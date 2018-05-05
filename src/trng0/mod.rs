use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub control: CONTROL,
    #[doc = "0x04 - FIFO Level Register"]
    pub fifolevel: FIFOLEVEL,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - FIFO Depth Register"]
    pub fifodepth: FIFODEPTH,
    #[doc = "0x10 - Key Register 0"]
    pub key0: KEY0,
    #[doc = "0x14 - Key Register 1"]
    pub key1: KEY1,
    #[doc = "0x18 - Key Register 2"]
    pub key2: KEY2,
    #[doc = "0x1c - Key Register 3"]
    pub key3: KEY3,
    #[doc = "0x20 - Test Data Register"]
    pub testdata: TESTDATA,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Status Register"]
    pub status: STATUS,
    #[doc = "0x34 - Initial Wait Counter"]
    pub initwaitval: INITWAITVAL,
    _reserved2: [u8; 200usize],
    #[doc = "0x100 - FIFO Data"]
    pub fifo: FIFO,
}
#[doc = "Main Control Register"]
pub struct CONTROL {
    register: VolatileCell<u32>,
}
#[doc = "Main Control Register"]
pub mod control;
#[doc = "FIFO Level Register"]
pub struct FIFOLEVEL {
    register: VolatileCell<u32>,
}
#[doc = "FIFO Level Register"]
pub mod fifolevel;
#[doc = "FIFO Depth Register"]
pub struct FIFODEPTH {
    register: VolatileCell<u32>,
}
#[doc = "FIFO Depth Register"]
pub mod fifodepth;
#[doc = "Key Register 0"]
pub struct KEY0 {
    register: VolatileCell<u32>,
}
#[doc = "Key Register 0"]
pub mod key0;
#[doc = "Key Register 1"]
pub struct KEY1 {
    register: VolatileCell<u32>,
}
#[doc = "Key Register 1"]
pub mod key1;
#[doc = "Key Register 2"]
pub struct KEY2 {
    register: VolatileCell<u32>,
}
#[doc = "Key Register 2"]
pub mod key2;
#[doc = "Key Register 3"]
pub struct KEY3 {
    register: VolatileCell<u32>,
}
#[doc = "Key Register 3"]
pub mod key3;
#[doc = "Test Data Register"]
pub struct TESTDATA {
    register: VolatileCell<u32>,
}
#[doc = "Test Data Register"]
pub mod testdata;
#[doc = "Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod status;
#[doc = "Initial Wait Counter"]
pub struct INITWAITVAL {
    register: VolatileCell<u32>,
}
#[doc = "Initial Wait Counter"]
pub mod initwaitval;
#[doc = "FIFO Data"]
pub struct FIFO {
    register: VolatileCell<u32>,
}
#[doc = "FIFO Data"]
pub mod fifo;
