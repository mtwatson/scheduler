// Each process has an id, a runtime remaining, and a time slice
// We'll stick all of the processes in the queue and then simulate executing them as so:
// set simulated_time to zero
// while the queue is not empty:
// 	dequeue a process
// 	get the minimum of that process's runtime_remaining or time_slice. Let's call it 	run_time.
// 	add run_time to simulated time
//     subtract run_time from that process's runtime_remaining
//     if that process's runtime_remaining is greater than zero, enqueue the process at the end.
// repeat the loop until queue is empty.

// while debugging it might be helpful to output what process you dequeued, what run_time was, and whether the process finishedor was preempted and put back on the queue

// lets keep track of when (in simulated_time) a process completes and we can use that to write test cases.

// Processes
// 	id:1, remaining: 23, slice: 5
// 	id:2, remaining: 29, slice 7
// Expected Results:
// 	id:1, completion time: 51
// 	id:2, completion time: 52

use std::collections::VecDeque;
#[derive (Debug)]
struct Process {
    id: i32,
    remaining: i32,
    slice: i32,
}

fn main() {
    let mut simulated_time: i32 = 0;
    let mut processes: VecDeque<Process> = VecDeque::new();
    processes.push_back(Process {id:1, remaining: 23, slice: 5});
    processes.push_back(Process {id:2, remaining: 29, slice: 7});

    while let Some(mut front) = processes.pop_front() {
        let slice = front.slice;
        let id = front.id;
        let run_time = if slice < front.remaining {slice} else {front.remaining};

        simulated_time += run_time;
        front.remaining -= run_time;
        println!("{:?}", front);
        if front.remaining > 0 {
            processes.push_back(front);
        } else {
            println!("{id} has completed at {simulated_time}");
        }

    }
}
