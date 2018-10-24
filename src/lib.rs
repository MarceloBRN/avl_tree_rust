use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Debug;
use std::rc::Rc;
use std::mem;
use std::fmt;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AVLTree<T: Clone>{
    size: usize,
    height_left: isize,
    height_right: isize,
    root: Option<Rc<RefCell<AVLNode<T>>>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
struct AVLNode<T: Clone>{
    data: Option<T>,
    left: Option<Rc<RefCell<AVLNode<T>>>>,
    right: Option<Rc<RefCell<AVLNode<T>>>>,
}

#[allow(dead_code, non_snake_case)]
pub enum BTreeOrder {
    PreOrder,
    InOrder,
    PostOrder,
}

#[derive(Debug)]
enum NodeDirection {
    NodeNone,
    NodeLeft,
    NodeRight,
}

#[allow(dead_code, unused_variables, non_snake_case, unreachable_patterns)]
impl <T> AVLTree<T> where T: Clone + Ord + Debug{
    pub fn new() -> Self {
        AVLTree {
            size: 0,
            height_left: 0,
            height_right: 0,
            root: None,
        }
    }

    pub fn as_vec(&self, arg: BTreeOrder) -> Vec<T> {
        let mut v = vec![];
        if self.size == 0 {
            return v
        }


        match arg {
            BTreeOrder::PreOrder => {
                let node_root = self.root.clone();
                let mut stack = VecDeque::<Option<Rc<RefCell<AVLNode<T>>>>>::new();
                stack.push_back(node_root.clone());

                loop {
                    let mut node_count: usize = stack.len();
                    if node_count == 0 {
                        return v
                    }

                    while node_count > 0 {

                        let node = stack.pop_back().unwrap().unwrap();
                        v.push(node.borrow_mut().data.as_ref().unwrap().clone());
                        if node.borrow().right.is_some() {
                            stack.push_back(node.borrow().right.clone());
                        }
                        if node.borrow().left.is_some() {
                            stack.push_back(node.borrow().left.clone());
                        }
                        node_count = node_count - 1;
                    }
                }
            },
            BTreeOrder::InOrder => {
                let node_root = self.root.as_ref().unwrap();
                let mut node_current: Rc<RefCell<AVLNode<T>>> = node_root.clone();

                let mut stack = VecDeque::<Option<Rc<RefCell<AVLNode<T>>>>>::new();

                while stack.len() != 0 || node_current.borrow().data.is_some() {

                    while node_current.borrow().data.is_some() {

                        stack.push_back(Some(node_current.clone()));
                        let node_next = Rc::clone(&node_current);
                        if node_current.borrow().left.is_some() {
                            node_current = Rc::clone(node_next.borrow().left.as_ref().unwrap());
                        } else {
                            break;
                        }
                    }

                    node_current = stack.pop_back().unwrap().unwrap();
                    v.push(node_current.borrow_mut().data.as_ref().unwrap().clone());

                    let node_next = Rc::clone(&node_current);
                    if node_current.borrow().right.is_some() {
                        node_current = Rc::clone(node_next.borrow().right.as_ref().unwrap());
                    } else {
                        node_current = Rc::new(RefCell::new(AVLNode{data: None, left: None, right: None}));
                    }

                }
                return v
            },
            BTreeOrder::PostOrder => {
                return vec![]
            }
        }
    }

    pub fn clear(&mut self) {
        if self.size != 0 {
            let root = self.root.take();
            self.size = 0;
        }
    }

    pub fn contains(&self, data: T) ->  bool where T: Clone{
        if self.size == 0 {
            false
        } else {
            let root = self.root.as_ref().unwrap();
            let mut node_current: Rc<RefCell<AVLNode<T>>> = root.clone();

            loop {

                let nc = node_current.borrow().data.clone();
                
                if nc.is_some() {

                    if &data == nc.as_ref().unwrap() {

                        return true

                    } else if &data < nc.as_ref().unwrap() {

                        let node_next = Rc::clone(&node_current);
                        if node_current.borrow().left.is_some() {
                            node_current = Rc::clone(node_next.borrow().left.as_ref().unwrap());
                        } else {
                            break;
                        }

                    } else {

                        let node_next = Rc::clone(&node_current);
                        if node_current.borrow().right.is_some() {
                            node_current = Rc::clone(node_next.borrow().right.as_ref().unwrap());
                        } else {
                            break;
                        }
                        
                    }
                } else {
                    break;
                }
            }

            false
        }
    }

    pub fn children(&self, data: T) ->  (Option<T>, Option<T>) where T: Clone {
        unimplemented!();
    }

    pub fn depth(&self) -> isize {
        if self.height_left > self.height_right {
            self.height_left
        } else {
            self.height_right
        }
    }

    pub fn height(&self) ->  isize {
        if self.size == 0 { return 0 }

        let mut q = VecDeque::<Option<Rc<RefCell<AVLNode<T>>>>>::new();

        q.push_back(self.root.clone());

        let mut height: isize = 0;

        loop {
            let mut node_count: usize = q.len();
            if node_count == 0 {
                return height
            }

            height += 1;

            while node_count > 0
            {
                let node = q.pop_front().unwrap().unwrap();
                if node.borrow().left.is_some() {
                    q.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    q.push_back(node.borrow().right.clone());
                }
                node_count = node_count - 1;
            }
        }
    }

    pub fn height_left(&self) ->  isize {
        let mut height_left: isize = 0;

        if self.root.is_none() { return height_left }

        let left = &self.root.as_ref().unwrap().borrow().left;
        if left.is_some() {
            height_left = AVLTree::<T>::height_node(&left);
        }

        height_left
    }

    pub fn height_left_from_data(&self, data: T) ->  isize where T: Clone {
        unimplemented!();
    }

    pub fn height_right(&self) ->  isize {
        let mut height_right: isize = 0;

        if self.root.is_none() { return height_right }

        let right = &self.root.as_ref().unwrap().borrow().right;
        if right.is_some() {
            height_right = AVLTree::<T>::height_node(&right);
        }

        height_right
    }

    pub fn height_right_from_data(&self, data: T) ->  isize where T: Clone {
        unimplemented!();
    }

    pub fn insert(&mut self, data: T) -> bool where T: Clone{
        if self.size == 0 {
            let new_node = Rc::new(RefCell::new(AVLNode {
                data: Some(T::clone(&data)),
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

                let nc = node_current.borrow().data.clone();
                
                if nc.is_some() {

                    if &data == nc.as_ref().unwrap() {

                        self.root = Some(root);
                        return false

                    } else if &data < nc.as_ref().unwrap() {

                        node_parent = node_current;

                        if node_parent.borrow().left.is_some() {
                            node_current = Rc::clone(node_parent.borrow().left.as_ref().unwrap());
                        } else {
                            break;
                        }

                    } else {

                        node_parent = node_current;
                        
                        if node_parent.borrow().right.is_some() {
                            node_current = Rc::clone(node_parent.borrow().right.as_ref().unwrap());
                        } else {
                            break;
                        }
                        
                    }
                } else {
                    break;
                }
            }

            let new_node = Rc::new(RefCell::new(AVLNode {   data: Some(T::clone(&data)), 
                                                                left: None, 
                                                                right: None, }));

            if Rc::ptr_eq(&root, &node_parent) {

                self.root = Some(root);

                let n = Rc::make_mut(self.root.as_mut().unwrap());

                if Some(&data) < n.get_mut().data.as_ref(){
                    n.borrow_mut().left = Some(new_node);
                    self.height_left += 1;
                } else {
                    n.borrow_mut().right = Some(new_node);
                    self.height_right += 1;
                }

            } else {

                self.root = Some(root);

                if Some(&data) < node_parent.borrow().data.as_ref(){
                    node_parent.borrow_mut().left = Some(new_node);
                    if node_parent.borrow().right.is_none(){
                        if Some(&data) < self.root.as_ref().unwrap().borrow().data.as_ref(){
                            self.height_left += 1;
                        } else {
                            self.height_right += 1;
                        } 
                    }
                } else {
                    node_parent.borrow_mut().right = Some(new_node);
                    if node_parent.borrow().left.is_none(){
                        if Some(&data) < self.root.as_ref().unwrap().borrow().data.as_ref(){
                            self.height_left += 1;
                        } else {
                            self.height_right += 1;
                        } 
                    }
                }

                self.rebalance();
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

    pub fn len_left(&self) -> usize {
        unimplemented!();
    }

    pub fn len_right(&self) -> usize {
        unimplemented!();
    }

    pub fn max_value(&self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let node_data: Option<T>;
            if self.root.as_ref().unwrap().borrow().right.is_none() {
                node_data = self.root.as_ref().unwrap().borrow().data.clone();
            } else {

                let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(self.root.as_ref().unwrap());

                loop {
                    if node_current.borrow().right.is_some() {
                        let node = Rc::clone(&node_current);
                        node_current = Rc::clone(node.borrow().right.as_ref().unwrap());
                    } else {
                        let node_c = Rc::clone(&node_current);
                        let node_c = node_c.borrow();
                        node_data = node_c.data.clone();
                        break;
                    }
                }
            }
            node_data
        }
    }

    pub fn min_value(&self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let node_data: Option<T>;
            if self.root.as_ref().unwrap().borrow().left.is_none() {
                node_data = self.root.as_ref().unwrap().borrow().data.clone();
            } else {

                let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(self.root.as_ref().unwrap());

                loop {
                    if node_current.borrow().left.is_some() {
                        let node = Rc::clone(&node_current);
                        node_current = Rc::clone(node.borrow().left.as_ref().unwrap());
                    } else {
                        let node_c = Rc::clone(&node_current);
                        let node_c = node_c.borrow();
                        node_data = node_c.data.clone();
                        break;
                    }
                }
            }
            node_data
        }
    }

    pub fn remove(&mut self, data: T) -> Option<T> where T: Clone{
        if self.size == 0 {
            None
        } else {
            let mut node_data: Option<T> = None;
            if &data == self.root.as_ref().unwrap().borrow().data.as_ref().unwrap() {
                let node_root = Rc::clone(self.root.as_ref().unwrap());
                let mut node_root = node_root.borrow_mut();

                if node_root.right.is_none() && node_root.left.is_none() {
                    let nd = self.root.take().unwrap();
                    node_data = nd.borrow().data.clone();
                } else if node_root.right.is_none() {
                    self.root = node_root.left.take();
                } else if node_root.left.is_none() {
                    self.root = node_root.right.take();
                } else {
                    
                }
                self.size -= 1;
                
            } else {
                let root: Rc<RefCell<AVLNode<T>>> = self.root.take().unwrap();

                let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);
                let mut node_parent: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);

                let mut node_direction: NodeDirection = NodeDirection::NodeNone;

                loop {
                    if node_current.borrow().data.is_some() {
                        if &data == node_current.borrow().data.as_ref().unwrap() {
                            let node_c = Rc::clone(&node_current);
                            let mut node_c = node_c.borrow_mut();

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

                                break;
                            } else if node_c.right.is_none() {
                                match node_direction {
                                    NodeDirection::NodeLeft => {
                                        node_parent.borrow_mut().left = node_c.left.take();
                                    },
                                    NodeDirection::NodeRight => {
                                        node_parent.borrow_mut().right = node_c.left.take();
                                    },
                                    _ => {}
                                }
                                break;
                            } else if node_c.left.is_none() {
                                match node_direction {
                                    NodeDirection::NodeLeft => {
                                        node_parent.borrow_mut().left = node_c.right.take();
                                    },
                                    NodeDirection::NodeRight => {
                                        node_parent.borrow_mut().right = node_c.right.take();
                                    },
                                    _ => {}
                                }
                                break;
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

                        self.size -= 1;
                    } else {
                        break;
                    }
                }
                self.root = Some(root);

                
            }

            node_data
        }
    }

    pub fn remove_max(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let node_data: Option<T>;
            if self.root.as_ref().unwrap().borrow().right.is_none() {

                if self.root.as_ref().unwrap().borrow().left.is_none() {
                    let nd = self.root.take().unwrap();
                    node_data = nd.borrow_mut().data.clone();
                } else {
                    let nd = self.root.take().unwrap();
                    node_data = nd.borrow().data.clone();
                    self.root = nd.borrow_mut().left.take();
                    self.height_right -= 1;
                }

                self.size -= 1;
                
            } else {
                let root: Rc<RefCell<AVLNode<T>>> = self.root.take().unwrap();

                let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);
                let mut node_parent: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);

                loop {
                    if node_current.borrow().right.is_some() {
                        node_parent = node_current;
                        let node = Rc::clone(&node_parent);
                        node_current = Rc::clone(node.borrow().right.as_ref().unwrap());
                    } else {
                        let node_c = Rc::clone(&node_current);
                        let mut node_c = node_c.borrow_mut();

                        if node_c.left.is_none() {
                            let value = Rc::into_raw(node_parent.borrow_mut().right.take().unwrap());
                            node_data = node_c.data.take();
                        } else {
                            node_parent.borrow_mut().right = node_c.left.take();
                            node_data = node_c.data.take();
                        }
                        break;
                    }
                }
                self.size -= 1;

                self.root = Some(root);
            }

            self.rebalance();

            node_data
        }
    }

    pub fn remove_min(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let node_data: Option<T>;
            if self.root.as_ref().unwrap().borrow().left.is_none() {

                if self.root.as_ref().unwrap().borrow().right.is_none() {
                    let nd = self.root.take().unwrap();
                    node_data = nd.borrow_mut().data.clone();
                } else {
                    let nd = self.root.take().unwrap();
                    node_data = nd.borrow().data.clone();
                    self.root = nd.borrow_mut().right.take();
                    self.height_left -= 1;
                }

                self.size -= 1;
                
            } else {
                let root: Rc<RefCell<AVLNode<T>>> = self.root.take().unwrap();

                let mut node_current: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);
                let mut node_parent: Rc<RefCell<AVLNode<T>>> = Rc::clone(&root);

                loop {
                    if node_current.borrow().left.is_some() {
                        node_parent = node_current;
                        let node = Rc::clone(&node_parent);
                        node_current = Rc::clone(node.borrow().left.as_ref().unwrap());
                    } else {
                        let node_c = Rc::clone(&node_current);
                        let mut node_c = node_c.borrow_mut();

                        if node_c.right.is_none() {
                            let value = Rc::into_raw(node_parent.borrow_mut().left.take().unwrap());
                            node_data = node_c.data.take();
                        } else {
                            node_parent.borrow_mut().left = node_c.right.take();
                            node_data = node_c.data.take();
                        }
                        break;
                    }
                }
                self.size -= 1;

                self.root = Some(root);
            }
            self.rebalance();

            node_data
        }
    }

    pub fn sub_tree(&self, data: T) -> AVLTree<T> where T: Clone{
        unimplemented!();
    }

    #[inline(always)]
    fn height_node<'a>(node: &'a Option<Rc<RefCell<AVLNode<T>>>>) ->  isize {
        if node.is_none() { return 0 }

        let node_root = node.clone();

        let mut q = VecDeque::<Option<Rc<RefCell<AVLNode<T>>>>>::new();

        q.push_back(node_root.clone());

        let mut height: isize = 0;

        loop {
            let mut node_count: usize = q.len();
            if node_count == 0 {
                return height
            }

            height += 1;

            while node_count > 0
            {
                let node = q.pop_front().unwrap().unwrap();
                if node.borrow().left.is_some() {
                    q.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    q.push_back(node.borrow().right.clone());
                }
                node_count = node_count - 1;
            }
        }
    }

    #[inline]
    fn left_rotate(&mut self) {
        let root = self.root.take();
        let node_right: Option<Rc<RefCell<AVLNode<T>>>> = root.as_ref().unwrap().borrow_mut().right.take();

        let node_left: Option<Rc<RefCell<AVLNode<T>>>>;
        if node_right.is_some() {
            node_left = node_right.as_ref().unwrap().borrow_mut().left.take();
            root.as_ref().unwrap().borrow_mut().right = node_left;
            node_right.as_ref().unwrap().borrow_mut().left = root;
        }
        self.root = node_right;
    }

    #[inline(always)]
    fn left_rotate_node<'a>(node: &'a mut Option<Rc<RefCell<AVLNode<T>>>>) {
        if node.is_some() {
            let mut node_root: AVLNode<T> = node.as_ref().take().unwrap().borrow().clone();
            let node_right: Option<Rc<RefCell<AVLNode<T>>>> = node_root.right.take();

            let node_left: Option<Rc<RefCell<AVLNode<T>>>>;
            if node_right.is_some() {
                node_left = node_right.as_ref().unwrap().borrow_mut().left.take();
                node_root.right = node_left;
                node_right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(node_root.clone())));
            }

            *node = node_right;

        }
    }

    #[inline(always)]
    fn rebalance(&mut self) {
        if self.height_left > self.height_right {
            let height_diff = self.height_left - self.height_right; //left - right == -2
            
            if height_diff > 1 {
                let mut root = self.root.take();
                let mut node_left = root.as_mut().unwrap().borrow_mut().left.take();

                if node_left.is_some(){
                    let node_l = &node_left.as_ref().unwrap().borrow().left.clone();
                    let node_r = &node_left.as_ref().unwrap().borrow().right.clone();

                    if AVLTree::<T>::height_node(&node_r) > AVLTree::<T>::height_node(&node_l) { //left - right == 1                             
                        AVLTree::<T>::left_rotate_node(&mut node_left);
                    }
                    
                }
                root.as_mut().unwrap().borrow_mut().left = node_left;
                self.root = Option::clone(&root);
                
                self.right_rotate();

                self.height_right = self.height_right + 1;
                self.height_left = self.height_left();


            }
        } else {
            let height_diff = self.height_right - self.height_left; //left - right == 2

            if height_diff > 1 {
                let mut root = self.root.take();
                let mut node_right = root.as_mut().unwrap().borrow_mut().right.take();

                if node_right.is_some(){
                    let node_l = &node_right.as_ref().unwrap().borrow().left.clone();
                    let node_r = &node_right.as_ref().unwrap().borrow().right.clone();

                    if AVLTree::<T>::height_node(&node_l) > AVLTree::<T>::height_node(&node_r) { //left - right == -1
                        AVLTree::<T>::right_rotate_node(&mut node_right);
                    }
                    
                }
                root.as_mut().unwrap().borrow_mut().right = node_right;
                self.root = Option::clone(&root);

                self.left_rotate();

                self.height_right = self.height_right();
                self.height_left = self.height_left + 1;


            }
        }
    }

    #[inline]
    fn right_rotate(&mut self) {
        let root = self.root.take();
        let node_left: Option<Rc<RefCell<AVLNode<T>>>> = root.as_ref().unwrap().borrow_mut().left.take();

        let node_right: Option<Rc<RefCell<AVLNode<T>>>>;
        if node_left.is_some() {
            node_right = node_left.as_ref().unwrap().borrow_mut().right.take();
            root.as_ref().unwrap().borrow_mut().left = node_right;
            node_left.as_ref().unwrap().borrow_mut().right = root;
        }

        self.root = node_left;
    }

    #[inline(always)]
    fn right_rotate_node<'a>(node: &'a mut Option<Rc<RefCell<AVLNode<T>>>>) {
        if node.is_some() {
            let mut node_root: AVLNode<T> = node.as_ref().take().unwrap().borrow().clone();
            let node_left: Option<Rc<RefCell<AVLNode<T>>>> = node_root.left.take();

            let node_right: Option<Rc<RefCell<AVLNode<T>>>>;
            if node_left.is_some() {
                node_right = node_left.as_ref().unwrap().borrow_mut().right.take();
                node_root.left = node_right;
                node_left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(node_root.clone())));
            }

            *node = node_left;

        }
    }
}

