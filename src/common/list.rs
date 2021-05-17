use core::ptr::{null, null_mut};

#[repr(C)]
#[derive(Copy,Clone)]
pub struct list_node
{
    pub prev: *mut list_node,
    pub next: *mut list_node
}

impl list_node
{
    pub const fn new() -> list_node
    {
        list_node{prev: null_mut(),next: null_mut()}
    }

    pub fn insert_after(&mut self,reference_node: &mut list_node)
    {
        self.next = reference_node.next;
        self.prev = reference_node;
        reference_node.next = self;

        if !self.next.is_null()
        {
            unsafe{(*self.next).prev = self;}
        }
    }

    pub fn insert_before(&mut self,reference_node: &mut list_node)
    {
        self.next = reference_node;
        self.prev = reference_node.prev;
        reference_node.prev = self;

        if !self.prev.is_null()
        {
            unsafe{(*self.prev).next = self;}
        }
    }

    pub fn remove_from_list(&mut self)
    {
        if !self.prev.is_null()
        {
            unsafe{(*self.prev).next = self.next};
        }

        if !self.next.is_null()
        {
            unsafe{(*self.next).prev = self.prev};
        }

        self.prev = null_mut();
        self.next = null_mut();
    }
}

#[macro_export]
macro_rules! list_init
{
    ($list_node: expr) => 
    {
        unsafe 
        {
            $list_node.prev = get_pointer!($list_node);
            $list_node.next = get_pointer!($list_node);
        }
    }
}

#[macro_export]
macro_rules! list_insert_head
{
    ($list: expr,$list_node: expr) =>
    {
        $list_node.insert_after(&mut $list);
    }
}

#[macro_export]
macro_rules! list_insert_tail
{
    ($list: expr,$list_node: expr) =>
    {
        $list_node.insert_before(&mut $list);
    }
}

#[macro_export]
macro_rules! list_head
{
    ($list: expr) => 
    {
        unsafe{$list.next}
    }
}

#[macro_export]
macro_rules! list_tail
{
    ($list: expr) =>
    {
        unsafe{$list.prev}
    }
}

#[macro_export]
macro_rules! list_empty
{
    ($list: expr) =>
    {
        unsafe{$list.prev == $list.next}
    }
}

#[macro_export]
macro_rules! list_entry
{
    ($list_node_object: expr,$object_type: ty,$list_node_field: ident) => 
    {
        unsafe{&*((get_address!(*$list_node_object) - offsetof!($object_type,$list_node_field)) as *mut $object_type)}
    };
}

#[macro_export]
macro_rules! list_entry_const
{
    ($list_node_object: expr,$object_type: ty,$list_node_field: ident) => 
    {
        unsafe{&*((get_address!(*$list_node_object) - offsetof!($object_type,$list_node_field)) as *const $object_type)}
    };
}

#[macro_export]
macro_rules! list_entry_foreach
{
    ($list_node_object_pointer: expr,$object_type: ty,$list_node_field: ident,$entry_variable: ident,$body: block) => 
    {
        {
            let mut cur_node = unsafe{$list_node_object_pointer.next};

            while(cur_node != unsafe{&mut $list_node_object_pointer})
            {
                let $entry_variable = unsafe{&mut *((get_address!(*cur_node) - offsetof!($object_type,$list_node_field)) as *mut $object_type)};
                $body
                unsafe{cur_node = (*cur_node).next};
            }
        }
    };
}

#[macro_export]
macro_rules! list_entry_foreach_const
{
    ($list_node_object_pointer: expr,$object_type: ty,$list_node_field: ident,$entry_variable: ident,$body: block) => 
    {
        {
            let mut cur_node = $list_node_object_pointer.next;

            while(cur_node != &mut $list_node_object_pointer)
            {
                unsafe 
                {
                    let $entry_variable = unsafe{&*((get_address!(*cur_node) - offsetof!($object_type,$list_node_field)) as *const $object_type)};
                    $body
                    unsafe{cur_node = (*cur_node).next};
                }
            }
        }
    };
}

#[macro_export]
macro_rules! list_entry_foreach_safe
{
    ($list_node_object_pointer: expr,$object_type: ty,$list_node_field: ident,$entry_variable: ident,$body: block) => 
    {
        {
            let mut cur_node = $list_node_object_pointer.next;

            while(cur_node != &mut $list_node_object_pointer)
            {
                unsafe 
                {
                    let $entry_variable = unsafe{&*((get_address!(*cur_node) - offsetof!($object_type,$list_node_field)) as *mut $object_type)};
                    unsafe{cur_node = (*cur_node).next};
                    $body
                }
            }
        }
    };
}

#[macro_export]
macro_rules! list_entry_foreach_safe_const
{
    ($list_node_object_pointer: expr,$object_type: ty,$list_node_field: ident,$entry_variable: ident,$body: block) => 
    {
        {
            let mut cur_node = $list_node_object_pointer.next;

            while(cur_node != &mut $list_node_object_pointer)
            {
                unsafe 
                {
                    let $entry_variable = unsafe{&*((get_address!(*cur_node) - offsetof!($object_type,$list_node_field)) as *const $object_type)};
                    unsafe{cur_node = (*cur_node).next};
                    $body  
                }
            }
        }
    };
}
