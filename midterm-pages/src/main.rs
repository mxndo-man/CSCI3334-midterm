use std::collections::HashMap;
use std::fs;

//BOOKBAL is the assigned value in the assignment
struct LoadPages {
    days_load: Vec<Vec<u64>>,
}

impl LoadPages {
    const BOOKVAL: u64 = 64;
    const DAYAMOUNT: f64 = 32.0;
    const BITVAL: i32 = 100000;

    pub fn load_f() -> Self {
        let mut days_load = Vec::new();

        for day in 1..=32 {
            let input = fs::read_to_string(format!("./workloads/day_{day:02}/loads")).unwrap();
            days_load.push(Self::parse(&input));
        }
        LoadPages { days_load }
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

    pub fn solve(&self) -> Vec<(i32, u64)> {
        let mut penatly_scores: HashMap<i32, u64> = HashMap::new();

        for i in 1..=Self::BITVAL {
            let mut _penatly: u64 = 0;

            for day in &self.days_load {
                _penatly += self.get_pen(day, i);
            }

            // let average = _penatly / (Self::DAYAMOUNT as u64);
            penatly_scores.insert(i, _penatly);
        }
        // push the hash map into a vector which will get the top 3 that producewd the least amount
        // of penalty

        let mut sorted_scores: Vec<(&i32, &u64)> = penatly_scores.iter().collect();

        sorted_scores.sort_by_key(|entry| entry.1);
        sorted_scores
            .into_iter()
            .take(3)
            .map(|(k, v)| (*k, *v))
            .collect()
    }

    pub fn print(&self, best_page: i32) {
        // Essentailly this get the score of the smallest penatly.
        // it iwll then compute the the bookkeeping bits penalty.
        // get the internal fragmentaion penalty
        // the total penalty for that day.
        println!("min page size {}", best_page);
        let mut overall_penatly: u64 = 0;
        for (i, day) in self.days_load.iter().enumerate() {
            // This is just a repeat of get_pen and solve() but for just one day.
            let mut day_bk: u64 = 0;
            let mut day_un: u64 = 0;
            let len_prog = day.len();

            for &req in day {
                let p = req.div_ceil(best_page as u64);
                let bk_costt = p * Self::BOOKVAL;
                let un_cost = (p * best_page as u64) - req;

                day_bk += bk_costt;
                day_un += un_cost;
            }
            let day_penalty = day_bk + day_un;
            overall_penatly += day_penalty;

            println!(
                "Day {:02}| {:>3} programs | bk bits {:>8} |internal frag {:>9} | total Pen {:>7}",
                i + 1,
                len_prog,
                day_bk,
                day_un,
                day_penalty
            );
        }

        let avg_pen = (overall_penatly as f64) / Self::DAYAMOUNT as f64;
        println!("The average penatly was {}" , avg_pen);

    }
}

fn main() {
    // Main function: restructed compared to the initial commit history.
    // This version will be used for clean and readability code
    let run_sol = LoadPages::load_f();

    let top_num = run_sol.solve();

    for (i, (bit_num, total_pen)) in top_num.iter().enumerate() {
        let avg_pen = (*total_pen as f64) / LoadPages::DAYAMOUNT;
        println!(
            "{}. Bit Nums: {} | Average Pen: {:.2}",
            i + 1,
            bit_num,
            avg_pen
        );
    }

    let (best_bit, _) = top_num[0];
    run_sol.print(best_bit);
}
