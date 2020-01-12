use x86_64::structures::gdt::*;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{PrivilegeLevel, VirtAddr};
use x86_64::registers::model_specific::Msr;
use x86_64::instructions::segmentation::set_cs;
use x86_64::instructions::tables::load_tss;

const STACK_SIZE: usize = 0x1000;

// Copied from rCore
const KCODE: Descriptor = Descriptor::UserSegment(0x0020980000000000); // EXECUTABLE | USER_SEGMENT | PRESENT | LONG_MODE
const UCODE: Descriptor = Descriptor::UserSegment(0x0020F80000000000); // EXECUTABLE | USER_SEGMENT | USER_MODE | PRESENT | LONG_MODE
const KDATA: Descriptor = Descriptor::UserSegment(0x0000920000000000); // DATA_WRITABLE | USER_SEGMENT | PRESENT
const UDATA: Descriptor = Descriptor::UserSegment(0x0000F20000000000); // DATA_WRITABLE | USER_SEGMENT | USER_MODE | PRESENT

pub const KCODE_SELECTOR: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);
pub const TSS_SELECTOR: SegmentSelector = SegmentSelector::new(5, PrivilegeLevel::Ring0);

static mut CPU: Option<Cpu> = None;

pub struct Cpu {
    gdt: GlobalDescriptorTable,
    tss: TaskStateSegment,
    double_fault_stack: [u8; STACK_SIZE]
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            gdt: GlobalDescriptorTable::new(),
            tss: TaskStateSegment::new(),
            double_fault_stack: [0u8; STACK_SIZE]
        }
    }

    pub fn init(&'static mut self) {
        // Set double fault stack
        self.tss.interrupt_stack_table[0] = VirtAddr::new(self.double_fault_stack.as_ptr() as u64 + STACK_SIZE as u64);

        // Set GDT
        self.gdt.add_entry(KCODE);
        self.gdt.add_entry(KDATA);
        self.gdt.add_entry(UCODE);
        self.gdt.add_entry(UDATA);
        self.gdt.add_entry(Descriptor::tss_segment(&self.tss));
        self.gdt.load();

        // Set cs and tss
        unsafe {
            set_cs(KCODE_SELECTOR);
            load_tss(TSS_SELECTOR);
            Msr::new(0xC0000102).write(&self.tss as *const _ as u64);
        }
    }

    pub fn set_return_rsp(&mut self, rsp: usize) {
        self.tss.privilege_stack_table[0] = VirtAddr::new(rsp as u64);
    }
}

pub fn init() {
    unsafe {
        CPU = Some(Cpu::new());
        CPU.as_mut().unwrap().init();
    }
}