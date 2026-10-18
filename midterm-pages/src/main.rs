use std::collections::HashMap;
use std::fs;

//BOOKBAL is the assigned value in the assignment
struct LoadPages{
    days_load: Vec<Vec<u64>>,
}

impl LoadPages{
    const BOOKVAL: u64 = 64;
    const DAYAMOUNT: f64 = 32.0;
    const BITVAL: i32 = 100000;

    pub fn load_f() -> Self{
        let mut days_load = Vec::new();

        for day in 1..=32 {
            let input = fs::read_to_string(format!("./workloads/day_{day:02}/loads")).unwrap();
            days_load.push(Self::parse(&input));
        }
        LoadPages{days_load}
    }

    pub fn parse(input: &str) -> Vec<u64> {
        //Returns a simple Vec<u64>

        //basic docs but will update later
        //Using parse function from AOC day 1: What I made was just a simple parser for u64 for the
        //loads
        let mut page_request: Vec<u64> = vec![];
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let amount: u64 = line.parse().unwrap();
            page_request.push(amount);
        }
        page_request
    }
    pub fn get_pen(&self, page_request: &[u64], bit_page: i32) -> u64 {
        //get_pen argruments
        //page_request a pass by refernce of a slice for the loads
        //bit_page: the "perfect value"
        //returns type u64
        let mut _pen: u64 = 0;
        for item in page_request {
            //iterate over the vector
            //pages: is value of the vector with ceiling division with rounding up
            let pages: u64 = item.div_ceil(bit_page as u64);
            //_pen = pages * const bookVal + we convert bit_page as u64 as the iterator cannot pass be
            //converted from u32 in fn main(), - a defrence of the item as u64
            _pen += (pages * Self::BOOKVAL) + ((pages * bit_page as u64) - *item as u64);
        }

        _pen
    }

    pub fn solve(&self) -> (i32,u64){

        let mut penatly_scores: HashMap<i32, u64> = HashMap::new();

        for i in 1..= Self::BITVAL {
            let mut _penatly: u64 = 0;

            for day in &self.days_load{ _penatly += self.get_pen(day, i); }

            let average = _penatly / (Self::DAYAMOUNT as u64);
            penatly_scores.insert(i, average);
        }

        let (_key, _value) = penatly_scores.iter().min_by_key(|entry| entry.1).unwrap();
        (*_key,*_value)
    }

}



fn main() {
    let run_sol = LoadPages::load_f();

    let (bit_num, avg_pen) = run_sol.solve();
    println!("Bit num:{} \n Penataly_score: {} ",bit_num,avg_pen);




}
