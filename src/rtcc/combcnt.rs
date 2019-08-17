#[doc = "Reader of register COMBCNT"]
pub type R = crate::R<u32, super::COMBCNT>;
#[doc = "Reader of field `PRECNT`"]
pub type PRECNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `CNTLSB`"]
pub type CNTLSB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:31 - Counter Value"]
    #[inline(always)]
    pub fn cntlsb(&self) -> CNTLSB_R {
        CNTLSB_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
}
