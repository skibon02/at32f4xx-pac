#[doc = "Register `CM1_INPUT` reader"]
pub type R = crate::R<CM1_INPUT_SPEC>;
#[doc = "Register `CM1_INPUT` writer"]
pub type W = crate::W<CM1_INPUT_SPEC>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1C_R = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1C_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1IDIV` reader - Channel 1 input divider"]
pub type C1IDIV_R = crate::FieldReader;
#[doc = "Field `C1IDIV` writer - Channel 1 input divider"]
pub type C1IDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1DF` reader - Channel 1 digital filter"]
pub type C1DF_R = crate::FieldReader;
#[doc = "Field `C1DF` writer - Channel 1 digital filter"]
pub type C1DF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2C` reader - Channel 2 configure"]
pub type C2C_R = crate::FieldReader;
#[doc = "Field `C2C` writer - Channel 2 configure"]
pub type C2C_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C2IDIV` reader - Channel 2 input divider"]
pub type C2IDIV_R = crate::FieldReader;
#[doc = "Field `C2IDIV` writer - Channel 2 input divider"]
pub type C2IDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C2DF` reader - Channel 2 digital filter"]
pub type C2DF_R = crate::FieldReader;
#[doc = "Field `C2DF` writer - Channel 2 digital filter"]
pub type C2DF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1C_R {
        C1C_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&self) -> C1IDIV_R {
        C1IDIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&self) -> C1DF_R {
        C1DF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&self) -> C2C_R {
        C2C_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    pub fn c2idiv(&self) -> C2IDIV_R {
        C2IDIV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    pub fn c2df(&self) -> C2DF_R {
        C2DF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM1_INPUT")
            .field("c2df", &self.c2df())
            .field("c2idiv", &self.c2idiv())
            .field("c2c", &self.c2c())
            .field("c1df", &self.c1df())
            .field("c1idiv", &self.c1idiv())
            .field("c1c", &self.c1c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&mut self) -> C1C_W<'_, CM1_INPUT_SPEC> {
        C1C_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&mut self) -> C1IDIV_W<'_, CM1_INPUT_SPEC> {
        C1IDIV_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&mut self) -> C1DF_W<'_, CM1_INPUT_SPEC> {
        C1DF_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 2 configure"]
    #[inline(always)]
    pub fn c2c(&mut self) -> C2C_W<'_, CM1_INPUT_SPEC> {
        C2C_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 2 input divider"]
    #[inline(always)]
    pub fn c2idiv(&mut self) -> C2IDIV_W<'_, CM1_INPUT_SPEC> {
        C2IDIV_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 2 digital filter"]
    #[inline(always)]
    pub fn c2df(&mut self) -> C2DF_W<'_, CM1_INPUT_SPEC> {
        C2DF_W::new(self, 12)
    }
}
#[doc = "Channel input mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM1_INPUT_SPEC;
impl crate::RegisterSpec for CM1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_input::R`](R) reader structure"]
impl crate::Readable for CM1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm1_input::W`](W) writer structure"]
impl crate::Writable for CM1_INPUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CM1_INPUT to value 0"]
impl crate::Resettable for CM1_INPUT_SPEC {}
