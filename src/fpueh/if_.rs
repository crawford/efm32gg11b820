#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `FPIOC`"]
pub type FPIOC_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPDZC`"]
pub type FPDZC_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPUFC`"]
pub type FPUFC_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPOFC`"]
pub type FPOFC_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPIDC`"]
pub type FPIDC_R = crate::R<bool, bool>;
#[doc = "Reader of field `FPIXC`"]
pub type FPIXC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FPU invalid operation"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FPU divide-by-zero exception"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FPU underflow exception"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FPU overflow exception"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FPU input denormal exception"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FPU inexact exception"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
