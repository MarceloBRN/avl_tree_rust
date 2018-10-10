use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::mem;
use std::fmt;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AVLTree<T: Clone>{
    size: usize,
    depth_left: usize,
    depth_right: usize,
    root: Option<Rc<RefCell<AVLNode<T>>>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
struct AVLNode<T: Clone>{
    data: Option<T>,
    left: Option<Rc<RefCell<AVLNode<T>>>>,
    right: Option<Rc<RefCell<AVLNode<T>>>>,
}

#[allow(dead_code)]
pub enum BTreeOrder {
    Preorder,
    Inorder,
    Postorder,
}

#[derive(Debug)]
enum NodeDirection {
    NodeNone,
    NodeLeft,
    NodeRight,
}

#[allow(dead_code, unused_variables)]
impl <T> AVLTree<T> where T: Clone + Ord + Debug {
    pub fn new() -> Self {
        AVLTree {
            size: 0,
            depth_left: 0,
            depth_right: 0,
            root: None,
        }
    }

    pub fn as_vec(&self, arg: BTreeOrder) -> Vec<T> {
        unimplemented!();
    }

    pub fn clear(&mut self) {
        if self.size != 0 {
            let root = self.root.take();
        }
    }
    
    pub fn contains(&self, data: T, mode: BTreeOrder) ->  bool {
        unimplemented!();
    }

    pub fn children(&self, data: T) ->  (Option<T>, Option<T>) {
        unimplemented!();
    }

    #[allow(unused_assignments)]
    pub fn insert(&mut self, key: T) -> bool{
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(AVLNode {
                data: Some(key),
                left: None,
                right: None,
            }));

            self.root = Some(new_node);
            self.size += 1;
            true
        } else {
            let root = self.root.take().unwrap();
            let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);
            let mut node_parent: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);

            loop {

                if node_current.borrow().data.is_some() {

                    if &key == node_current.borrow().data.as_ref().unwrap() {
                        
                        self.root = Some(root);
                        return false
                        
                    } else if &key < node_current.borrow().data.as_ref().unwrap() {

                        node_parent = node_current;
                        // node_current = Rc::new(RefCell::new(AVLNode { data: None, left: None, right: None}));
                        
                        match node_parent.borrow().left {
                            None => { break; }
                            _ => { node_current = Rc::clone(node_parent.borrow().left.as_ref().unwrap()); }
                        }
                    } else {   
                        
                        node_parent = node_current;
                        // node_current = Rc::new(RefCell::new(AVLNode { data: None, left: None, right: None,}));
                        
                        match node_parent.borrow().right {
                            None => { break; }
                            _ => { node_current = Rc::clone(node_parent.borrow().right.as_ref().unwrap()); }
                        }
                    }
                } else {
                    break;
                }
            }

            // node_current = Rc::new(RefCell::new(AVLNode { data: None, left: None, right: None }));

            let new_node = Rc::new(RefCell::new(AVLNode { data: Some(T::clone(&key)), left: None, right: None }));

            if Rc::ptr_eq(&root, &node_parent) {

                self.root = Some(root);

                let n = Rc::make_mut(self.root.as_mut().unwrap());
                
                if Some(&key) < n.get_mut().data.as_ref(){
                    n.borrow_mut().left = Some(new_node);
                    self.depth_left += 1;
                } else {
                    n.borrow_mut().right = Some(new_node);
                    self.depth_right += 1;
                }


            } else {

                self.root = Some(root);

                if Some(&key) < node_parent.borrow().data.as_ref(){
                    node_parent.borrow_mut().left = Some(new_node);
                    if node_parent.borrow().right.is_none(){
                        self.depth_left += 1;
                    }
                } else {
                    node_parent.borrow_mut().right = Some(new_node);
                    if node_parent.borrow().left.is_none(){
                        self.depth_right += 1;
                    }
                }

            }

            self.size += 1;            
            true
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.size == 0 { true } else { false }
    }

    pub fn len(&self) -> usize {
        self.size
    }
    
    #[allow(unused_assignments)]
    pub fn remove(&mut self, data: T) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let mut node_data: Option<T> = None;
            if &data == self.root.as_ref().unwrap().borrow().data.as_ref().unwrap() {
                let node_root = Rc::clone(self.root.as_ref().unwrap());
                let node_root = node_root.borrow();

                if node_root.right.is_none() && node_root.left.is_none() {
                    let nd = self.root.take().unwrap();
                    node_data = nd.borrow().data.clone();
                    self.size -= 1;
                } else if node_root.right.is_none() {

                } else if node_root.left.is_none() {

                } else {

                }
                                
            } else {
                let root: Rc<RefCell<AVLNode<T>>> = self.root.take().unwrap();
            
                let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);
                let mut node_parent: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);

                let mut node_direction: NodeDirection = NodeDirection::NodeNone;

                loop {
                    if node_current.borrow().data.is_some() {
                        if &data == node_current.borrow().data.as_ref().unwrap() {
                            let node_c = Rc::clone(&node_current);
                            let node_c = node_c.borrow();

                            if node_c.right.is_none() && node_c.left.is_none() {
                                match node_direction {
                                    NodeDirection::NodeLeft => {
                                        let value = Rc::into_raw(node_parent.borrow_mut().left.take().unwrap()); 
                                    },
                                    NodeDirection::NodeRight => {
                                        let value = Rc::into_raw(node_parent.borrow_mut().right.take().unwrap()); 
                                    },
                                    _ => {}
                                }
                            
                            } else if node_c.right.is_none() {

                            } else if node_c.left.is_none() {

                            } else {

                            }

                            break;
                        } else if &data < node_current.borrow().data.as_ref().unwrap() {

                            let node_c = Rc::clone(&node_current);
                            let node_c = node_c.borrow();

                            match node_c.left {
                                None => {
                                    let value = Rc::into_raw(node_parent.borrow_mut().left.take().unwrap());
                                    // let value = node_current.borrow();
                                }
                                _ => {
                                    node_parent = node_current;
                                    let node = Rc::clone(&node_parent);
                                    node_current = Rc::clone(node.borrow().left.as_ref().unwrap());
                                    node_direction = NodeDirection::NodeLeft;
                                }
                            }

                        } else {
                            let node_c = node_current.clone();
                            let node_c = node_c.borrow();

                            match node_c.right {
                                None => {
                                    let value = Rc::into_raw(node_parent.borrow_mut().right.take().unwrap());
                                    // let value = node_current.borrow();
                                }
                                _ => {
                                    node_parent = node_current;
                                    let node = Rc::clone(&node_parent);
                                    node_current = Rc::clone(node.borrow().right.as_ref().unwrap());
                                    node_direction = NodeDirection::NodeRight;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                self.root = Some(root);

                self.size -= 1;
            }        

            node_data
        }
    }

    pub fn sub_tree(&self, data: T) -> AVLTree<T> {
        unimplemented!();
    }

    pub fn depth(&self) -> usize {
        if self.depth_left > self.depth_right {
            self.depth_left
        } else {
            self.depth_right
        }
    }

    pub fn height(&self) ->  usize {
        if self.size == 0 { return 0 }

        let mut q = VecDeque::<Rc<RefCell<AVLNode<T>>>>::new();

        q.push_back(self.root.as_ref().unwrap().clone());

        let mut height: usize = 0;

        loop {
            let mut node_count: usize = q.len();
            if node_count == 0 {
                return height
            }

            height += 1;

            while node_count > 0
            { 
                let node = q.pop_front().unwrap();
                if node.borrow().left.is_some() {
                    q.push_back(node.borrow().left.as_ref().unwrap().clone()); 
                }
                if node.borrow().right.is_some() { 
                    q.push_back(node.borrow().right.as_ref().unwrap().clone()); 
                }
                node_count = node_count- 1; 
            } 
        }
    }

    fn left_rotate(&mut self) {
        let mut root = self.root.take();
        let mut node_right: Option<Rc<RefCell<AVLNode<T>>>> = root.as_ref().unwrap().borrow_mut().right.take();

        let node_left: Option<Rc<RefCell<AVLNode<T>>>>;
        if node_right.is_some() {
            node_left = node_right.as_ref().unwrap().borrow_mut().left.take();
            root.as_mut().unwrap().borrow_mut().right = node_left;
            node_right.as_mut().unwrap().borrow_mut().left = root;
        }

        self.root = node_right;        

    }

    fn right_rotate(&mut self) {
        unimplemented!();
    }
    
}


#[allow(dead_code, unused_variables)]
impl <T> AVLNode<T> where T: Clone{
    pub fn height(&self) ->  usize {
        if self.data.is_none() { return 0 }

        let mut q = VecDeque::<Rc<RefCell<AVLNode<T>>>>::new();

        q.push_back(Rc::new(RefCell::new(self.clone())));

        let mut height: usize = 0;

        loop {
            let mut node_count: usize = q.len();
            if node_count == 0 {
                return height
            }

            height += 1;

            while node_count > 0
            { 
                let node = q.pop_front().unwrap();
                if node.borrow().left.is_some() {
                    q.push_back(node.borrow().left.as_ref().unwrap().clone()); 
                }
                if node.borrow().right.is_some() { 
                    q.push_back(node.borrow().right.as_ref().unwrap().clone()); 
                }
                node_count = node_count- 1; 
            } 
        }
    }

    pub fn height_left(&self) ->  usize {
        unimplemented!();
    }
    
    pub fn height_right(&self) ->  usize {
        unimplemented!();
    }
}

#[allow(unused_mut, unused_variables, unused_assignments)]
impl <T> Display for AVLTree<T> where T: Clone + Display + Debug{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut column: usize = 0;
        writeln!(f, "AVLTree (size = {}): ", self.size);
        
        if self.root.is_some() {
            let mut node_vec: Vec<(usize, Rc<RefCell<AVLNode<T>>>)> = Vec::new();
            let mut node_vec_current: Vec<(usize, Rc<RefCell<AVLNode<T>>>)> = vec![(column, Rc::clone(&self.root.as_ref().unwrap()))];
            
            let mut node_removed: Rc<RefCell<AVLNode<T>>> = Rc::new(RefCell::new(AVLNode { data: None, left: None, right: None }));;

            while !node_vec_current.is_empty() {
                if node_vec_current[0].1.borrow().data.is_some() {
                    column = node_vec_current[0].0;
                    for i in 0..column {
                        write!(f, "|\t");
                    }
                    writeln!(f, "{}", node_vec_current[0].1.borrow().data.as_ref().unwrap() );
                    node_removed = node_vec_current.remove(0).1;
                }
                
                if node_removed.borrow().right.is_some() {
                    node_vec_current.insert(0, (column + 1, Rc::clone(node_removed.borrow().right.as_ref().unwrap())));
                }

                if node_removed.borrow().left.is_some() {
                    node_vec_current.insert(0, (column + 1, Rc::clone(node_removed.borrow().left.as_ref().unwrap())));
                }

            }
            
        }
        write!(f, "")
    }
}

