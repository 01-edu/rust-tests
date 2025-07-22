use std::rc::Rc;

use drop_the_thread::*;

#[test]
fn tests_drops_are_correct() {
    let worker = ThreadPool::new();
    let (pid, thread) = worker.new_thread(String::from("gnome-shell"));
    let (pid0, thread0) = worker.new_thread(String::from("i3"));
    let (pid1, thread1) = worker.new_thread(String::from("shell"));
    let (pid2, thread2) = worker.new_thread(String::from("spotify"));

    thread.skill();
    assert_eq!(worker.drops.get(), 1);
    thread0.skill();

    assert!(worker.is_dropped(pid), "{} should have been dropped", pid);
    assert!(worker.is_dropped(pid0), "{} should have been dropped", pid0);
    assert!(
        !worker.is_dropped(pid1),
        "{} should not have been dropped",
        pid1
    );
    assert!(
        !worker.is_dropped(pid2),
        "{} should not have been dropped",
        pid2
    );

    assert_eq!(worker.drops.get(), 2);

    thread1.skill();
    thread2.skill();

    assert_eq!(worker.drops.get(), 4);
}

#[test]
fn test_drops_only_happen_without_further_references() {
    let worker = ThreadPool::new();
    let (_, thread) = worker.new_thread(String::from("Xorg"));
    let thread = Rc::new(thread);

    {
        let _thread_clone = Rc::clone(&thread);
    }

    assert_eq!(worker.drops.get(), 0);
}

#[test]
#[should_panic(expected = "0 is already dropped")]
fn test_dropping_dropped_thread_panics() {
    let worker = ThreadPool::new();
    let (_pid, thread) = worker.new_thread(String::from("gsd-rfkill"));
    thread.skill();

    let thread_clone = Thread {
        pid: 0,
        cmd: "gsd-rfkill".to_owned(),
        parent: &worker,
    };
    thread_clone.skill();
}
