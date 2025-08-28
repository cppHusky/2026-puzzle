const N:usize=4;
fn main()->Result<(),Box<dyn std::error::Error>>{
    use std::io::Read;
    let mut file=std::fs::File::open("expectation.txt")?;
    let mut expectation=String::new();
    let _=file.read_to_string(&mut expectation);
    eprint!("{}",expectation);
    let expectation=expectation.split_terminator("\n").collect::<Vec<&str>>().into_iter().map(|row|{
        row.split_whitespace().collect::<Vec<&str>>().iter().map(|x|match *x{
            "1"=>true,
            "0"=>false,
            _=>panic!("unknown value: {}",x),
        }).collect::<Vec<bool>>()
    }).collect::<Vec<Vec<bool>>>();
    let strategy=Strategy::new();
    monte_carlo(strategy,expectation);
    Ok(())
}
fn monte_carlo(strategy:Strategy,expectation:Vec<Vec<bool>>){
    let mut counter_matched=0;
    static TOTAL:usize=1000000;
    for _ in 1..TOTAL+1{
        if run_once(&strategy,&expectation){
            counter_matched+=1;
        }
    }
    eprintln!("The monte carlo result is {}/{} ({}‱  )",counter_matched,TOTAL,counter_matched as f64/TOTAL as f64*10000.0);
    eprintln!("If giving totally random values, the probability will be {}‱  )",1.0/(1<<N*N) as f64*10000.0);
}
#[derive(Default,Debug)]
struct Strategy{
    pub p:[f64;N],
    pub q:[f64;N],
}
impl Strategy{
    pub fn new()->Self{
        use std::io::Read;
        let mut file=std::fs::File::open("strategy.txt").expect("Open strategy.txt failed");
        let mut content=String::new();
        let _=file.read_to_string(&mut content);
        let nums:Vec<f64>=content.split_whitespace().map(str::parse).collect::<Result<_,_>>().expect("Parse error");
        Self{
            p:[nums[0],nums[1],nums[2],nums[3]],
            q:[nums[4],nums[5],nums[6],nums[7]],
        }
    }
}
fn run_once(strategy:&Strategy,expectation:&Vec<Vec<bool>>)->bool{
    let mut result=vec![vec![false;N];N];
    for i in 0..N{
        for j in 0..N{
            if rand::random_bool(strategy.p[i])^rand::random_bool(strategy.q[j]){
                result[i][j]=!result[i][j];
            }
        }
    }
    return result==*expectation;
}
