#[doc = "Reader of register TFTPIXEL"]
pub type R = crate::R<u32, super::TFTPIXEL>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Alpha Blending Result"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
