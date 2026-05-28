#[doc = "Register `MACA0H` reader"]
pub type R = crate::R<MACA0H_SPEC>;
#[doc = "Register `MACA0H` writer"]
pub type W = crate::W<MACA0H_SPEC>;
#[doc = "Field `MA0H` reader - MAC address0 high"]
pub type MA0H_R = crate::FieldReader<u16>;
#[doc = "Field `MA0H` writer - MAC address0 high"]
pub type MA0H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn ma0h(&self) -> MA0H_R {
        MA0H_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0H")
            .field("ma0h", &self.ma0h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn ma0h(&mut self) -> MA0H_W<'_, MACA0H_SPEC> {
        MA0H_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 0 high register. This register changed to 16 bits wide, because the only available another field is bit 31 - AE, but it is restricted to be always 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`maca0h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maca0h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0H_SPEC;
impl crate::RegisterSpec for MACA0H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0h::R`](R) reader structure"]
impl crate::Readable for MACA0H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca0h::W`](W) writer structure"]
impl crate::Writable for MACA0H_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACA0H to value 0xffff"]
impl crate::Resettable for MACA0H_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
