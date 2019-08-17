#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `VBUSDETH`"]
pub type VBUSDETH_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEMACTIVE`"]
pub type LEMACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDTO`"]
pub type DCDTO_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDP`"]
pub type SDP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CDP`"]
pub type CDP_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCP`"]
pub type DCP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACAFS`"]
pub type ACAFS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACALS`"]
pub type ACALS_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBCDBUSY`"]
pub type USBCDBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VBUS Detect High"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LEMACTIVE_R {
        LEMACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data Contact Detection Timeout"]
    #[inline(always)]
    pub fn dcdto(&self) -> DCDTO_R {
        DCDTO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Standard Downstream Port Detected"]
    #[inline(always)]
    pub fn sdp(&self) -> SDP_R {
        SDP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Charging Downstream Port Detected"]
    #[inline(always)]
    pub fn cdp(&self) -> CDP_R {
        CDP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dedicated Charging Port Detected"]
    #[inline(always)]
    pub fn dcp(&self) -> DCP_R {
        DCP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ACA Full Speed TypeB Device"]
    #[inline(always)]
    pub fn acafs(&self) -> ACAFS_R {
        ACAFS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ACA Low Speed TypeB Device"]
    #[inline(always)]
    pub fn acals(&self) -> ACALS_R {
        ACALS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USB Charger Detect Busy"]
    #[inline(always)]
    pub fn usbcdbusy(&self) -> USBCDBUSY_R {
        USBCDBUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
