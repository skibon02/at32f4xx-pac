#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `DRS` reader - DMA request source"]
pub type DRS_R = crate::BitReader;
#[doc = "Field `DRS` writer - DMA request source"]
pub type DRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTOS` reader - Primary TMR output selection"]
pub type PTOS_R = crate::FieldReader;
#[doc = "Field `PTOS` writer - Primary TMR output selection"]
pub type PTOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C1INSEL` reader - C1IN selection"]
pub type C1INSEL_R = crate::BitReader;
#[doc = "Field `C1INSEL` writer - C1IN selection"]
pub type C1INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DRS_R {
        DRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&self) -> PTOS_R {
        PTOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    pub fn c1insel(&self) -> C1INSEL_R {
        C1INSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("c1insel", &self.c1insel())
            .field("ptos", &self.ptos())
            .field("drs", &self.drs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&mut self) -> DRS_W<'_, CTRL2_SPEC> {
        DRS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&mut self) -> PTOS_W<'_, CTRL2_SPEC> {
        PTOS_W::new(self, 4)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    pub fn c1insel(&mut self) -> C1INSEL_W<'_, CTRL2_SPEC> {
        C1INSEL_W::new(self, 7)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {}
