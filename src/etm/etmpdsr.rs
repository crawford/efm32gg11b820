#[doc = "Reader of register ETMPDSR"]
pub type R = crate::R<u32, super::ETMPDSR>;
#[doc = "Reader of field `ETMUP`"]
pub type ETMUP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ETM Powered Up"]
    #[inline(always)]
    pub fn etmup(&self) -> ETMUP_R {
        ETMUP_R::new((self.bits & 0x01) != 0)
    }
}
