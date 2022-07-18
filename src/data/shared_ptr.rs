use std::{sync::atomic::{AtomicI32,Ordering},ptr, ops::{DerefMut, Deref}};


struct RefCount{
    ref_count:AtomicI32,
    wref_count:AtomicI32,
}

impl RefCount{
    #[inline]
    pub fn new()->*mut RefCount{
        return Box::into_raw(Box::new(RefCount { ref_count: AtomicI32::new(1), wref_count: AtomicI32::new(1) }));
    }

    #[inline]
    pub fn destroy(ptr: *mut RefCount){
        unsafe{
            drop(Box::from_raw(ptr))
        }
    }

    #[inline]
    pub fn get_ref(&self)->i32{
        return self.ref_count.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn get_wref(&self)->i32{
        return self.wref_count.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn inc_ref(&self){
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline]
    pub fn inc_ref_nz(&self) -> bool {
        let mut now = self.get_ref();
        while now != 0 {
            let old = self
                .ref_count
                .compare_exchange(now, now + 1, Ordering::SeqCst,Ordering::SeqCst).unwrap();
            if old == now {
                return true;
            }
            now = old;
        }
        return false;
    }

    #[inline]
    pub fn inc_wref(&self) {
        self.wref_count.fetch_add(1, Ordering::SeqCst);
    }

    #[inline]
    pub fn dec_ref(&self) -> bool {
        let r = self.ref_count.fetch_add(-1, Ordering::SeqCst);
        return r == 1;
    }

    #[inline]
    pub fn dec_wref(&self) -> bool {
        let r = self.wref_count.fetch_add(-1, Ordering::SeqCst);
        return r == 1;
    }
}

pub struct SharedPtr<T:?Sized>{
    ptr:*mut T,
    ref_count: *mut RefCount,
}

impl<T:?Sized> SharedPtr<T>{
    #[inline]
    pub fn new(b:Box<T>)->SharedPtr<T>{
        let p:*mut T=Box::into_raw(b);
        SharedPtr::from_raw(p)
    }

    #[inline]
    pub fn from_raw(p: *mut T) -> SharedPtr<T> {
        if p.is_null() {
            return SharedPtr {
                ptr: p,
                ref_count: ptr::null_mut(),
            };
        }
        let ref_ptr: *mut RefCount = RefCount::new();
        SharedPtr {
            ptr: p,
            ref_count: ref_ptr,
        }
    }

    #[inline]
    pub fn get_mut_self(&self) -> &mut SharedPtr<T> {
        unsafe {
            let x = self as *const SharedPtr<T> as *mut SharedPtr<T>;
            return &mut *x;
        }
    }

    //析构函数
    #[inline]
    pub fn destroy(&self) {
        if !self.is_valid() {
            return;
        }
        //ref减1，如果ref减成0
        if self.get_refcount().dec_ref() {
            //释放ptr
            self.destroy_ptr();
            if self.get_refcount().dec_wref() {
                //ref_count释放后赋新值
                self.destroy_refcount();
            }
        }
    }

    //将ptr指向的堆值释放，ptr在这个方法退栈后也释放
    #[inline]
    pub fn destroy_ptr(&self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
            //use ref count to test the ptr is destroyed
            //not the ptr == null
        }
        //assert!(false);
    }

    //将ref及其指向值释放后赋新值
    #[inline]
    pub fn destroy_refcount(&self) {
        RefCount::destroy(self.ref_count);
        self.get_mut_self().ref_count = ptr::null_mut::<RefCount>();
    }

    //确认你的SharedPtr是否有效，其中的ref_count必须不为空且其中的ref必须大于0
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.data_valid()
    }

    #[inline]
    pub fn data_valid(&self) -> bool {
        !self.ref_count.is_null() && self.get_refcount().get_ref() > 0
    }

    #[inline]
    #[allow(dead_code)]
    pub fn refcount_valid(&self) -> bool {
        self.ref_count != ptr::null_mut()
    }

