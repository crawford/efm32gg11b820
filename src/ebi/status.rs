#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `AHBACT`"]
pub type AHBACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCACT`"]
pub type ECCACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXEL0EMPTY`"]
pub type TFTPIXEL0EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXEL1EMPTY`"]
pub type TFTPIXEL1EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXELFULL`"]
pub type TFTPIXELFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDACT`"]
pub type DDACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTDDEMPTY`"]
pub type TFTDDEMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EBI Busy With AHB Transaction"]
    #[inline(always)]
    pub fn ahbact(&self) -> AHBACT_R {
        AHBACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - EBI ECC Generation Active"]
    #[inline(always)]
    pub fn eccact(&self) -> ECCACT_R {
        ECCACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL0 is Empty"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL1 is Empty"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EBI_TFTPIXEL0 is Full"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EBI Busy With Direct Drive Transactions"]
    #[inline(always)]
    pub fn ddact(&self) -> DDACT_R {
        DDACT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EBI_TFTDD Register is Empty"]
    #[inline(always)]
    pub fn tftddempty(&self) -> TFTDDEMPTY_R {
        TFTDDEMPTY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
