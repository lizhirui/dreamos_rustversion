pub mod phypage;
pub mod slab;
pub mod allocator;

pub fn init()
{
    phypage::init();
    test();
}

fn test()
{
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 
    {
        vec.push(i);
    }

    assert_eq!(vec.len(), 10000);

    for (i, value) in vec.into_iter().enumerate() 
    {
        assert_eq!(value, i);
    }
    
    println!("heap test passed");
}