#[doc = "Reader of register ETMPIDR0"]
pub type R = crate::R<u32, super::ETMPIDR0>;
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
