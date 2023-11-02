pub struct FnQueue<T>{
    front: Vec<T>,
    back: Vev<T>,
}

impl<T> Default for FnQueue<T>{
    fn default() -> Self{
        Self{front:Vec::default()}
    }
}

impl<T> FnQueue<T>{
    ///Make a new empty queue
    pub fn new()->Self{
        Self::default()
    }

    ///Push an elemnet on to the queue
    pub fn push_back(&mut self, x:T){
        self.back.push(x);
    }

    ///pop an element from the queue
    /// Returns [Some] if there is some to opo
    /// Return [None] otherwise
    pub fn pop_front()->Option<T>{
        
    }
}