#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `VBUSDETH`"]
pub type VBUSDETH_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUSDETL`"]
pub type VBUSDETL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCD`"]
pub type DCD_R = crate::R<bool, bool>;
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Reader of field `SD`"]
pub type SD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - VBUS Detect High Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUS Detect Low Interrupt Flag"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VBUSDETL_R {
        VBUSDETL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Detection Error Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data Contact Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Primary Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Secondary Detection Complete Interrupt Flag"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
