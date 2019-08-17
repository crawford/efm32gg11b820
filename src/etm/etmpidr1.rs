#[doc = "Reader of register ETMPIDR1"]
pub type R = crate::R<u32, super::ETMPIDR1>;
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `IDCODE`"]
pub type IDCODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Part Number"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 Identity Code"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