#[allow(unused_mut, unused_variables, unused_assignments)]
impl<T: Clone> Drop for AVLNode<T>{
    fn drop(&mut self) {
        // let mut data = mem::replace(&mut self.data, None);
        let mut left = mem::replace(&mut self.left, None);
        let mut right = mem::replace(&mut self.right, None);

        loop {
            left = match left {
                Some(mut n) => mem::replace(&mut (*Rc::make_mut(&mut n)).get_mut().left, None),
                None => break,
            }
        }

        loop {
            right = match right {
                Some(mut n) => mem::replace(&mut (*Rc::make_mut(&mut n)).get_mut().right, None),
                None => break,
            }
        }
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {
    // extern crate heapsize;
    // extern crate time;

    // use self::heapsize::*;
    // use std::rc::Rc;
    use super::AVLTree;
    use std::time::Instant;

    // use std::process::Command;

    #[allow(unused_mut, unused_variables, unused_assignments)]
    #[test]
    fn new_test() {
        let mut tree = AVLTree::<i32>::new();
        // tree.insert(5);
        // println!("depth 1 = {:?}", tree.depth());
        // tree.insert(1);
        // println!("depth 2 = {:?}", tree.depth());
        // tree.insert(3);
        // tree.insert(4);
        // tree.insert(2);
        // tree.insert(10);
        // tree.insert(-3);
        // tree.insert(-1);
        // tree.insert(7);
        // tree.insert(11);
        // println!("{:?}\n", tree);
        // println!("{}\n", tree);

        let now = Instant::now();
        let mut r = false;
        for i in 0..2000 {
            r = tree.insert(i);
        }

        let new_now = Instant::now();
        println!("time = {:?}", new_now.duration_since(now));

        println!("depth = {:?}", tree.depth());
        println!("height = {:?}", tree.height());
        let right = &tree.root.as_ref().unwrap().borrow().right;
        if right.is_some() {
            println!("height right = {:?}", right.as_ref().unwrap().borrow().height());
        }

        let left = &tree.root.as_ref().unwrap().borrow().left;
        if left.is_some() {
            println!("height left = {:?}", left.as_ref().unwrap().borrow().height());
        }
        
        // tree.remove(2);
        // tree.remove(11);        
        // tree.remove(4);
        // tree.remove(3);
        // println!("{:?}", tree);
        // println!("{}", tree);
        // tree.remove(1);
        // println!("Remove = {:?}", tree);
        // tree.remove(5);
        // println!("Remove = {:?}", tree);
    }
}