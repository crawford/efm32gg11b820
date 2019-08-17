#[doc = "Reader of register ETMPIDR2"]
pub type R = crate::R<u32, super::ETMPIDR2>;
#[doc = "Reader of field `IDCODE`"]
pub type IDCODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ALWAYS1`"]
pub type ALWAYS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Always 1"]
    #[inline(always)]
    pub fn always1(&self) -> ALWAYS1_R {
        ALWAYS1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
