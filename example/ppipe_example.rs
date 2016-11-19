/* The point of this simplistic example is to demonstrate how WITHOUT ppipe, every iteration moves
 * the corresponding item along the pipeline and then executes, in this case, the `for` loop's
 * body. The items are never pre-loaded into some buffer waiting for the iteration variable to take
 * ownership after being forced to move along the pipeline. This is almost never idealistic; why
 * limit yourself to serial processing when you have Rust's powerful parallel processing at hand?
 * This is what ppipe does. WITH ppipe, all previous iterator adaptors are ran regardless of what
 * iteration, in this case, the `for` loop is on, including any previous ppipe adaptors which are 
 * busy doing their own thing. Every item that is processed is put in a buffer which can be 
 * accessed as it is being added to, and if there are no items in the buffer, the iteration will 
 * simply block until an item is available, or break if there are no more items being processed. 
 * This means items can be added to the buffer as you are iterating over previous items in the 
 * buffer, which ultimately reduces bottlenecking and GREATLY increases performance!
 */


extern crate ppipe;


use ppipe::*;
use std::thread::sleep;
use std::time::Duration;


#[allow(unused_variables)]
fn main() {
    println!("Without ppipe usage:\n");
    for v in (0..10).map(|x| { println!("not multitasking"); })
                    /* .filter..., .take..., etc... */
    {
        sleep(Duration::from_millis(2000));
        println!("Iteration completed");
    }
    println!("\n");

    println!("With ppipe usage:\n");
    for v in (0..10).map(|x| { println!("multitasking"); })
                    .ppipe(None)
                    /* .filter..., .take..., etc... */
    {
        sleep(Duration::from_millis(2000));
        println!("Iteration completed");
    }
    println!("\n");
}