#[allow(unused_mut, unused_variables, unused_assignments)]
impl <T> Display for AVLTree<T> where T: Clone + Display{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut column: usize = 0;
        writeln!(f, "AVLTree (size = {}): ", self.size);

        if self.root.is_some() {
            let mut node_vec: Vec<(&str ,usize, Rc<RefCell<AVLNode<T>>>)> = Vec::new();
            let mut node_vec_current: Vec<(&str, usize, Rc<RefCell<AVLNode<T>>>)> = vec![("T", column, Rc::clone(&self.root.as_ref().unwrap()))];

            let mut node_removed: Rc<RefCell<AVLNode<T>>> = Rc::new(RefCell::new(AVLNode {  data: None, 
                                                                                                    left: None,
                                                                                                    right: None, }));;

            while !node_vec_current.is_empty() {
                if node_vec_current[0].2.borrow().data.is_some() {
                    column = node_vec_current[0].1;
                    for i in 0..column {
                        write!(f, "¦\t");
                    }
                    writeln!(f, "{}: {}", node_vec_current[0].0 ,node_vec_current[0].2.borrow().data.as_ref().unwrap() );
                    node_removed = node_vec_current.remove(0).2;
                }

                if node_removed.borrow().right.is_some() {
                    node_vec_current.insert(0, ("R", column + 1, Rc::clone(node_removed.borrow().right.as_ref().unwrap())));
                }

                if node_removed.borrow().left.is_some() {
                    node_vec_current.insert(0, ("L", column + 1, Rc::clone(node_removed.borrow().left.as_ref().unwrap())));
                }

            }

        }
        write!(f, "")
    }
}

