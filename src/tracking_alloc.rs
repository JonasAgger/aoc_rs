use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::AtomicUsize,
};

pub struct TrackingAllocator;

impl TrackingAllocator {
    #[allow(dead_code)]
    pub fn delta() -> isize {
        let alloc = ALLOC.load(std::sync::atomic::Ordering::SeqCst);
        let dealloc = DEALLOC.load(std::sync::atomic::Ordering::SeqCst);
        (alloc as isize) - (dealloc as isize)
    }

    #[allow(dead_code)]
    pub fn alloced() -> usize {
        ALLOC.load(std::sync::atomic::Ordering::SeqCst)
    }

    #[allow(dead_code)]
    pub fn wat() -> usize {
        ALLOC.load(std::sync::atomic::Ordering::SeqCst)
    }
}

static ALLOC: AtomicUsize = AtomicUsize::new(0);
static DEALLOC: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        record_alloc(layout);
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        record_dealloc(layout);
        unsafe { System.dealloc(ptr, layout) };
    }
}

fn record_alloc(layout: Layout) {
    ALLOC.fetch_add(layout.size(), std::sync::atomic::Ordering::SeqCst);
}

fn record_dealloc(layout: Layout) {
    DEALLOC.fetch_add(layout.size(), std::sync::atomic::Ordering::SeqCst);
}
