/* Includes */
use crate::JobStatus::{JobDone, JobNotDone, JobWaiting, JobRemoved};
use crate::MemoryStatus::{BusyPage, FreePage};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/* Constants */
const PAGE_SIZE: u32 = 4096;
const NUM_PAGES: u32 = 16;

#[derive(Clone, Copy, PartialEq, Debug)]
enum JobStatus {
    JobNotDone,
    JobWaiting,
    JobDone,
    JobRemoved
}
impl Display for JobStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let print: String = match self {
            JobNotDone => String::from("Job Not Done"),
            JobWaiting => String::from("Job Waiting"),
            JobDone => String::from("Job Done"),
            JobRemoved => String::from("Job Removed"),
        };
        write!(f, "{}", print)
    }
}
/* Job struct, provides core functionality for a job */
#[derive(Clone, Debug)]
struct Job {
    job_num: u32,  // job number
    size: u32,     // the amount of memory requested
    page_req: u32, // how many pages the job will need
    progress: u32, // calculates how much of the job is done, updated with do_tick()
    start_time: u32,
    status: JobStatus,
    in_memory: bool,
    pmt: PMT, // the job's page manager
}
impl Job {
    /* simple initializer function for the job */
    pub fn init(number: u32, memory_req: u32) -> Job {
        let page_req = memory_req / PAGE_SIZE + 1;
        Job {
            job_num: number,
            size: memory_req,
            page_req,
            progress: 0,
            start_time: 0,
            status: JobNotDone,
            in_memory: false,
            pmt: PMT::init(page_req),
        }
    }
}
impl Display for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Num: {}\tSize: {}\t#Pages: {}\tProgress: {}\t Status: {}",
            self.job_num, self.size, self.page_req, self.progress, self.status
        )
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
        let mut job_num_vec = vec![]; // empty vectors
        let mut page_num_mem_vec = vec![];
        for x in 0..num_pages {
            // push initialization values
            job_num_vec.push(x);
        }
        for _ in 0..num_pages {
            page_num_mem_vec.push(-1);
        }
        PMT {
            job_page_num: job_num_vec,
            page_num_mem: page_num_mem_vec,
        }
    }

    pub fn insert_job(&mut self, memory: &mut Memory) -> bool {
        if self.page_num_mem.len() < memory.available_pages as usize {
            // we have enough free pages
            let mut j: usize = 0;
            for i in 0..NUM_PAGES as usize {
                // 0..15
                if memory.status[i] == FreePage {
                    // if the page is free
                    memory.status[i] = BusyPage; // tell it it's busy
                    memory.available_pages -= 1; // decrement available pages
                    self.page_num_mem[j] = i as i32; // tell the page manager where the page is
                    j += 1; // keep track of what page we are dealing with
                    if j == self.page_num_mem.len() {
                        return true;
                    } // once all pages are in memory, exit
                }
            }
        }
        false
    }
    pub fn remove_job(&mut self, memory: &mut Memory) {
        let mut j: usize = 0;
        for i in 0..(NUM_PAGES - 1) as usize {
            if self.page_num_mem[j] == i as i32 {
                // if the page number belongs to this job
                memory.status[i] = FreePage; // free it
                memory.available_pages += 1;
                self.page_num_mem[j] = -1; // set the page to a null value
                j += 1; // keep track of the job number
                if j == self.page_num_mem.len() {
                    break;
                } // exit on all pages free
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MemoryStatus {
    // simple enum for memory status
    FreePage,
    BusyPage,
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
        for _ in 0..NUM_PAGES {
            status_vec.push(FreePage);
        }
        for _ in 0..NUM_PAGES {
            pages_vec.push(PAGE_SIZE);
        }
        Memory {
            status: status_vec,
            pages: pages_vec,
            available_pages: NUM_PAGES,
        }
    }
    fn show(&self) {
        for i in 0..NUM_PAGES as usize {
            if self.status[i] == FreePage {
                println!("Page#{} - {}", i, String::from("Free"));
            } else {
                println!("Page#{} - {}", i, String::from("Busy"));
            }
        }
    }
}

/*
I found code for this function at
https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3
from user Yandros. It did not compile initially so I had to change it.
A basic input function.
*/
fn input() -> String {
    let mut return_string = String::new();
    io::stdout().flush().expect("Failure to flush stdout");
    io::stdin()
        .read_line(&mut return_string)
        .expect("Failed to read from stdin");
    return_string
}

// TODO: Bugs | Insert 1 page job with 1 page available

fn main() {
    /* This thread is just treated like a counter, it just counts how many lines of code are
    executed in a given time frame. It's just an arbitrary number I chose. Because program
    execution halts on user input, it has to be a little high for execution time to make any sort of
    sense. */
    let (tx, rx) = mpsc::channel(); // create a new transmiter (tx) and receiver (rx)
    let _ = thread::spawn(move || {
        let mut x: u32 = 0;
        loop {
            x+=333;                              // increment our counter
            tx.send(x.clone()).unwrap();     // send the data off to the main thread
            thread::sleep(Duration::from_nanos(1)); // sleep to hand off execution just in case
        }
    });
    /* Strings used for program info + help */
    let help_str: String = String::from("\n+---------------------------------------------------------------------\n\
                                      | Welcome to the Pager! For help, type '?\n\
                                      |                           --Commands--\n\
                                      | <job number> <bytes> - start a new job with a certain amount of memory\n\
                                      | <job number> 0 - delete a job\n\
                                      | insertw - attempt to insert waiting jobs into memory\n\
                                      | print - display the current memory status\n\
                                      | pjobs - display all jobs and their relevant info\n\
                                      | pjobs-waiting - display all waiting jobs\n\
                                      | pjobs <job number> - display a job's info\n\
                                      | ? - display this prompt\n\
                                      | exit - quit the pager\n\
                                      +---------------------------------------------------------------------\n");
    let info: String = String::from(
        "+-------------------------------------------------------------------------------\n\
    | Welcome to the Pager! I had a bear of a time getting threads to work\n\
    | in rust so I implemented virtual time in this way: every time a command is executed\n\
    | 500 \"lines of code\" are executed on a job. If you want to see how much progress a job\n\
    | a job has made you can use the pjobs command - try '?' for help.\n\
    +-------------------------------------------------------------------------------",
    );
    println!("{}", info);                                           // print off the info string

    let prompt: String = String::from("Pager (? for help)> ");   // prompt string

    /* Beginning of Read, Execute, Print Loop */
    let mut memory: Memory = Memory::init();                        // initialize our dumby memory
    let mut jobs: Vec<Job> = Vec::with_capacity(10);        // vector of jobs to store info about them
    print!("{}", prompt);                                           // display our prompt
    let mut user_input: String = input();
    loop {
        let tokenize = user_input.split_whitespace(); // tokenize the string on whitespace
        let args: Vec<&str> = tokenize.collect();                   // collect the tokens
        if args[0] == "exit" {                                      // check for exit
            break;                                                  // exit the loop
        } else if args[0] == "?" {                                  // check for help
            print!("{}", help_str);                                 // display the help string
        } else if args[0] == "print" {                              // show the pages in memory and if the are busy or not
            memory.show();
        } else if args[0] == "pjobs" {                              // print off specific job information
            /* prints a specific job number */
            if args.len() > 1 {
                if !args[1].to_string().parse::<u32>().is_err() {   // check if we can convert the job number to an int
                    let job_number: u32 = args[1].to_string().parse().unwrap(); // parse it if we can
                    for i in 0..jobs.len() {                  // check for the relevant job
                        if jobs[i].job_num == job_number {
                            println!("{}", jobs[i]);                // print it off
                        }
                    }
                } else {
                    println!("{}", "| Please provide a valid job number\n| Syntax: pjobs <job number (unsigned int)>");
                }
            }
            /* if no job number has been provided, */
            else {
                for i in 0..jobs.len() {                    // otherwise we just print off all the jobs
                    println!("{}", jobs[i]);
                }
            }
        }
        else if args[0] == "insertw" {
            for i in 0..jobs.len() {
                if jobs[i].status == JobWaiting {
                    if jobs[i].pmt.insert_job(&mut memory) {
                        print!("Insert success! Job {} inserted into memory!", jobs[i].job_num);
                        jobs[i].status = JobNotDone;
                        jobs[i].in_memory = true;
                    }
                }

            }
        }
        /* job insert and deletion */
        else if !args[0].to_string().parse::<usize>().is_err()
            && !args[1].to_string().parse::<u32>().is_err()
        {
            let job_number: usize = args[0].to_string().parse().unwrap(); // grab the job number
            let mem_requested: u32 = args[1].to_string().parse().unwrap(); // grab the size of the job
            let mut new_job = Job::init(job_number as u32, mem_requested); // otherwise we are making a new job
            if mem_requested == 0 {
                // check to make sure if we are deleting it
                for i in 0..jobs.len() {
                    if jobs[i].job_num == job_number as u32 {
                        // search for the correct job
                        jobs[i].pmt.remove_job(&mut memory); // remove it when we find it
                        jobs[i].in_memory = false;
                        jobs[i].status = JobRemoved;
                    }
                }
            } else {
                if new_job.pmt.insert_job(&mut memory) {            // if we can successfully insert the job into memory
                    new_job.in_memory = true;
                    new_job.start_time = rx.recv().unwrap();        // set our start execution time
                    jobs.push(new_job);                       // push the job onto the jobs vector
                } else {
                    let mut remove_job = jobs.pop().unwrap();   // retrieve the first job inserted into memory
                    remove_job.pmt.remove_job(&mut memory);
                    remove_job.in_memory = false;
                    remove_job.status = JobWaiting;
                    jobs.push(remove_job);
                }
            }
        } else {
            println!("{}", "| Not a valid command, please try '?' for help.");
        }
        for i in 0..jobs.len() {
            /* calculate how many "lines of code" have been executed on each job */
            if jobs[i].status == JobNotDone {
                jobs[i].progress = rx.recv().unwrap() - jobs[i].start_time;
                if jobs[i].progress > jobs[i].size {
                    jobs[i].status = JobDone;
                    jobs[i].pmt.remove_job(&mut memory);
                    jobs[i].progress = jobs[i].size;
                }
            }
        }
        print!("{}", prompt);
        user_input = input(); // grab new user input
    } /* End Loop */

}