#[allow(unused_mut, unused_variables, unused_assignments)]
impl<T> Drop for AVLNode<T> where T: Clone{
    #[inline]
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

impl<T> From<Vec<T>> for AVLTree<T> where T: Clone + Ord + Debug{
    fn from(vec: Vec<T>) -> Self {
        let mut tree = AVLTree::<T>::new();
        for x in &vec {
            tree.insert(x.clone());
        }
        tree
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod test {

    use super::AVLTree;
    use super::BTreeOrder;
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

        // let now = Instant::now();
        let mut r = false;

        let repeat = 10;
        let mut time: u64 = 0;
        for _ in 0..repeat {
            let now = Instant::now();
            for i in 0..1000 {
                r = tree.insert(i);
            }
            let new_now = Instant::now();
            time += new_now.duration_since(now).as_secs() * 1_000_000 + new_now.duration_since(now).subsec_nanos() as u64
        }
        let time = time as f64 / repeat as f64;
        println!("time = {:?} ms", time / 1_000_000.0);

        // println!("depth = {:?}", tree.depth());
        // println!("height = {:?}", tree.height());

        // println!("height right = {:?}", &tree.height_right());

        // println!("height left = {:?}", &tree.height_left());

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

    #[test]
    fn left_rotation() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(44);
        tree.insert(30);
        tree.insert(76);
        tree.insert(16);
        tree.insert(39);
        // println!("{}", tree);
        tree.insert(15);
        // println!("{}", tree);

        // 30
        // |	16
        // |	|	15
        // |	44
        // |	|	39
        // |	|	76
    }

    #[test]
    fn right_rotation() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(-44);
        tree.insert(-30);
        tree.insert(-76);
        tree.insert(-16);
        tree.insert(-39);
        // println!("{}", tree);
        tree.insert(-15);
        // println!("{}", tree);

        // -30
        // |	-44
        // |	|	-76
        // |	|	-39
        // |	-16
        // |	|	-15
    }

