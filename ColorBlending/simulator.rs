mod rgb;
fn main(){
    let target=match std::env::args().nth(1){
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
        _=>rgb::Rgb::rand(),
    };
    let mut color=rgb::Rgb::BLACK;
    loop{
        println!("{} / {}",color,target);
        if color==target{
            break;
        }
        let mut input=String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts:Vec<&str>=input.trim().split_whitespace().collect();
        if parts.len()!=2{
            println!("Please enter two values: <color> <alpha>");
            continue;
        }
        let over_color=match parts[0].to_lowercase().as_str(){
            "k"|"black"=>rgb::Rgb::BLACK,
            "r"|"red"=>rgb::Rgb::RED,
            "g"|"green"=>rgb::Rgb::GREEN,
            "b"|"blue"=>rgb::Rgb::BLUE,
            "c"|"cyan"=>rgb::Rgb::CYAN,
            "m"|"magenta"=>rgb::Rgb::MAGENTA,
            "y"|"yellow"=>rgb::Rgb::YELLOW,
            "w"|"white"=>rgb::Rgb::WHITE,
            _=>{
                println!("Unknown color: {}",parts[0]);
                continue;
            }
        };
        let alpha:u32=match parts[1].parse(){
            Ok(v)=>v,
            Err(_)=>{
                println!("Invalid alpha value: {}",parts[1]);
                continue;
            }
        };
        if alpha>rgb::Rgb::STEPS as u32{
            println!("Alpha must be between 0 and 1000");
            continue;
        }
        color.over(over_color,alpha);
    }
}
