#[doc = "Reader of register SYNCBUSY"]
pub type R = crate::R<u32, super::SYNCBUSY>;
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 7 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
