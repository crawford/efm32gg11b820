#[doc = "Reader of register R5VSTATUS"]
pub type R = crate::R<u32, super::R5VSTATUS>;
#[doc = "Reader of field `VREGIDET`"]
pub type VREGIDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUSDET`"]
pub type VBUSDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREGODET`"]
pub type VREGODET_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUSGTVREGI`"]
pub type VBUSGTVREGI_R = crate::R<bool, bool>;
#[doc = "Reader of field `LDODROPOUTDET`"]
pub type LDODROPOUTDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `COLDSTART`"]
pub type COLDSTART_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VREGI Detected"]
    #[inline(always)]
    pub fn vregidet(&self) -> VREGIDET_R {
        VREGIDET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB VBUS Detected"]
    #[inline(always)]
    pub fn vbusdet(&self) -> VBUSDET_R {
        VBUSDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VREGO Detected"]
    #[inline(always)]
    pub fn vregodet(&self) -> VREGODET_R {
        VREGODET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output of the Supply Comparator Between VBUS and VREGI"]
    #[inline(always)]
    pub fn vbusgtvregi(&self) -> VBUSGTVREGI_R {
        VBUSGTVREGI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regulator Dropout Detection"]
    #[inline(always)]
    pub fn ldodropoutdet(&self) -> LDODROPOUTDET_R {
        LDODROPOUTDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates If the Regulator is Going Through a Cold Start"]
    #[inline(always)]
    pub fn coldstart(&self) -> COLDSTART_R {
        COLDSTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
