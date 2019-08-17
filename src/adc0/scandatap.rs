#[doc = "Reader of register SCANDATAP"]
pub type R = crate::R<u32, super::SCANDATAP>;
#[doc = "Reader of field `DATAP`"]
pub type DATAP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Conversion Result Data Peek"]
    #[inline(always)]
    pub fn datap(&self) -> DATAP_R {
        DATAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