    #[inline]
    fn get_refcount(&self) -> &mut RefCount {
        unsafe { &mut *self.ref_count }
    }

    #[inline]
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.ptr }
    }

    #[inline]
    fn get_raw(&self) -> (*mut T, *mut RefCount) {
        (self.ptr, self.ref_count)
    }
}

impl<T: ?Sized> Drop for SharedPtr<T> {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl<T: ?Sized> Clone for SharedPtr<T> {
    //clone会增加ref，并返回一个SharedPtr
    fn clone(&self) -> Self {
        self.get_refcount().inc_ref();

        SharedPtr {
            ptr: self.ptr,
            ref_count: self.ref_count,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.destroy();

        self.ptr = source.ptr.clone();
        self.ref_count = source.ref_count.clone();
        self.get_refcount().inc_ref();
    }
}

impl<T: ?Sized> Deref for SharedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_mut()
    }
}

impl<T: ?Sized> DerefMut for SharedPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

unsafe impl<T: ?Sized> Send for SharedPtr<T> {}
unsafe impl<T: ?Sized> Sync for SharedPtr<T> {}

pub struct WeakPtr<T: ?Sized> {
    ptr: *mut T,
    ref_count: *mut RefCount,
}

impl<T: ?Sized> WeakPtr<T> {
    //如果ref_count不为空，wref+1，返回WeakPtr
    #[inline]
    pub fn new(s: &SharedPtr<T>) -> WeakPtr<T> {
        let (ptr, ref_count) = s.get_raw();
        if ref_count != ptr::null_mut() {
            unsafe {
                (*ref_count).inc_wref();
            }
        }

        WeakPtr {
            ptr: ptr,
            ref_count: ref_count,
        }
    }

    #[inline]
    pub fn destroy(&self) {
        if !self.is_valid() {
            return;
        }
        if self.get_mut().dec_wref() {
            self.destroy_refcount();
        }
    }

    #[inline]
    pub fn get_mut_self(&self) -> &mut WeakPtr<T> {
        unsafe {
            let x = self as *const WeakPtr<T> as *mut WeakPtr<T>;
            return &mut *x;
        }
    }

    #[inline]
    pub fn destroy_refcount(&self) {
        RefCount::destroy(self.ref_count);
        self.get_mut_self().ref_count = ptr::null_mut();
        //assert!(false);
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.ref_count != ptr::null_mut()
    }

    #[inline]
    fn get_mut(&self) -> &mut RefCount {
        unsafe { &mut *self.ref_count }
    }

    #[inline]
    pub fn lock(&self) -> Option<SharedPtr<T>> {
        //将ref+1，如果成功则返回Some(r)
        if self.ref_count != ptr::null_mut() && self.get_mut().inc_ref_nz() {
            let r = SharedPtr {
                ptr: self.ptr,
                ref_count: self.ref_count,
            };
            return Some(r);
        }
        return None;
    }
}

