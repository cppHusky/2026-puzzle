#[derive(Debug,Clone,Copy)]
pub struct Rgb{
    pub r:u8,
    pub g:u8,
    pub b:u8,
}
impl Rgb{
    pub const BLACK:Self=Rgb{r:0,g:0,b:0};
    pub const RED:Self=Rgb{r:255,g:0,b:0};
    pub const GREEN:Self=Rgb{r:0,g:255,b:0};
    pub const BLUE:Self=Rgb{r:0,g:0,b:255};
    pub const CYAN:Self=Rgb{r:0,g:255,b:255};
    pub const MAGENTA:Self=Rgb{r:255,g:0,b:255};
    pub const YELLOW:Self=Rgb{r:255,g:255,b:0};
    pub const WHITE:Self=Rgb{r:255,g:255,b:255};
    pub const STEPS:f64=10000.0;
    pub fn rand()->Self{
        Rgb{
            r:rand::random::<u8>(),
            g:rand::random::<u8>(),
            b:rand::random::<u8>(),
        }
    }
    pub fn from(r:u8,g:u8,b:u8)->Self{
        Rgb{r,g,b}
    }
    pub fn over(&mut self,other:Self,alpha:u32){
        let alpha=alpha as f64/Self::STEPS;
        self.r=((other.r as f64)*alpha+(self.r as f64)*(1.0-alpha)).round_ties_even()as u8;
        self.g=((other.g as f64)*alpha+(self.g as f64)*(1.0-alpha)).round_ties_even()as u8;
        self.b=((other.b as f64)*alpha+(self.b as f64)*(1.0-alpha)).round_ties_even()as u8;
    }
    pub fn min(&self)->u8{
        self.r.min(self.g.min(self.b))
    }
    pub fn max(&self)->u8{
        self.r.max(self.g.max(self.b))
    }
}
impl PartialEq for Rgb{
    fn eq(&self,other:&Self)->bool{
        self.r==other.r && self.g==other.g && self.b==other.b
    }
}
impl std::fmt::Display for Rgb{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        let luma=0.2126*(self.r as f64)+0.7152*(self.g as f64)+0.0722*(self.b as f64);
        if luma<144.0{
            write!(f,"\x1b[48;2;{r};{g};{b}m\x1b[38;2;255;255;255m#{r:02X}{g:02X}{b:02X}\x1b[0m",r=self.r,g=self.g,b=self.b)
        }else{
            write!(f,"\x1b[48;2;{r};{g};{b}m\x1b[38;2;0;0;0m#{r:02X}{g:02X}{b:02X}\x1b[0m",r=self.r,g=self.g,b=self.b)
        }
    }
}
