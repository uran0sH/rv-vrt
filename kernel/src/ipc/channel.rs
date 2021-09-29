use alloc::{collections::VecDeque, sync::{Arc, Weak}, vec::Vec};
use spin::Mutex;

use crate::task::PidHandle;

type T = MessagePacket;

pub struct Channel {
    peer: Weak<Channel>,
    recv_queue: Mutex<VecDeque<T>>,
}

#[repr(C)]
pub struct MessagePacket {
    pub data: Vec<u8>,
    pub handle: PidHandle, // use pid as handle temporally
}

impl Channel {
    pub fn create() -> (Arc<Self>, Arc<Self>) {
        let mut channel0 = Arc::new(Channel {
            peer: Weak::default(),
            recv_queue: Default::default(),
        });
        let channel1 = Arc::new(Channel {
            peer: Arc::downgrade(&channel0),
            recv_queue: Default::default(),
        });
        unsafe {
            Arc::get_mut_unchecked(&mut channel0).peer = Arc::downgrade(&channel1);
        }
        (channel0, channel1)
    }

    // Read a packet from the channel
    pub fn read_msg(&self) -> Option<MessagePacket> {
        let mut recv_queue = self.recv_queue.lock();
        if let Some(_msg) = recv_queue.front() {
            let msg = recv_queue.pop_front().unwrap();
            return Some(msg);
        } else {
            None
        }
    }

    // Write a packet to the channel
    pub fn write_msg(&self, msg: T) {
        let peer = self.peer.upgrade().unwrap();
        peer.push_general(msg);
    }

    fn push_general(&self, msg: T) {
        let mut send_queue = self.recv_queue.lock();
        send_queue.push_back(msg);
    }

    #[allow(dead_code)]
    fn peer_closed(&self) -> bool {
        self.peer.strong_count() == 0
    }
}
