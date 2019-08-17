#[doc = "Reader of register BUFDATA"]
pub type R = crate::R<u32, super::BUFDATA>;
#[doc = "Reader of field `BUFDATA`"]
pub type BUFDATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `BUFDATASRC`"]
pub type BUFDATASRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Result Data"]
    #[inline(always)]
    pub fn bufdata(&self) -> BUFDATA_R {
        BUFDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn bufdatasrc(&self) -> BUFDATASRC_R {
        BUFDATASRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
