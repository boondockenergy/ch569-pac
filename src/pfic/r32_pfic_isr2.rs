#[doc = "Register `R32_PFIC_ISR2` reader"]
pub struct R(crate::R<R32_PFIC_ISR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_PFIC_ISR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_PFIC_ISR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_PFIC_ISR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTENSTA` reader - Interrupt ID Status"]
pub type INTENSTA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - Interrupt ID Status"]
    #[inline(always)]
    pub fn intensta(&self) -> INTENSTA_R {
        INTENSTA_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_pfic_isr2](index.html) module"]
pub struct R32_PFIC_ISR2_SPEC;
impl crate::RegisterSpec for R32_PFIC_ISR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_pfic_isr2::R](R) reader structure"]
impl crate::Readable for R32_PFIC_ISR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R32_PFIC_ISR2 to value 0"]
impl crate::Resettable for R32_PFIC_ISR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
