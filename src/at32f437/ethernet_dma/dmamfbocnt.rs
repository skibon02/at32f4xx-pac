#[doc = "Register `DMAMFBOCNT` reader"]
pub type R = crate::R<DMAMFBOCNT_SPEC>;
#[doc = "Field `MFC` reader - Missed frames control\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type MFC_R = crate::FieldReader<u16>;
#[doc = "Overflow bit for missed frame counter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBMFC_A {
    #[doc = "0: No overflow of MFC field"]
    NoOverflow = 0,
    #[doc = "1: Overflow of MFC field. The MFC field is reset to 0 when this bit is set"]
    Overflow = 1,
}
impl From<OBMFC_A> for bool {
    #[inline(always)]
    fn from(variant: OBMFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBMFC` reader - Overflow bit for missed frame counter\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type OBMFC_R = crate::BitReader<OBMFC_A>;
impl OBMFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBMFC_A {
        match self.bits {
            false => OBMFC_A::NoOverflow,
            true => OBMFC_A::Overflow,
        }
    }
    #[doc = "No overflow of MFC field"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OBMFC_A::NoOverflow
    }
    #[doc = "Overflow of MFC field. The MFC field is reset to 0 when this bit is set"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OBMFC_A::Overflow
    }
}
#[doc = "Field `OFC` reader - Overflow frame counter\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type OFC_R = crate::FieldReader<u16>;
#[doc = "Overflow bit for FIFO overflow counter\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBFOC_A {
    #[doc = "0: No overflow of OFC field"]
    NoOverflow = 0,
    #[doc = "1: Overflow of OFC field. The OFC field is reset to 0 when this bit is set"]
    Overflow = 1,
}
impl From<OBFOC_A> for bool {
    #[inline(always)]
    fn from(variant: OBFOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBFOC` reader - Overflow bit for FIFO overflow counter\n\n<div class=\"warning\">The field is <b>cleared</b> (set to zero) following a read operation.</div>"]
pub type OBFOC_R = crate::BitReader<OBFOC_A>;
impl OBFOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBFOC_A {
        match self.bits {
            false => OBFOC_A::NoOverflow,
            true => OBFOC_A::Overflow,
        }
    }
    #[doc = "No overflow of OFC field"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OBFOC_A::NoOverflow
    }
    #[doc = "Overflow of OFC field. The OFC field is reset to 0 when this bit is set"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OBFOC_A::Overflow
    }
}
impl R {
    #[doc = "Bits 0:15 - Missed frames control"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn obmfc(&self) -> OBMFC_R {
        OBMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow frame counter"]
    #[inline(always)]
    pub fn ofc(&self) -> OFC_R {
        OFC_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn obfoc(&self) -> OBFOC_R {
        OBFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMFBOCNT").finish()
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamfbocnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMFBOCNT_SPEC;
impl crate::RegisterSpec for DMAMFBOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamfbocnt::R`](R) reader structure"]
impl crate::Readable for DMAMFBOCNT_SPEC {}
#[doc = "`reset()` method sets DMAMFBOCNT to value 0"]
impl crate::Resettable for DMAMFBOCNT_SPEC {}
