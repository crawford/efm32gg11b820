#[doc = "Reader of register ECCPARITY"]
pub type R = crate::R<u32, super::ECCPARITY>;
#[doc = "Reader of field `ECCPARITY`"]
pub type ECCPARITY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Parity Data"]
    #[inline(always)]
    pub fn eccparity(&self) -> ECCPARITY_R {
        ECCPARITY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
