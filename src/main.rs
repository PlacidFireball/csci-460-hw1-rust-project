/* Includes */
use std::io;
use std::io::*;
use crate::MemoryStatus::{FreePage, BusyPage};
/* Constants */
const PAGE_SIZE: u32 = 4096;
const NUM_PAGES: u32 = 16;

/* Job struct, provides core functionality for a job */
#[derive(Clone, Debug)]
struct Job {
    job_num: u32,   // job number
    size: u32,      // the amount of memory requested
    page_req: u32,  // how many pages the job will need
    pmt: PMT        // the job's page manager
}
impl Job {
    /* simple initializer function for the job */
    pub fn init(number: u32, memory_req: u32) -> Job {
        let mut page_req = memory_req/PAGE_SIZE+1;
        Job {
            job_num: number,
            size: memory_req,
            page_req,
            pmt: PMT::init(page_req),
        }
    }
    /* Job information printed off to the console */
    pub fn show(&self) {
        print!("Num: {}\tSize: {}\t#Pages: {}\n", self.job_num, self.size, self.page_req);
    }
}

/* PMT struct, provides core page management features for a Job */
#[derive(Clone, Debug)]
struct PMT {
    job_page_num: Vec<u32>, // page numbers for the job {0, 1, 2 .... n}
    page_num_mem: Vec<i32>, // page numbers in memory, {-1 (error state), 0, 1, 2 ... NUM_PAGES}
}
impl PMT {
    /* simple initializer function for the page manager */
    pub fn init(num_pages: u32) -> PMT {
        let mut job_num_vec = vec![];   // empty vectors
        let mut page_num_mem_vec = vec![];
        for x in 0..num_pages { // push initialization values
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
    /* show function for the page manager to print debug info to the console*/
    pub fn show(&self) {
        for i in &self.job_page_num {
            print!("JP#: {}\t ML: {}\n", i, self.page_num_mem[(*i as usize)]);
        }
    }
    pub fn insert_job(&mut self, memory: &mut Memory) {
        if self.page_num_mem.len() < memory.available_pages as usize { // we have enough free pages
            let mut j: usize = 0;
            for i in 0..(NUM_PAGES-1)as usize { // 0..15
                if memory.status[i] == FreePage {   // if the page is free
                    memory.status[i] = BusyPage;    // tell it it's busy
                    memory.available_pages -= 1;    // decrement available pages
                    self.page_num_mem[j] = i as i32;    // tell the page manager where the page is
                    j += 1;     // keep track of what page we are dealing with
                    if j == self.page_num_mem.len() { break; } // once all pages are in memory, exit
                }
            }
        }
        else {
            // TODO: handle memory is full
        }
    }
    pub fn remove_job(&mut self, memory: &mut Memory) {
        let mut j: usize = 0;
        for i in 0..(NUM_PAGES-1) as usize {
            if self.page_num_mem[j] == i as i32 { // if the page number belongs to this job
                memory.status[i] = FreePage;    // free it
                self.page_num_mem[j] = -1;  // set the page to a null value
                j += 1; // keep track of the job number
                if j == self.page_num_mem.len() { break; } // exit on all pages free
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MemoryStatus { // simple enum for memory status
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
    fn show(&self) {
        for i in 0..(NUM_PAGES-1) as usize{
            if self.status[i] == FreePage { println!("Page#{} - {}", i, String::from("Free")); }
            else { println!("Page#{} - {}", i, String::from("Busy")); }
        }
    }
}

/*
I found code for this function at
https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3
from user Yandros. It did not compile initially so I had to change it.
A basic input function.
*/
fn input (message: &String) -> String {
    print!("{}", *message);
    let mut return_string = String::new();
    io::stdin().read_line(&mut return_string).expect("Failed to read from stdin");
    return_string
}

fn main() {
    let HELP_STR: String = String::from("\nWelcome to the Pager! For help, type '?
                                      \n--Commands--
                                      \n<job number> <bytes> - start a new job with a certain amount of memory
                                      \n<job number> 0 - delete a job
                                      \nprint - display the current memory status
                                      \n? - display this prompt
                                      \nexit - quit the pager");
    let PROMPT: String = String::from("\nPager (? for help)>");
    let mut jobs: [Job; 20] = [Job::init(0, 0); 20];
    let mut memory: Memory = Memory::init();

    /* Beginning of Read, Execute, Print Loop */
    let mut user_input: String = input(&PROMPT);
    while true {
        let mut tokenize = user_input.split(" ");
        let mut args: Vec<&str> = tokenize.collect();
        if args[0] == "?" {
            print!("{}", HELP_STR);
        }
        else if args[0] == "print" {
            memory.show();
        }
        else {
            let mut job_number: u32 = args[0].into_string().parse().unwrap();
            let mut mem_requested: u32 = args[1].into_string().parse().unwrap();
            if mem_requested == 0 {
                jobs[job_number].pmt.remove_job(&mut memory);
            }
            let mut new_job = Job::init(job_number, mem_requested);
            new_job.pmt.insert_job(&mut memory);
            jobs[job_number] = new_job;
        }



        user_input = input(&PROMPT);
    }

}

