use png::{BitDepth, ColorType};

pub trait Overdraw<Another> {
    fn overdraw_on(&self, rhs: &mut Another);
}

pub trait ColorFn {

    const PNG_COLOR_TYPE: ColorType;
    const PNG_BIT_DEPTH: BitDepth;
    const BYTE_LENGTH: usize;

    fn to_raw_bytes(self) -> [u8; Self::BYTE_LENGTH];

}



#[derive(Clone, Copy, PartialEq)]
pub struct Rgb8 {

    pub r: u8,
    pub g: u8,
    pub b: u8,

}

impl Rgb8 {

    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {

        Self { r, g, b }
    }

}

impl ColorFn for Rgb8 {
    const PNG_COLOR_TYPE: ColorType = ColorType::Rgb;
    const PNG_BIT_DEPTH: BitDepth = BitDepth::Eight;
    const BYTE_LENGTH: usize = 3;

    fn to_raw_bytes(self) -> [u8; Self::BYTE_LENGTH] {

        [
            self.r,
            self.g,
            self.b,
        ]
    }
}

impl Overdraw<Rgb8> for Rgb8 {
    fn overdraw_on(&self, rhs: &mut Rgb8) {
        *rhs = *self;
    }
}

impl Overdraw<Rgba8> for Rgb8 {
    fn overdraw_on(&self, rhs: &mut Rgba8) {
        *rhs = (*self).into();
    }
}

impl Overdraw<Grey8> for Rgb8 {
    fn overdraw_on(&self, rhs: &mut Grey8) {
        *rhs = (*self).into();
    }
}

impl From<Rgba8> for Rgb8 {
    fn from(value: Rgba8) -> Self {
        Rgb8::new(value.r, value.g, value.b)
    }
}

impl From<Grey8> for Rgb8 {
    fn from(value: Grey8) -> Self {
        Rgb8::new(value.0, value.0, value.0)
    }
}



#[derive(Clone, Copy, PartialEq)]
pub struct Rgba8 {

    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,

}

impl Rgba8 {

    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {

        Self { r, g, b, a }
    }


    pub fn weighed_r(&self) -> u16 {

        self.r as u16 * self.a as u16
    }

    pub fn weighed_g(&self) -> u16 {

        self.g as u16 * self.a as u16
    }

    pub fn weighed_b(&self) -> u16 {

        self.b as u16 * self.a as u16
    }

}

impl ColorFn for Rgba8 {
    const PNG_COLOR_TYPE: ColorType = ColorType::Rgba;
    const PNG_BIT_DEPTH: BitDepth = BitDepth::Eight;
    const BYTE_LENGTH: usize = 4;

    fn to_raw_bytes(self) -> [u8; Self::BYTE_LENGTH] {

        [
            self.r,
            self.g,
            self.b,
            self.a,
        ]
    }
}

impl Overdraw<Rgb8> for Rgba8 {
    fn overdraw_on(&self, rhs: &mut Rgb8) {

        rhs.r = ((self.r as i32  -  rhs.r as i32)  *  self.a as i32  /  0xff  +  rhs.r as i32) as u8;
        rhs.g = ((self.g as i32  -  rhs.g as i32)  *  self.a as i32  /  0xff  +  rhs.g as i32) as u8;
        rhs.b = ((self.b as i32  -  rhs.b as i32)  *  self.a as i32  /  0xff  +  rhs.b as i32) as u8;

    }
}

impl Overdraw<Rgba8> for Rgba8 {
    fn overdraw_on(&self, rhs: &mut Rgba8) {

        rhs.r = ((self.r as i32  -  rhs.r as i32)  *  self.a as i32  /  0xff  +  rhs.r as i32) as u8;
        rhs.g = ((self.g as i32  -  rhs.g as i32)  *  self.a as i32  /  0xff  +  rhs.g as i32) as u8;
        rhs.b = ((self.b as i32  -  rhs.b as i32)  *  self.a as i32  /  0xff  +  rhs.b as i32) as u8;
        rhs.a = self.a.checked_add(self.a).unwrap_or(u8::MAX);

    }
}

impl Overdraw<Grey8> for Rgba8 {
    fn overdraw_on(&self, rhs: &mut Grey8) {
        let grey: Grey8 = (*self).into();

        rhs.0 = ((grey.0 as i32  -  rhs.0 as i32)  *  grey.0 as i32  /  0xff  +  rhs.0 as i32) as u8;
    }
}

impl From<Rgb8> for Rgba8 {
    fn from(value: Rgb8) -> Self {
        Rgba8::new(value.r, value.g, value.b, u8::MAX)
    }
}

impl From<Grey8> for Rgba8 {
    fn from(value: Grey8) -> Self {
        Rgba8::new(value.0, value.0, value.0, u8::MAX)
    }
}



pub struct Grey8 ( u8);

impl Grey8 {

    pub const WHITE: Self = Self::new(255);
    pub const BLACK: Self = Self::new(0);

    pub const fn new(grey: u8) -> Self {

        Self ( grey )
    }

}

impl ColorFn for Grey8 {
    const PNG_COLOR_TYPE: ColorType = ColorType::Grayscale;
    const PNG_BIT_DEPTH: BitDepth = BitDepth::Eight;
    const BYTE_LENGTH: usize = 1;

    fn to_raw_bytes(self) -> [u8; Self::BYTE_LENGTH] {

        [self.0]
    }
}

impl Overdraw<Rgb8> for Grey8 {
    fn overdraw_on(&self, rhs: &mut Rgb8) {

        rhs.r = self.0;
        rhs.g = self.0;
        rhs.b = self.0;

    }
}

impl Overdraw<Rgba8> for Grey8 {
    fn overdraw_on(&self, rhs: &mut Rgba8) {

        rhs.r = self.0;
        rhs.g = self.0;
        rhs.b = self.0;
        rhs.a = self.0;

    }
}

impl Overdraw<Grey8> for Grey8 {
    fn overdraw_on(&self, rhs: &mut Grey8) {
        rhs.0 = self.0;
    }
}

impl From<Rgb8> for Grey8 {
    fn from(value: Rgb8) -> Self {
        Self ( ((value.r as u16 + value.g as u16 + value.b as u16) / 3) as u8 )
    }
}

impl From<Rgba8> for Grey8 {
    fn from(value: Rgba8) -> Self {
        let rgb: Rgb8 = value.into();

        rgb.into()
    }
}
