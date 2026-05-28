#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `SCLL` reader - SCL low level. TSCLL = (SCLL + 1) x (DIV + 1) x TI2C_CLK"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low level. TSCLL = (SCLL + 1) x (DIV + 1) x TI2C_CLK"]
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `SCLH` reader - SCL high level. TSCLH = (SCLH + 1) x (DIV + 1) x TI2C_CLK"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high level. TSCLH = (SCLH + 1) x (DIV + 1) x TI2C_CLK"]
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `SDAD` reader - SDA delay. TSDAD = (SDAD + 1) x (DIV + 1) x TI2C_CLK"]
pub type SDAD_R = crate::FieldReader;
#[doc = "Field `SDAD` writer - SDA delay. TSDAD = (SDAD + 1) x (DIV + 1) x TI2C_CLK"]
pub type SDAD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `SCLD` reader - SCL delay. TSCLD = (SCLD + 1) x (DIV + 1) x TI2C_CLK"]
pub type SCLD_R = crate::FieldReader;
#[doc = "Field `SCLD` writer - SCL delay. TSCLD = (SCLD + 1) x (DIV + 1) x TI2C_CLK"]
pub type SCLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DIVH` reader - Clock prescaler high. DIV = (DIVH << 4) + DIVL"]
pub type DIVH_R = crate::FieldReader;
#[doc = "Field `DIVH` writer - Clock prescaler high. DIV = (DIVH << 4) + DIVL"]
pub type DIVH_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Field `DIVL` reader - Clock prescaler low. DIV = (DIVH << 4) + DIVL"]
pub type DIVL_R = crate::FieldReader;
#[doc = "Field `DIVL` writer - Clock prescaler low. DIV = (DIVH << 4) + DIVL"]
pub type DIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - SCL low level. TSCLL = (SCLL + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high level. TSCLH = (SCLH + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SDA delay. TSDAD = (SDAD + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn sdad(&self) -> SDAD_R {
        SDAD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCL delay. TSCLD = (SCLD + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn scld(&self) -> SCLD_R {
        SCLD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock prescaler high. DIV = (DIVH << 4) + DIVL"]
    #[inline(always)]
    pub fn divh(&self) -> DIVH_R {
        DIVH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Clock prescaler low. DIV = (DIVH << 4) + DIVL"]
    #[inline(always)]
    pub fn divl(&self) -> DIVL_R {
        DIVL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("scll", &self.scll())
            .field("sclh", &self.sclh())
            .field("sdad", &self.sdad())
            .field("scld", &self.scld())
            .field("divh", &self.divh())
            .field("divl", &self.divl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low level. TSCLL = (SCLL + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W<'_, CLKCTRL_SPEC> {
        SCLL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high level. TSCLH = (SCLH + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<'_, CLKCTRL_SPEC> {
        SCLH_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - SDA delay. TSDAD = (SDAD + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn sdad(&mut self) -> SDAD_W<'_, CLKCTRL_SPEC> {
        SDAD_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SCL delay. TSCLD = (SCLD + 1) x (DIV + 1) x TI2C_CLK"]
    #[inline(always)]
    pub fn scld(&mut self) -> SCLD_W<'_, CLKCTRL_SPEC> {
        SCLD_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Clock prescaler high. DIV = (DIVH << 4) + DIVL"]
    #[inline(always)]
    pub fn divh(&mut self) -> DIVH_W<'_, CLKCTRL_SPEC> {
        DIVH_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Clock prescaler low. DIV = (DIVH << 4) + DIVL"]
    #[inline(always)]
    pub fn divl(&mut self) -> DIVL_W<'_, CLKCTRL_SPEC> {
        DIVL_W::new(self, 28)
    }
}
#[doc = "Clock contorl register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {}
