#[doc = "Reader of register SCANDATAX"]
pub type R = crate::R<u32, super::SCANDATAX>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `SCANINPUTID`"]
pub type SCANINPUTID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Input ID"]
    #[inline(always)]
    pub fn scaninputid(&self) -> SCANINPUTID_R {
        SCANINPUTID_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