    #[test]
    fn left_right_rotation() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(44);
        tree.insert(30);
        tree.insert(76);
        tree.insert(16);
        tree.insert(39);
        // println!("{}", tree);
        tree.insert(37);
        // println!("{}", tree);

        // 39
        // |	30
        // |	|	16
        // |	|	37
        // |	44
        // |	|	76
    }

    #[test]
    fn right_left_rotation() {
        let mut tree = AVLTree::<i32>::new();
        tree.insert(-44);
        tree.insert(-30);
        tree.insert(-76);
        tree.insert(-16);
        tree.insert(-39);
        // println!("{}", tree);
        tree.insert(-37);
        // println!("{}", tree);

        // -39
        // |	-44
        // |	|	-76
        // |	-30
        // |	|	-37
        // |	|	-16

    }

    #[test]
    fn from_vec() {
        let vec = vec![20, 10, 1, -1, 100, 32, 56];
        let _tree = AVLTree::from(vec);

        // println!("{}", tree);
        // T: 20
        // ¦	L: 10
        // ¦	¦	L: 1
        // ¦	¦	¦	L: -1
        // ¦	R: 100
        // ¦	¦	L: 32
        // ¦	¦	¦	R: 56

    }

    #[test]
    fn remove() {
        let vec = vec![20, 10, 1, -1, 100, 32, 56];
        let mut tree = AVLTree::from(vec);
        tree.remove(10);
        tree.remove(100);

        // println!("{}", tree);
        // T: 20
        // ¦	L: 1
        // ¦	¦	L: -1
        // ¦	R: 32
        // ¦	¦	R: 56

    }

