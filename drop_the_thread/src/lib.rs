/*
## drop

### Instructions

"Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data"

You must create a Drop checker API. For this you must create:

- Two structures:
  - `Workers` that will have two fields:
    - `drops` that will save the number of dropped threads
    - `states` that will save a state of multiple threads.
      If the thread is not dropped, the state will be false otherwise true.
  - `Thread` that will have the following fields:
    - `pid`, the id of the thread
    - `cmd`, the name of the thread
    - `parent`, that will be the link to the structure `Workers` (Tip: this must be a reference to the structure Workers)

- Implementation of each structure:
  - `Workers` :
    - `new`, that creates a default worker
    - `new_worker`, that returns a tuple with the `pid` and a new `Thread`,
      this function must receive a `String` being the `cmd`
      - `is_dropped`, that receives a `pid` and returns a `bool` that indicates the state of the thread by using the `pid`
      - `track_worker`, it should return a `usize`, that will be the last available index of the `states` vector, being the new next thread
    - `add_drop`, this function must be **called by the `Drop` trait**. It will receive a `pid` that will be used to change the
      state of the thread. If the state of that thread is `true` then it will panic with the message ("Cannot drop {}, because its already dropped", pid).
      Otherwise it should change the state to true and increment the `drops` field by one.

  - `Thread`:
    - `new_thread`, that initializes a new thread
    - `skill`, that drops the thread

- You must implement for the structure `Thread` the `Drop` trait. In this trait you must call the function `add_drop` so that the state of the thread changes

### Notions

- https://doc.bccnsoft.com/docs/rust-1.36.0-docs-html/std/ops/trait.Drop.html
- https://doc.rust-lang.org/std/cell/struct.RefCell.html
- https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

*/

use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers::default()
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let g = Thread::new_thread(self.track_worker(), c, self);
        self.states.borrow_mut().push(false);
        (g.pid, g)
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{:?} is already dropped", id)
        }
        self.states.borrow_mut()[id] = true;
        self.drops.set(self.drops.get() + 1);
    }

}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub // time: (String, u16),
    parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread { pid: p, cmd: c , parent: t }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}

/*
use std::rc::Rc;

fn main() {
    let worker = Workers::new();
    let (id, thread) = worker.new_worker(String::from("command"));
    let (id1, thread1) = worker.new_worker(String::from("command1"));

    thread.skill();

    println!("{:?}", (worker.is_dropped(id), id, &worker.drops));
    // output: (true, 0, Cell { value: 1 })

    thread1.skill();
    println!("{:?}", (worker.is_dropped(id1), id1, &worker.drops));
    // output: (true, 1, Cell { value: 2 })

    let (id2, thread2) = worker.new_worker(String::from("command2"));
    let thread2 = Rc::new(thread2);
    let thread2_clone = thread2.clone();

    // thread2_clone.skill();
    drop(thread2_clone);

    println!("{:?}", (worker.is_dropped(id2), id2, &worker.drops, Rc::strong_count(&thread2)));
    // (false, 2, Cell { value: 2 }, 1)
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_is_dropped_and_drops() {
        let worker = Workers::new();
        let (pid, thread) = worker.new_worker(String::from("gnome-shell"));
        let (pid0, thread0) = worker.new_worker(String::from("i3"));
        let (pid1, thread1) = worker.new_worker(String::from("shell"));
        let (pid2, thread2) = worker.new_worker(String::from("spotify"));

       thread.skill();
       assert_eq!(worker.drops.get(), 1_usize);
       thread0.skill();

       assert!(worker.is_dropped(pid), "{} should have been dropped", pid);
       assert!(worker.is_dropped(pid0), "{} should have been dropped", pid0);
       assert!(!worker.is_dropped(pid1), "{} should not have been dropped", pid1);
       assert!(!worker.is_dropped(pid2), "{} should not have been dropped", pid2);

       assert_eq!(worker.drops.get(), 2_usize);

       thread1.skill();
       thread2.skill();

       assert_eq!(worker.drops.get(), 4_usize);
    }

    #[test]
    fn test_using_rc() {
      // will create a new reference to the thread
      // this will test the following
      // if we drop the cloned value the RC will decrease
      // but the thread will not be dropped! 
      let worker = Workers::new();
      let (_, thread) = worker.new_worker(String::from("Xorg"));
      let thread = Rc::new(thread);
      let thread_clone = thread.clone();

      assert_eq!(Rc::strong_count(&thread), 2);

      drop(thread_clone);

      assert_eq!(Rc::strong_count(&thread), 1);
    }

    #[test]
    #[should_panic(expected = "0 is already dropped")]
    fn test_drop_same_thread() {
        // test if we drop the same thread after it was already been dropped
        let worker = Workers::new();
        let (pid, thread) = worker.new_worker(String::from("gsd-rfkill"));
        let thread_clone = thread.clone();
        thread.skill();
        thread_clone.skill();
    }
}
