#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `PPUPRIV`"]
pub type PPUPRIV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PPUPRIV_R {
        PPUPRIV_R::new((self.bits & 0x01) != 0)
    }
}
