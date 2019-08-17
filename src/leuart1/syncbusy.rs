#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<bool, bool>;
#[doc = "Reader of field `STARTFRAME`"]
pub type STARTFRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIGFRAME`"]
pub type SIGFRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDATAX`"]
pub type TXDATAX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDATA`"]
pub type TXDATA_R = crate::R<bool, bool>;
#[doc = "Reader of field `PULSECTRL`"]
pub type PULSECTRL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CLKDIV Register Busy"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STARTFRAME Register Busy"]
    #[inline(always)]
    pub fn startframe(&self) -> STARTFRAME_R {
        STARTFRAME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SIGFRAME Register Busy"]
    #[inline(always)]
    pub fn sigframe(&self) -> SIGFRAME_R {
        SIGFRAME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TXDATAX Register Busy"]
    #[inline(always)]
    pub fn txdatax(&self) -> TXDATAX_R {
        TXDATAX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXDATA Register Busy"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PULSECTRL Register Busy"]
    #[inline(always)]
    pub fn pulsectrl(&self) -> PULSECTRL_R {
        PULSECTRL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