impl<T: ?Sized> Drop for WeakPtr<T> {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl<T: ?Sized> Clone for WeakPtr<T> {
    fn clone(&self) -> Self {
        self.get_mut().inc_wref();

        WeakPtr {
            ptr: self.ptr,
            ref_count: self.ref_count,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.destroy();

        self.ptr = source.ptr.clone();
        self.ref_count = source.ref_count.clone();
        self.get_mut().inc_wref();
    }
}

unsafe impl<T: ?Sized> Send for WeakPtr<T> {}
unsafe impl<T: ?Sized> Sync for WeakPtr<T> {}

#[allow(dead_code)]
pub fn s1(){
    //ref_count_add_and_sub
    let rc = RefCount::new();
    let reference: &RefCount = unsafe { &*rc };
    assert_eq!(1, reference.get_ref());
    assert_eq!(1, reference.get_wref());

    reference.inc_ref();
    assert_eq!(2, reference.get_ref());
    reference.inc_ref();
    assert_eq!(3, reference.get_ref());

    reference.inc_wref();
    assert_eq!(2, reference.get_wref());

    let r1 = reference.dec_ref();
    assert!(!r1);
    assert_eq!(2, reference.get_ref());

    let r2 = reference.dec_ref();
    assert!(!r2);
    assert_eq!(1, reference.get_ref());

    let r = reference.dec_ref();
    assert!(r);
    assert_eq!(0, reference.get_ref());

    //ref_count_inc_ref_nz
    let rc = RefCount::new();
    let reference: &RefCount = unsafe { &*rc };

    assert_eq!(1, reference.get_ref());

    reference.dec_ref();

    let r = reference.inc_ref_nz();
    assert!(!r);

    reference.inc_ref();
    let r1 = reference.inc_ref_nz();
    assert!(r1);
    assert_eq!(2, reference.get_ref());
    //shared_ptr_simple_test
    let p: SharedPtr<i32> = SharedPtr::new(Box::new(1));

    assert_eq!(1, *p.get_mut());

    *p.get_mut() = 2;
    assert_eq!(2, *p.get_mut());

    //move here
    let x = p;
    assert_eq!(2, *x.get_mut());

    //shared_ptr_test_drop
    let p: SharedPtr<i32> = SharedPtr::new(Box::new(1));
    drop(p);

    //shared_ptr_clone
    let p: SharedPtr<i32> = SharedPtr::new(Box::new(1));
    //每次clone都会增加ref
    let x = p.clone();

    assert_eq!(1, *p.get_mut());

    *x.get_mut() = 2;
    assert_eq!(2, *p.get_mut());
    assert_eq!(2, *x.get_mut());

    assert_eq!(2, p.get_refcount().get_ref());
    assert_eq!(1, p.get_refcount().get_wref());

    let z = x.clone();

    assert_eq!(3, x.get_refcount().get_ref());
    assert_eq!(1, x.get_refcount().get_wref());
    //ref减1，减到0后就destroy
    drop(p);
    drop(z);

    assert_eq!(1, x.get_refcount().get_ref());
    assert_eq!(1, x.get_refcount().get_wref());

    drop(x);

    struct Sample {
        num: i32,
    }

    impl Sample {
        pub fn new(i: i32) -> Sample {
            Sample { num: i }
        }

        pub fn get_number(&self) -> i32 {
            return self.num;
        }
    }
    //shared_ptr_deref
    let p: SharedPtr<Sample> = SharedPtr::new(Box::new(Sample::new(10)));
    assert_eq!(10, p.get_number());
    p.get_mut().num = 20;
    assert_eq!(20, p.get_number());

    //weak_ptr_lock
    let p: SharedPtr<i32> = SharedPtr::new(Box::new(1));
    let w = WeakPtr::new(&p);//这里会将wref+1

    assert_eq!(2, p.get_refcount().get_wref());

    let l = w.lock();//这里ref+1
    match l {
        None => assert!(false),
        Some(r) => {
            assert_eq!(2, r.get_refcount().get_ref());
            *r.get_mut() = 10;
            assert_eq!(10, *r.get_mut());
        }
    }

    //weak_ptr_lock_after_drop
    let p: SharedPtr<i32> = SharedPtr::new(Box::new(1));
    let w = WeakPtr::new(&p);

    drop(p);

    let l = w.lock();
    match l {
        Some(_r) => assert!(false),
        None => assert!(true),
    }
    //weak_ptr_clone
    let p: SharedPtr<i32> = SharedPtr::new(Box::new(1));
    let w = WeakPtr::new(&p);

    let c = w.clone();
    assert_eq!(3, c.get_mut().get_wref());

    drop(c);
    assert_eq!(2, w.get_mut().get_wref());

    *p.get_mut() = 10;
    assert_eq!(10, *p.get_mut());

    let w1 = w.clone();

    if let Some(p1) = w1.lock() {
        assert!(p1.is_valid());
        assert_eq!(10, *p1.get_mut());

        drop(p);
        drop(p1);

        let p2 = w1.lock();
        assert!(p2.is_none());
    } else {
        assert!(false);
    }
}