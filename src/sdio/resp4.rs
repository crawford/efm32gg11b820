#[doc = "Reader of register RESP4"]
pub type R = crate::R<u32, super::RESP4>;
#[doc = "Reader of field `CMDRESP2`"]
pub type CMDRESP2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdresp2(&self) -> CMDRESP2_R {
        CMDRESP2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
