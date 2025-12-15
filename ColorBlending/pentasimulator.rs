mod rgb;
mod pentagen;
fn main(){
    let target=pentagen::pentagen();
    let mut color=[rgb::Rgb::BLACK;5];
    loop{
        for i in 0..5{
            if color[i]==target[i]{
                println!("Block {}: {} / {} *",i,color[i],target[i]);
            }else{
                println!("Block {}: {} / {}",i,color[i],target[i]);
            }
        }
        if color==target{
            break;
        }
        let mut input=String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts:Vec<&str>=input.trim().split_whitespace().collect();
        if parts.len()!=3{
            println!("Please enter three values: <index> <color> <alpha>");
            continue;
        }
        let index:usize=match parts[0].parse(){
            Ok(1)=>1,
            Ok(2)=>2,
            Ok(3)=>3,
            Ok(4)=>4,
            _=>{
                println!("Invalid index: {}",parts[0]);
                continue;
            }
        };
        let over_color=match parts[1].to_lowercase().as_str(){
            "k"|"black"=>rgb::Rgb::BLACK,
            "r"|"red"=>rgb::Rgb::RED,
            "g"|"green"=>rgb::Rgb::GREEN,
            "b"|"blue"=>rgb::Rgb::BLUE,
            "c"|"cyan"=>rgb::Rgb::CYAN,
            "m"|"magenta"=>rgb::Rgb::MAGENTA,
            "y"|"yellow"=>rgb::Rgb::YELLOW,
            "w"|"white"=>rgb::Rgb::WHITE,
            _=>{
                println!("Unknown color: {}",parts[1]);
                continue;
            }
        };
        let alpha:u32=match parts[2].parse(){
            Ok(v)=>v,
            Err(_)=>{
                println!("Invalid alpha value: {}",parts[2]);
                continue;
            }
        };
        if alpha>rgb::Rgb::STEPS as u32{
            println!("Alpha must be between 0 and 1000");
            continue;
        }
        color[0].over(over_color,alpha);
        color[index].over(over_color,alpha);
    }
}
