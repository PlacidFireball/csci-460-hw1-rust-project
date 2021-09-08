/* Constants */
const PAGE_SIZE: u32 = 4096;
const NUM_PAGES: u32 = 16;

#[derive(Clone, Debug)]
struct Job {
    job_num: u32,
    size: u32,
    page_req: u32,
    pmt: PMT
}
impl Job {
    pub fn init(number: u32, memory_req: u32) -> Job {
        let mut page_req = memory_req/PAGE_SIZE+1;
        Job {
            job_num: number,
            size: memory_req,
            page_req,
            pmt: PMT::init(page_req),
        }
    }
    pub fn show(&self) {
        print!("Num: {}\tSize: {}\t#Pages: {}\n", self.job_num, self.size, self.page_req);
    }
}

#[derive(Clone, Debug)]
struct PMT {
    job_page_num: Vec<u32>,
    page_num_mem: Vec<u32>,
}
impl PMT {
    pub fn init(num_pages: u32) -> PMT {
        let mut job_num_vec = vec![];
        let mut page_num_mem_vec = vec![];
        for x in 0..num_pages {
            job_num_vec.push(x);
        }
        for x in 0..num_pages {
            page_num_mem_vec.push(0);
        }
        PMT {
            job_page_num: job_num_vec,
            page_num_mem: page_num_mem_vec,
        }
    }
    pub fn show(&self) {
        for i in &self.job_page_num {
            print!("JP#: {}\t ML: {}\n", i, self.page_num_mem[(*i as usize)]);
        }
    }
}

fn main() {
    let mut job: Job = Job::init(0, 20000);
    job.show();
    job.pmt.show();
}

