#[doc = "Reader of register DATABYTEREV"]
pub type R = crate::R<u32, super::DATABYTEREV>;
#[doc = "Reader of field `DATABYTEREV`"]
pub type DATABYTEREV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Byte Reverse Value"]
    #[inline(always)]
    pub fn databyterev(&self) -> DATABYTEREV_R {
        DATABYTEREV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
