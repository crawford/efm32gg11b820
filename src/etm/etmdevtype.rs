#[doc = "Reader of register ETMDEVTYPE"]
pub type R = crate::R<u32, super::ETMDEVTYPE>;
#[doc = "Reader of field `TRACESRC`"]
pub type TRACESRC_R = crate::R<u8, u8>;
#[doc = "Reader of field `PROCTRACE`"]
pub type PROCTRACE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Trace Source"]
    #[inline(always)]
    pub fn tracesrc(&self) -> TRACESRC_R {
        TRACESRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Processor Trace"]
    #[inline(always)]
    pub fn proctrace(&self) -> PROCTRACE_R {
        PROCTRACE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
