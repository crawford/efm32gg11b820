#[doc = "Reader of register PTR"]
pub type R = crate::R<u32, super::PTR>;
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u8, u8>;
#[doc = "Reader of field `WR`"]
pub type WR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Result Buffer Read Pointer"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Result Buffer Write Pointer"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
