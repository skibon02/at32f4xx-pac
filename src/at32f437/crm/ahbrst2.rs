#[doc = "Register `AHBRST2` reader"]
pub type R = crate::R<AHBRST2_SPEC>;
#[doc = "Register `AHBRST2` writer"]
pub type W = crate::W<AHBRST2_SPEC>;
#[doc = "DVP reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DVPW_A {
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<DVPW_A> for bool {
    #[inline(always)]
    fn from(variant: DVPW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DVP` reader - DVP reset"]
pub type DVP_R = crate::BitReader<DVPW_A>;
impl DVP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DVPW_A> {
        match self.bits {
            true => Some(DVPW_A::Reset),
            _ => None,
        }
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DVPW_A::Reset
    }
}
#[doc = "Field `DVP` writer - DVP reset"]
pub type DVP_W<'a, REG> = crate::BitWriter1S<'a, REG, DVPW_A>;
impl<'a, REG> DVP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DVPW_A::Reset)
    }
}
#[doc = "Field `OTGFS1` reader - OTGFS1 reset"]
pub use DVP_R as OTGFS1_R;
#[doc = "Field `SDIO1` reader - SDIO1 reset"]
pub use DVP_R as SDIO1_R;
#[doc = "Field `OTGFS1` writer - OTGFS1 reset"]
pub use DVP_W as OTGFS1_W;
#[doc = "Field `SDIO1` writer - SDIO1 reset"]
pub use DVP_W as SDIO1_W;
impl R {
    #[doc = "Bit 0 - DVP reset"]
    #[inline(always)]
    pub fn dvp(&self) -> DVP_R {
        DVP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 reset"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST2")
            .field("dvp", &self.dvp())
            .field("otgfs1", &self.otgfs1())
            .field("sdio1", &self.sdio1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP reset"]
    #[inline(always)]
    pub fn dvp(&mut self) -> DVP_W<'_, AHBRST2_SPEC> {
        DVP_W::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1(&mut self) -> OTGFS1_W<'_, AHBRST2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 reset"]
    #[inline(always)]
    pub fn sdio1(&mut self) -> SDIO1_W<'_, AHBRST2_SPEC> {
        SDIO1_W::new(self, 15)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8081;
}
#[doc = "`reset()` method sets AHBRST2 to value 0"]
impl crate::Resettable for AHBRST2_SPEC {}