    #[test]
    fn remove_max_and_min() {
        let vec = vec![20, 10, 1, -1, 100, 32, 56];
        let mut tree = AVLTree::from(vec);
        let max = tree.remove_max();
        assert_eq!(max, Some(100));
        let max = tree.remove_max();
        assert_eq!(max, Some(56));

        let min = tree.remove_min();
        assert_eq!(min, Some(-1));
        let min = tree.remove_min();
        assert_eq!(min, Some(1));

        // println!("{}", tree);
        // T: 20
        // ¦	L: 10
        // ¦	R: 32

        let vec = vec![100];
        let mut tree = AVLTree::from(vec);
        let max = tree.remove_max();
        assert_eq!(max, Some(100));
        let max = tree.remove_max();
        assert_eq!(max, None);

        let vec = vec![100];
        let mut tree = AVLTree::from(vec);
        let min = tree.remove_min();
        assert_eq!(min, Some(100));
        let min = tree.remove_min();
        assert_eq!(min, None);

    }

    #[test]
    fn max_and_min() {
        let vec = vec![20, 10, 1, -1, 100, 32, 56];
        let mut tree = AVLTree::from(vec);
        assert_eq!(tree.max_value(), Some(100));
        tree.remove_max();
        assert_eq!(tree.max_value(), Some(56));

        assert_eq!(tree.min_value(), Some(-1));
        tree.remove_min();
        assert_eq!(tree.min_value(), Some(1));

        let vec = vec![100];
        let mut tree = AVLTree::from(vec);
        assert_eq!(tree.max_value(), Some(100));
        tree.remove_max();
        assert_eq!(tree.max_value(), None);

        let vec = vec![100];
        let mut tree = AVLTree::from(vec);
        assert_eq!(tree.min_value(), Some(100));
        tree.remove_min();
        assert_eq!(tree.min_value(), None);

    }

    #[test]
    fn tree_contains() {
        let vec = vec![20, 10, 1, -1, 100, 32, 56];
        let tree = AVLTree::from(vec);
        assert_eq!(tree.contains(100), true);
        assert_eq!(tree.contains(1000), false);
        assert_eq!(tree.contains(1), true);
        let a = 32;
        assert_eq!(tree.contains(a), true);
        assert_eq!(a, 32);
    }

    #[test]
    fn tree_as_vec() {
        let vec = vec![23, 18, 44, 12, 20, 35, 52];
        let tree = AVLTree::from(vec);
        assert_eq!(tree.as_vec(BTreeOrder::PreOrder), vec![23, 18, 12, 20, 44, 35, 52]);
        assert_eq!(tree.as_vec(BTreeOrder::InOrder), vec![12, 18, 20, 23, 35, 44, 52]);
        // assert_eq!(tree.as_vec(BTreeOrder::PreOrder), vec![23, 18, 12, 20, 44, 35, 52]);
    }
}
