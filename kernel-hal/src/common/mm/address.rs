use crate::config::*;
use core::fmt::{self, Debug, Formatter};

use super::page_table::PageTableEntry;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct VirtPageNum(pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct PhysPageNum(pub usize);

impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl From<usize> for VirtPageNum {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {
        v.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {
        v.0
    }
}

impl From<VirtAddr> for usize {
    fn from(v: VirtAddr) -> Self {
        v.0
    }
}

impl From<VirtPageNum> for usize {
    fn from(v: VirtPageNum) -> Self {
        v.0
    }
}

impl PhysAddr {
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe { (self.0 as *mut T).as_mut().unwrap() }
    }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl PhysPageNum {
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let addr: PhysAddr = self.clone().into();
        unsafe { core::slice::from_raw_parts_mut(addr.0 as *mut u8, 4096) }
    }

    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let addr: PhysAddr = self.clone().into();
        unsafe { core::slice::from_raw_parts_mut(addr.0 as *mut PageTableEntry, 512) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let addr: PhysAddr = self.clone().into();
        unsafe { (addr.0 as *mut T).as_mut().unwrap() }
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }

    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev() {
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}

impl From<VirtAddr> for VirtPageNum {
    fn from(v: VirtAddr) -> VirtPageNum {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtPageNum> for VirtAddr {
    fn from(v: VirtPageNum) -> VirtAddr {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

impl Debug for VirtPageNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VPN:{:#x}", self.0))
    }
}

pub trait StepByOne {
    fn step(&mut self);
}

impl StepByOne for VirtPageNum {
    fn step(&mut self) {
        self.0 += 1;
    }
}

#[derive(Clone, Copy)]
pub struct SimpleRange<T>
where
    T: StepByOne + PartialEq + PartialOrd + Copy + Debug,
{
    l: T,
    r: T,
}

impl<T> SimpleRange<T>
where
    T: StepByOne + PartialEq + PartialOrd + Copy + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }

    pub fn get_start(&self) -> T {
        self.l
    }

    pub fn get_end(&self) -> T {
        self.r
    }
}

pub struct SimpleRangeIterator<T>
where
    T: StepByOne + PartialEq + PartialOrd + Copy + Debug,
{
    current: T,
    end: T,
}

impl<T> SimpleRangeIterator<T>
where
    T: StepByOne + PartialEq + PartialOrd + Copy + Debug,
{
    fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}

impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + PartialEq + PartialOrd + Copy + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}

impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + PartialEq + PartialOrd + Copy + Debug,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator::new(self.l, self.r)
    }
}

pub type VPNRange = SimpleRange<VirtPageNum>;
