#[doc = "Reader of register SCANDATAXP"]
pub type R = crate::R<u32, super::SCANDATAXP>;
#[doc = "Reader of field `DATAP`"]
pub type DATAP_R = crate::R<u16, u16>;
#[doc = "Reader of field `SCANINPUTIDPEEK`"]
pub type SCANINPUTIDPEEK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Scan Conversion Data Source Peek"]
    #[inline(always)]
    pub fn scaninputidpeek(&self) -> SCANINPUTIDPEEK_R {
        SCANINPUTIDPEEK_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
