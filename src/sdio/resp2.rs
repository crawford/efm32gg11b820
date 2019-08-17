#[doc = "Reader of register RESP2"]
pub type R = crate::R<u32, super::RESP2>;
#[doc = "Reader of field `CMDRESP1`"]
pub type CMDRESP1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 1"]
    #[inline(always)]
    pub fn cmdresp1(&self) -> CMDRESP1_R {
        CMDRESP1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
