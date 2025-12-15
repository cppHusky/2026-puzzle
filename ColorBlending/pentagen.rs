use crate::rgb;
extern crate rand;
use rand::seq::SliceRandom;
pub fn pentagen()->[rgb::Rgb;5]{
    const MIN_PIGMENT:u8=0x60;
    const MAX_PIGMENT:u8=0xa0;
    let standard_color=(
        rand::random_range(MIN_PIGMENT..=MAX_PIGMENT),
        rand::random_range(MIN_PIGMENT..=MAX_PIGMENT),
        rand::random_range(MIN_PIGMENT..=MAX_PIGMENT),
    );
    let mut rng=rand::rng();
    let mut reference_color=[
        (
            rand::random_range(0..standard_color.0),
            rand::random_range(standard_color.1+1..=255),
            rand::random_range(standard_color.2+1..=255),
        ),
        (
            rand::random_range(standard_color.0+1..=255),
            rand::random_range(0..standard_color.1),
            rand::random_range(standard_color.2+1..=255),
        ),
        (
            rand::random_range(standard_color.0+1..=255),
            rand::random_range(standard_color.1+1..=255),
            rand::random_range(0..standard_color.2),
        ),
        (
            rand::random_range(0..standard_color.0),
            rand::random_range(0..standard_color.1),
            rand::random_range(0..standard_color.2),
        ),
    ];
    reference_color.shuffle(&mut rng);
    [
        match std::env::args().nth(1){
            Some(color)=>{
                let color_hex=color.to_lowercase();
                if color_hex.len()!=6{
                    eprintln!("Color must be a 6-digit hex code (e.g. FF0000)");
                    std::process::exit(1);
                }
                let r=u8::from_str_radix(&color_hex[0..2],16).unwrap_or(0);
                let g=u8::from_str_radix(&color_hex[2..4],16).unwrap_or(0);
                let b=u8::from_str_radix(&color_hex[4..6],16).unwrap_or(0);
                rgb::Rgb::from(r,g,b)
            }
            _=>rgb::Rgb::from(standard_color.0,standard_color.1,standard_color.2),
        },
        match std::env::args().nth(2){
            Some(color)=>{
                let color_hex=color.to_lowercase();
                if color_hex.len()!=6{
                    eprintln!("Color must be a 6-digit hex code (e.g. FF0000)");
                    std::process::exit(1);
                }
                let r=u8::from_str_radix(&color_hex[0..2],16).unwrap_or(0);
                let g=u8::from_str_radix(&color_hex[2..4],16).unwrap_or(0);
                let b=u8::from_str_radix(&color_hex[4..6],16).unwrap_or(0);
                rgb::Rgb::from(r,g,b)
            }
            _=>rgb::Rgb::from(reference_color[0].0,reference_color[0].1,reference_color[0].2),
        },
        match std::env::args().nth(3){
            Some(color)=>{
                let color_hex=color.to_lowercase();
                if color_hex.len()!=6{
                    eprintln!("Color must be a 6-digit hex code (e.g. FF0000)");
                    std::process::exit(1);
                }
                let r=u8::from_str_radix(&color_hex[0..2],16).unwrap_or(0);
                let g=u8::from_str_radix(&color_hex[2..4],16).unwrap_or(0);
                let b=u8::from_str_radix(&color_hex[4..6],16).unwrap_or(0);
                rgb::Rgb::from(r,g,b)
            }
            _=>rgb::Rgb::from(reference_color[1].0,reference_color[1].1,reference_color[1].2),
        },
        match std::env::args().nth(4){
            Some(color)=>{
                let color_hex=color.to_lowercase();
                if color_hex.len()!=6{
                    eprintln!("Color must be a 6-digit hex code (e.g. FF0000)");
                    std::process::exit(1);
                }
                let r=u8::from_str_radix(&color_hex[0..2],16).unwrap_or(0);
                let g=u8::from_str_radix(&color_hex[2..4],16).unwrap_or(0);
                let b=u8::from_str_radix(&color_hex[4..6],16).unwrap_or(0);
                rgb::Rgb::from(r,g,b)
            }
            _=>rgb::Rgb::from(reference_color[2].0,reference_color[2].1,reference_color[2].2),
        },
        match std::env::args().nth(5){
            Some(color)=>{
                let color_hex=color.to_lowercase();
                if color_hex.len()!=6{
                    eprintln!("Color must be a 6-digit hex code (e.g. FF0000)");
                    std::process::exit(1);
                }
                let r=u8::from_str_radix(&color_hex[0..2],16).unwrap_or(0);
                let g=u8::from_str_radix(&color_hex[2..4],16).unwrap_or(0);
                let b=u8::from_str_radix(&color_hex[4..6],16).unwrap_or(0);
                rgb::Rgb::from(r,g,b)
            }
            _=>rgb::Rgb::from(reference_color[3].0,reference_color[3].1,reference_color[3].2),
        },
    ]
}
