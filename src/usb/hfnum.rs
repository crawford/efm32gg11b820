#[doc = "Reader of register HFNUM"]
pub type R = crate::R<u32, super::HFNUM>;
#[doc = "Reader of field `FRNUM`"]
pub type FRNUM_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRREM`"]
pub type FRREM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame Number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame Time Remaining"]
    #[inline(always)]
    pub fn frrem(&self) -> FRREM_R {
        FRREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
