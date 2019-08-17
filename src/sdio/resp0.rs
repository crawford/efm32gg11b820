#[doc = "Reader of register RESP0"]
pub type R = crate::R<u32, super::RESP0>;
#[doc = "Reader of field `CMDRESP0`"]
pub type CMDRESP0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 0"]
    #[inline(always)]
    pub fn cmdresp0(&self) -> CMDRESP0_R {
        CMDRESP0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
