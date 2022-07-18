
struct Heap<T>
where T:Default,
{
    count:usize,
    items:Vec<T>,
    comparator:fn(&T,&T)->bool,
}

impl<T> Heap<T>
where T:Default,
{
    fn new(comparator:fn(&T,&T)->bool)->Self{
        Self { count: 0, items: vec![T::default()], comparator,}
    }

    fn len(&self)->usize{
        self.count
    }

    #[allow(dead_code)]
    fn is_empty(&self)->bool{
        self.len()==0
    }

    fn add(&mut self,value:T){
        self.count+=1;
        self.items.push(value);
        let mut idx=self.count;
        while self.parent_idx(idx)>0{
            let pdx=self.parent_idx(idx);
            if(self.comparator)(&self.items[idx],&self.items[pdx]){
                self.items.swap(idx,pdx);
            }
            idx=pdx;
        }
    }

    fn parent_idx(&self,idx:usize)->usize{
        idx/2
    }

    fn children_present(&self,idx:usize)->bool{
        self.left_child_idx(idx)<=self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        } else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if (self.comparator)(&self.items[ldx], &self.items[rdx]) {
                ldx
            } else {
                rdx
            }
        }
    }
}

impl<T> Iterator for Heap<T>
where T:Default,
{
    type Item=T;
    fn next(&mut self)->Option<T>{
        if self.count==0{
            return None;
        }
        let next=Some(self.items.swap_remove(1));
        self.count-=1;
        if self.count>0{
            let mut idx=1;
            while self.children_present(idx){
                let cdx=self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx],&self.items[cdx]){
                    self.items.swap(idx, cdx);
                }
                idx=cdx;
            }
        }
        next
    }
}

struct MinHeap;

impl MinHeap{
    fn new<T>()->Heap<T> where T:Default+Ord,{
        Heap::new(|a,b|a<b)
    }
}

struct MaxHeap;

impl MaxHeap{
    fn new<T>()->Heap<T> where T:Default+Ord,{
        Heap::new(|a,b| a>b)
    }
}

struct Point(/* x */ i32, /* y */ i32);
impl Default for Point {
    fn default() -> Self {
        Self(0, 0)
    }
}

#[allow(dead_code)]
pub fn h1(){
    let mut heap = MaxHeap::new::<i32>();
    assert_eq!(heap.next(), None);
    let mut heap = MinHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.next(), Some(2));
    assert_eq!(heap.next(), Some(4));
    assert_eq!(heap.next(), Some(9));
    heap.add(1);
    assert_eq!(heap.next(), Some(1));
    let mut heap = MaxHeap::new();
    heap.add(4);
    heap.add(2);
    heap.add(9);
    heap.add(11);
    assert_eq!(heap.len(), 4);
    assert_eq!(heap.next(), Some(11));
    assert_eq!(heap.next(), Some(9));
    assert_eq!(heap.next(), Some(4));
    heap.add(1);
    assert_eq!(heap.next(), Some(2));
    let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
    heap.add(Point(1, 5));
    heap.add(Point(3, 10));
    heap.add(Point(-2, 4));
    assert_eq!(heap.len(), 3);
    assert_eq!(heap.next().unwrap().0, -2);
    assert_eq!(heap.next().unwrap().0, 1);
    heap.add(Point(50, 34));
    assert_eq!(heap.next().unwrap().0, 3);
}