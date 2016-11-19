# ppipe
An Elegantly Simple and Lightweight Library for Making Iterator Pipelines Concurrent and Amazingly Fast, Hence the Name "ppipe" (Parallel Pipe).


## Usage
Put this in your Cargo.toml:
```toml
[dependencies]

ppipe = "0.1.0"
```

Then add this to your root module:
```rust
extern crate ppipe;
```

And add this to whatever module you want to be able to use ppipe:
```rust
use ppipe::*;
```

Now generally all iterators in scope should have the `ppipe` method!


## Overview
If you followed the above steps, every iterator should have the `ppipe` method. The method takes an `Option<usize>` parameter which can be used to declare if you want back-pressure or not. `ppipe(Some(1000))` would mean that you want the concurrent receiver to hold no more than 1000 values and tell the sender to block until the receiver's buffer goes below 1000 over the course of, for example, a `for` loop.


## Example
```rust
extern crate ppipe;
use ppipe::*;

// ...

for item in excel_sheet.into_iter()
  .map(do_something)
  .write_out_err(some_err_handling_func)
  .ppipe(None)  // create a thread for the above work
  .map(do_something_else)
  .ppipe(None) // create another thread for the the above work
  // ...
{
  // ...
}
```

## How It Works Internally
The significance of this little library is hard to put into words, so please refer to the **ppipe_example.rs** in the *examples* folder of this repository, which I will reference in this explanation.

The point of this simplistic example is to demonstrate how WITHOUT `ppipe`, every iteration moves the corresponding item along the pipeline and then executes, in this case, the `for` loop's body. The items are never pre-loaded into some buffer waiting for the iteration variable to take ownership after being forced to move along the pipeline. This is almost never idealistic; why limit yourself to serial processing when you have Rust's powerful parallel processing at hand? This is what `ppipe` does. WITH `ppipe`, all previous iterator adaptors are ran regardless of what iteration, in this case, the `for` loop is on, including any previous `ppipe` adaptors which are busy doing their own thing. Every item that is processed is put in a buffer which can be accessed as it is being added to, and if there are no items in the buffer, the iteration will simply block until an item is available, or break if there are no more items being processed. This means items can be added to the buffer as you are iterating over previous items in the buffer, which ultimately reduces bottlenecking and GREATLY increases performance!

