//! # ppipe
//!
//! An elegantly simple and lightweight library for making iterator pipelines concurrent and 
//! amazingly fast, hence the name "ppipe" (parallel pipe). 


#![deny(missing_docs)]


use std::sync::mpsc;
use std::thread;


/// This trait does all the work for you so that generally every iterator you use has the ppipe 
/// method. Make sure to include this trait for it to take effect:
///
/// ```no_run
///
/// extern crate ppipe;
/// use ppipe::*;
///
/// ```
pub trait PPipe: Iterator {
    /// This method can be called on generally any iterator, making every previous task become part
    /// of a concurrent pipeline. 
    ///
    /// `ppipe` takes an `Option<usize>` parameter which can be used to declare if you want 
    /// back-pressure or not. `ppipe(Some(1000))` would mean that you want the concurrent receiver to 
    /// hold no more than 1000 values and tell the sender to block until the receiver's buffer goes 
    /// below 1000 over the course of, for example, a `for` loop.
    fn ppipe(self, back_pressure: Option<usize>) -> mpsc::IntoIter<Self::Item>;
}


impl<T> PPipe for T
    where T: Iterator + Send + 'static,
          T::Item: Send + 'static
{
    fn ppipe(self, back_pressure: Option<usize>) -> mpsc::IntoIter<Self::Item> {
        if back_pressure.is_some() {
            let (sender, receiver) = mpsc::sync_channel(back_pressure.unwrap());

            thread::spawn(move || {
                for item in self {
                    if sender.send(item).is_err() {
                        break;
                    }
                }
            });

            receiver.into_iter()
        } else {
            let (sender, receiver) = mpsc::channel();

            thread::spawn(move || {
                for item in self {
                    if sender.send(item).is_err() {
                        break;
                    }
                }
            });

            receiver.into_iter()
        }
    }
}
