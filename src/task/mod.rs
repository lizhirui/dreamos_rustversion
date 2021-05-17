use core::alloc::Layout;

use crate::common::list::list_node;

pub type task_entry_t = fn(arg: usize);

#[derive(PartialEq)]
pub enum task_state_t
{
    running,
    ready,
    blocking,
    suspending,
    stopped
}

#[repr(C)]
pub struct task_t
{
    pub sp: usize,
    pub pid: usize,
    pub tid: usize,
    pub sid: usize,
    pub stack_addr: usize,
    pub stack_size: usize,
    pub priority: usize,
    pub entry: task_entry_t,
    pub arg: usize,
    pub state: task_state_t,
    pub dispatch_node: list_node
}

impl task_t
{
    pub const fn new() -> task_t
    {
        task_t
        {
            sp: 0,
            pid: 0,
            tid: 0,
            sid: 0,
            stack_addr: 0,
            stack_size: 0,
            priority: 0,
            entry: |_arg|{},
            arg: 0,
            state: task_state_t::ready,
            dispatch_node: list_node::new()
        }
    }
}

const priority_max: usize = 31;
const priority_uplimit: usize = priority_max + 1;

static mut task_list: list_node = list_node::new();
static mut priority_list: [list_node;priority_uplimit] = [list_node::new();priority_uplimit];

pub fn init()
{
    for i in 0..priority_uplimit
    {
        list_init!(priority_list[i]);
    }
}

fn get_next_task() -> Option<&'static mut task_t>
{
    for i in 0..priority_uplimit
    {
        if !list_empty!(priority_list[i])
        {
            list_entry_foreach!(priority_list[i],task_t,dispatch_node,entry,
            {
                if entry.state == task_state_t::ready
                {
                    return Some(entry);
                }
            });
        }
    }

    return None;
}

/*
fn create_task(sp: usize,stack_size: usize,priority: usize,entry: task_entry_t,arg: usize)
{
    let mut new_task = unsafe{alloc(Layout::from_size_align(core::mem::size_of::<task_t>(),core::mem::size_of::<task_t>(8)))};
    new_task.sp = sp;
    new_task.stack_addr = unsafe{alloc(Layout::from_size_align(stack_size,stack_size).unwrap()) as usize}
    new_task.stack_size = stack_size;
    new_task.priority = priority;
    new_task.entry = entry;
    new_task.arg = arg;
}*/

fn task_dispatch()
{

}
