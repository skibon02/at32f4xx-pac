#[doc = "Register `SER_ID` reader"]
pub type R = crate::R<SER_ID_SPEC>;
#[doc = "version ID\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IDR_A {
    #[doc = "0: Revision A"]
    A = 0,
}
impl From<REV_IDR_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IDR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_IDR_A {
    type Ux = u8;
}
impl crate::IsEnum for REV_IDR_A {}
#[doc = "Field `REV_ID` reader - version ID"]
pub type REV_ID_R = crate::FieldReader<REV_IDR_A>;
impl REV_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REV_IDR_A> {
        match self.bits {
            0 => Some(REV_IDR_A::A),
            _ => None,
        }
    }
    #[doc = "Revision A"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == REV_IDR_A::A
    }
}
#[doc = "Field `SER_ID` reader - series ID"]
pub type SER_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - version ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - series ID"]
    #[inline(always)]
    pub fn ser_id(&self) -> SER_ID_R {
        SER_ID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SER_ID")
            .field("rev_id", &self.rev_id())
            .field("ser_id", &self.ser_id())
            .finish()
    }
}
#[doc = "SERIES ID\n\nYou can [`read`](crate::Reg::read) this register and get [`ser_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SER_ID_SPEC;
impl crate::RegisterSpec for SER_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser_id::R`](R) reader structure"]
impl crate::Readable for SER_ID_SPEC {}
#[doc = "`reset()` method sets SER_ID to value 0"]
impl crate::Resettable for SER_ID_SPEC {}
