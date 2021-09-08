/* Constants */
use crate::MemoryStatus::{FreePage, BusyPage};

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
    page_num_mem: Vec<i32>,
}
impl PMT {
    pub fn init(num_pages: u32) -> PMT {
        let mut job_num_vec = vec![];
        let mut page_num_mem_vec = vec![];
        for x in 0..num_pages {
            job_num_vec.push(x);
        }
        for x in 0..num_pages {
            page_num_mem_vec.push(-1);
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
    pub fn insert_job(&mut self, memory: &mut Memory) {
        if self.page_num_mem.len() < memory.available_pages as usize { // we have enough free pages
            for i in 0..NUM_PAGES as usize {
                if memory.status[i] == FreePage {
                    memory.status[i] = BusyPage;
                    memory.available_pages -= 1;
                    self.page_num_mem[i] = -1;
                }
            }
        }
        else {
            // TODO: handle memory is full
        }
    }
    pub fn remove_job(&mut self, memory: &mut Memory) {
        for i in &self.page_num_mem{
            memory.status[*i as usize] = FreePage;
            memory.available_pages += 1;
            self.page_num_mem[*i as usize] = -1;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MemoryStatus {
    FreePage,
    BusyPage
}
#[derive(Clone, Debug)]
struct Memory {
    status: Vec<MemoryStatus>,
    pages: Vec<u32>,
    available_pages: u32,
}
impl Memory {
    fn init() -> Memory {
        let mut status_vec = vec![];
        let mut pages_vec = vec![];
        for i in 0..NUM_PAGES {
            status_vec.push(FreePage);
        }
        for i in 0..NUM_PAGES {
            pages_vec.push(PAGE_SIZE);
        }
        Memory {
            status: status_vec,
            pages: pages_vec,
            available_pages: NUM_PAGES,
        }
    }
}


fn main() {
    let mut jobs: Vec<Job> = Vec::with_capacity(10); // initialize a vector with capacity 10
    let mut memory: Memory = Memory::init();
    jobs.push(Job::init(1, 20000));
    let mut curr_job = jobs.pop().unwrap();
    curr_job.pmt.insert_job(&mut memory);

}

