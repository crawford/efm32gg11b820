#[doc = "Reader of register RST"]
pub type R = crate::R<u32, super::RST>;
#[doc = "Writer for register RST"]
pub type W = crate::W<u32, super::RST>;
#[doc = "Register RST `reset()`'s with value 0"]
impl crate::ResetValue for super::RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
