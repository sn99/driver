#![no_std]
#![feature(alloc_error_handler)]

pub mod allocator;

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;