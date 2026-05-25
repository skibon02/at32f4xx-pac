#[doc = "Register `VMLB` reader"]
pub type R = crate::R<VMLB_SPEC>;
#[doc = "Register `VMLB` writer"]
pub type W = crate::W<VMLB_SPEC>;
#[doc = "Field `VMB` reader - Voltage monitoring low boundary"]
pub type VMB_R = crate::FieldReader<u16>;
#[doc = "Field `VMB` writer - Voltage monitoring low boundary"]
pub type VMB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Voltage monitoring low boundary"]
    #[inline(always)]
    pub fn vmb(&self) -> VMB_R {
        VMB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMLB").field("vmb", &self.vmb()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Voltage monitoring low boundary"]
    #[inline(always)]
    pub fn vmb(&mut self) -> VMB_W<'_, VMLB_SPEC> {
        VMB_W::new(self, 0)
    }
}
#[doc = "Voltage monitoring low boundary register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMLB_SPEC;
impl crate::RegisterSpec for VMLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmlb::R`](R) reader structure"]
impl crate::Readable for VMLB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmlb::W`](W) writer structure"]
impl crate::Writable for VMLB_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VMLB to value 0"]
impl crate::Resettable for VMLB_SPEC {}
