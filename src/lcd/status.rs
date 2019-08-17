#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `ASTATE`"]
pub type ASTATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `BLINK`"]
pub type BLINK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Current Animation State"]
    #[inline(always)]
    pub fn astate(&self) -> ASTATE_R {
        ASTATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Blink State"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
