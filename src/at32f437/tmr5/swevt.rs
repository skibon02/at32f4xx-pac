#[doc = "Register `SWEVT` reader"]
pub type R = crate::R<SWEVT_SPEC>;
#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SWEVT_SPEC>;
#[doc = "Overflow event triggered by software\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFSWTRW_A {
    #[doc = "1: Generate an overflow event"]
    Overflow = 1,
}
impl From<OVFSWTRW_A> for bool {
    #[inline(always)]
    fn from(variant: OVFSWTRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFSWTR` reader - Overflow event triggered by software"]
pub type OVFSWTR_R = crate::BitReader<OVFSWTRW_A>;
impl OVFSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVFSWTRW_A> {
        match self.bits {
            true => Some(OVFSWTRW_A::Overflow),
            _ => None,
        }
    }
    #[doc = "Generate an overflow event"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OVFSWTRW_A::Overflow
    }
}
#[doc = "Field `OVFSWTR` writer - Overflow event triggered by software"]
pub type OVFSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, OVFSWTRW_A>;
impl<'a, REG> OVFSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate an overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(OVFSWTRW_A::Overflow)
    }
}
#[doc = "Field `C1SWTR` reader - Channel 1 event triggered by software"]
pub type C1SWTR_R = crate::BitReader;
#[doc = "Field `C1SWTR` writer - Channel 1 event triggered by software"]
pub type C1SWTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2SWTR` reader - Channel 2 event triggered by software"]
pub type C2SWTR_R = crate::BitReader;
#[doc = "Field `C2SWTR` writer - Channel 2 event triggered by software"]
pub type C2SWTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3SWTR` reader - Channel 3 event triggered by software"]
pub type C3SWTR_R = crate::BitReader;
#[doc = "Field `C3SWTR` writer - Channel 3 event triggered by software"]
pub type C3SWTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4SWTR` reader - Channel 4 event triggered by software"]
pub type C4SWTR_R = crate::BitReader;
#[doc = "Field `C4SWTR` writer - Channel 4 event triggered by software"]
pub type C4SWTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSWTR` reader - Trigger event triggered by software"]
pub type TRGSWTR_R = crate::BitReader;
#[doc = "Field `TRGSWTR` writer - Trigger event triggered by software"]
pub type TRGSWTR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    pub fn ovfswtr(&self) -> OVFSWTR_R {
        OVFSWTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    pub fn c1swtr(&self) -> C1SWTR_R {
        C1SWTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    pub fn c2swtr(&self) -> C2SWTR_R {
        C2SWTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    pub fn c3swtr(&self) -> C3SWTR_R {
        C3SWTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    pub fn c4swtr(&self) -> C4SWTR_R {
        C4SWTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    pub fn trgswtr(&self) -> TRGSWTR_R {
        TRGSWTR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWEVT")
            .field("trgswtr", &self.trgswtr())
            .field("c4swtr", &self.c4swtr())
            .field("c3swtr", &self.c3swtr())
            .field("c2swtr", &self.c2swtr())
            .field("c1swtr", &self.c1swtr())
            .field("ovfswtr", &self.ovfswtr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    pub fn ovfswtr(&mut self) -> OVFSWTR_W<'_, SWEVT_SPEC> {
        OVFSWTR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    pub fn c1swtr(&mut self) -> C1SWTR_W<'_, SWEVT_SPEC> {
        C1SWTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    pub fn c2swtr(&mut self) -> C2SWTR_W<'_, SWEVT_SPEC> {
        C2SWTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    pub fn c3swtr(&mut self) -> C3SWTR_W<'_, SWEVT_SPEC> {
        C3SWTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    pub fn c4swtr(&mut self) -> C4SWTR_W<'_, SWEVT_SPEC> {
        C4SWTR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    pub fn trgswtr(&mut self) -> TRGSWTR_W<'_, SWEVT_SPEC> {
        TRGSWTR_W::new(self, 6)
    }
}
#[doc = "Software event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swevt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swevt::R`](R) reader structure"]
impl crate::Readable for SWEVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {}
