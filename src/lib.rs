extern crate crossbeam;
use crossbeam::sync::MsQueue;
use std::sync::{ Arc, Mutex };

unsafe impl<T: Send> Send for Senpai<T> {}
unsafe impl<T: Send> Sync for Senpai<T> {}
pub struct Senpai<T>
    where T: Send
{
    queue: Arc<MsQueue<T>>,
    done: Arc<Mutex<bool>>
}

unsafe impl<T: Send> Send for Kohai<T> {}
unsafe impl<T: Send> Sync for Kohai<T> {}
pub struct Kohai<T>
    where T: Send
{
    queue: Arc<MsQueue<T>>,
    done: Arc<Mutex<bool>>
}

impl<T: Send> Senpai<T> {
    /// Find out that Senpai is at your school
    pub fn exists() -> Self {
        Self {
            queue: Arc::new(MsQueue::new()),
            done: Arc::new(Mutex::new(false))
        }
    }

    /// Be greeted by your Senpai and realize you're a Kohai of theirs!
    pub fn hello_kohai(&self) -> Kohai<T> {
        Kohai {
            queue: self.queue.clone(),
            done:  self.done.clone()
        }
    }

    /// Senpai makes sure their Kohai have gifts to make them happy
    pub fn this_is_for_my_kohai(&self, gift: T) {
        self.queue.push(gift);
    }

    /// I'm sorry my Kohai. I need to move on to the next stage of my life
    pub fn im_graduating_my_kohai(&self) {
        // We're guaranteeing none of the kohai
        // will panic over their senpai
        *self.done.lock().unwrap() = true;
    }
}

impl<T: Send> Kohai<T> {
    /// Will Senpai get me a gift?
    pub fn notice_me_senpai(&self) -> Option<T> {
        self.queue.try_pop()
    }
    /// Has Senpai graduated without me?
    pub fn senpai_is_gone(&self) -> bool {
        // We're guaranteeing senpai didn't
        // panic over there kohai behind
        *self.done.lock().unwrap()
    }
}

#[test]
fn senpai_kun() {
    use std::thread;
    let senpai = Senpai::exists();

    let minami_chan  = senpai.hello_kohai();
    let minami_chan_join = thread::spawn(move || {
        loop {
            if let Some(x) = minami_chan.notice_me_senpai() {
                println!("It's me Minami. Senpai game me: {}", x);
            } else {
                if minami_chan.senpai_is_gone() {
                    break;
                }
            }
            thread::sleep_ms(1000);
        }
    });

    let mirai_chan   = senpai.hello_kohai();
    let mirai_chan_join = thread::spawn(move || {
        loop {
            if let Some(x) = mirai_chan.notice_me_senpai() {
                println!("It's me Mirai. Senpai game me: {}", x);
            } else {
                if mirai_chan.senpai_is_gone() {
                    break;
                }
            }
            thread::sleep_ms(1000);
        }
    });

    let setsuna_chan = senpai.hello_kohai();
    let setsuna_chan_join = thread::spawn(move || {
        loop {
            if let Some(x) = setsuna_chan.notice_me_senpai() {
                println!("It's me Setsuna. Senpai game me: {}", x);
            } else {
                if setsuna_chan.senpai_is_gone() {
                    break;
                }
            }
            thread::sleep_ms(1000);
        }
    });

    println!("I have so many wonderful gifts for my Kohai");
    senpai.this_is_for_my_kohai(1);
    senpai.this_is_for_my_kohai(2);
    senpai.this_is_for_my_kohai(3);
    senpai.this_is_for_my_kohai(4);
    senpai.this_is_for_my_kohai(5);
    senpai.this_is_for_my_kohai(6);
    senpai.this_is_for_my_kohai(7);
    senpai.this_is_for_my_kohai(8);
    senpai.this_is_for_my_kohai(9);
    senpai.this_is_for_my_kohai(10);
    senpai.this_is_for_my_kohai(11);
    senpai.this_is_for_my_kohai(12);
    senpai.this_is_for_my_kohai(13);
    senpai.this_is_for_my_kohai(14);
    senpai.this_is_for_my_kohai(15);
    senpai.this_is_for_my_kohai(16);
    senpai.this_is_for_my_kohai(17);

    println!("I have to graduate now my Kohai.");
    senpai.im_graduating_my_kohai();

    let _ = setsuna_chan_join.join().unwrap();
    let _ = mirai_chan_join.join().unwrap();
    let _ = minami_chan_join.join().unwrap();
}
