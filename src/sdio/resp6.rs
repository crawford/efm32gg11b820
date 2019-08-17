#[doc = "Reader of register RESP6"]
pub type R = crate::R<u32, super::RESP6>;
#[doc = "Reader of field `CMDRESP3`"]
pub type CMDRESP3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdresp3(&self) -> CMDRESP3_R {
        CMDRESP3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
