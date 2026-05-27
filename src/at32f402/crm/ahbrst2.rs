#[doc = "Register `AHBRST2` reader"]
pub type R = crate::R<AHBRST2_SPEC>;
#[doc = "Register `AHBRST2` writer"]
pub type W = crate::W<AHBRST2_SPEC>;
#[doc = "OTGFS1 reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFS1W_A {
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<OTGFS1W_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFS1W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTGFS1` reader - OTGFS1 reset"]
pub type OTGFS1_R = crate::BitReader<OTGFS1W_A>;
impl OTGFS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OTGFS1W_A> {
        match self.bits {
            true => Some(OTGFS1W_A::Reset),
            _ => None,
        }
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFS1W_A::Reset
    }
}
#[doc = "Field `OTGFS1` writer - OTGFS1 reset"]
pub type OTGFS1_W<'a, REG> = crate::BitWriter1S<'a, REG, OTGFS1W_A>;
impl<'a, REG> OTGFS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OTGFS1W_A::Reset)
    }
}
impl R {
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST2")
            .field("otgfs1", &self.otgfs1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1(&mut self) -> OTGFS1_W<'_, AHBRST2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
}
#[doc = "AHB peripheral reset register 2 (CRM_AHBRST2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST2_SPEC;
impl crate::RegisterSpec for AHBRST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst2::R`](R) reader structure"]
impl crate::Readable for AHBRST2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst2::W`](W) writer structure"]
impl crate::Writable for AHBRST2_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x80;
}
#[doc = "`reset()` method sets AHBRST2 to value 0"]
impl crate::Resettable for AHBRST2_SPEC {}
