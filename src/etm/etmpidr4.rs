#[doc = "Reader of register ETMPIDR4"]
pub type R = crate::R<u32, super::ETMPIDR4>;
#[doc = "Reader of field `CONTCODE`"]
pub type CONTCODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - JEP106 Continuation Code"]
    #[inline(always)]
    pub fn contcode(&self) -> CONTCODE_R {
        CONTCODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
