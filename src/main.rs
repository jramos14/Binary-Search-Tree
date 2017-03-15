// http://math.hws.edu/eck/cs225/s03/binary_trees/

//ONE STRUCT
//Deactivate program
//#[derive(Debug)]

use std::fmt::Debug;
use std::cmp::Ordering;
fn main() {

    let mut x : Tree<i32> = Tree::new();

    x.insert(6);
    println!("\nFirst Insert: {:?}\n", x);


    x.insert(5);
    println!("\nSecond Insert: {:?}\n", x);


    x.insert(4);

    println!("\nThird Insert: {:?}\n", x);




    x.insert(3);
    println!("\nFourth Insert: {:?}\n", x);


    x.insert(2);
    println!("\nFifth Insert: {:?}\n", x);
    x.insert(1);
    println!("\nSixth Insert: {:?}\n", x);

    //x.insert(4); // should be false

    //println!("{:?}", x);


    // I added the Debug trait to see the tree
    // to do so...
    // change the following:

        // impl<T: Ord> Tree<T> {
        // to
        // impl<T: Ord + Debug> Tree<T> {

    // and right above add these two lines...
    // add:

        // use std::fmt::Debug; <-- THIS ONE
        // #[derive(Debug)] <-- AND THIS ONE
        // pub struct Tree<T> { <-- THIS IS GIVEN

    x.find(&5);
    println!("\nFinding: {:?}\n", x);
    let y = x.find(&5);
    // should be false
    println!("\n{:?}\n", y);

    let pre = x.preorder();
    // should be [4,2,1,3,5,6]
    println!("pre {:?}", pre);


    let post = x.postorder();
    // should be [1,3,2,6,5,4]
    println!("post {:?}", post);


    let inor = x.inorder();
    // should be [1,2,3,4,5,6]
    println!("in {:?}", inor);

}



 // activate two structs
/*
                TWO STRUCTS
*/


#[derive(Debug)]
pub struct TreeNode<T> {
    node_key: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

//use std::fmt::Debug;
#[derive(Debug)]
pub struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

//#[derive(Debug)]
impl<T: Ord + Debug> TreeNode<T> {
    pub fn new(key: T) -> Self {
        TreeNode::<T> {
            node_key : key,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: T) -> bool {
        match self.node_key.cmp(&key) {
                Ordering::Greater => {

                    //self.left = Some(Box::new( Tree{ root: None }));
                    match self.left {
                        None => {
                            self.left = Some(Box::new( Tree{ root: Some(Box::new( TreeNode{ right:None, left: None, node_key: key })) }));
                    //self.node_key.insert(key);
                    //let result = Tree{ root: None }.insert(key);
                            return true

                        }
                        Some(ref mut left_node) => {
                            //println!("Left node Key is {:?}", key);
                            left_node.insert(key);
                            return true
                        }

            } // end match left
                    //return result;
                    //let result = Tree{ root: None }.insert(key);
                    //return result
                }

                Ordering::Less => {
                    match self.right {
                        None => {
                            self.right = Some(Box::new( Tree{ root: Some(Box::new( TreeNode{ right:None, left: None, node_key: key })) }));
                    //self.node_key.insert(key);
                    //let result = Tree{ root: None }.insert(key);
                            return true

                        }
                        Some(ref mut right_node) => {
                            //println!("Right node Key is {:?}", key);
                            right_node.insert(key);
                            return true
                        }

            } // end match right
                }

                Ordering::Equal => {
                    //println!("returns false");
                    return false

                }
            }   // end match
        } // end insert function

        pub fn find(&self, key: &T) -> bool {
            match self.node_key.cmp(&key) {
                    Ordering::Greater => {

                        //self.left = Some(Box::new( Tree{ root: None }));
                        match self.left {
                            None => {
                                //self.left = Some(Box::new( Tree{ root: Some(Box::new( TreeNode{ right:None, left: None, node_key: key })) }));
                        //self.node_key.insert(key);
                        //let result = Tree{ root: None }.insert(key);
                                return false

                            }
                            Some(ref left_node) => {
                                //println!("Left node Key is {:?}", key);
                                let result = left_node.find(&key);
                                //return false
                                return result
                            }

                } // end match left
                        //return result;
                        //let result = Tree{ root: None }.insert(key);
                        //return result
                    }

                    Ordering::Less => {
                        match self.right {
                            None => {
                                //self.right = Some(Box::new( Tree{ root: Some(Box::new( TreeNode{ right:None, left: None, node_key: key })) }));
                        //self.node_key.insert(key);
                        //let result = Tree{ root: None }.insert(key);
                                return false

                            }
                            Some(ref right_node) => {
                                //println!("Right node Key is {:?}", key);
                                let result = right_node.find(&key);
                                //return false
                                return result
                            }

                } // end match right
                    }

                    Ordering::Equal => {
                        //println!("returns false");
                        return true

                    }
                }   // end match
            } // end insert function

    pub fn preorder<'a>(&'a self, output: &mut Vec<&'a T>) {


                //println!("\nOutput vec before pushing in the key is: {:?}\n", output);
                output.push(&self.node_key);
                //println!("\nOutput vec before pushing in the key is: {:?}\n", output);
                match self.left {
                    Some(ref left_node) => {
                        //println!("\nOutput vec before pushing in the left keys is: {:?}\n", output);
                        let mut result : &mut Vec<&'a T> = &mut left_node.preorder();
                        //println!("Left Keys is : {:?}",result);

                        output.append(result);

                        //while result.len() > 0 {
                        //output.push(result.pop().unwrap());
                        //println!("\nOutput vec after pushing in the left keys is: {:?}\n", output);

                    //}
                    }
                    None => {

                    }
                }

                match self.right {
                    Some(ref right_node) => {
                        //println!("\nOutput vec before pushing in the right keys is: {:?}\n", output);
                        let mut result : &mut Vec<&'a T> = &mut right_node.preorder();
                        //println!("Right keys is {:?}",result);
                        //while result.len() > 0 {
                        output.append(result);
                    //}
                    }
                    None => {

                    }
                }








                //self.root.preorder(output);

            } // end preorder

    pub fn inorder<'a>(&'a self, output: &mut Vec<&'a T>) {



        match self.left {
            Some(ref left_node) => {
                //println!("\nOutput vec before pushing in the left keys is: {:?}\n", output);

                let mut result : &mut Vec<&'a T> = &mut left_node.inorder();
                //println!("Left Keys is : {:?}",result);

                output.append(result);


            }
            None => {

            }
        }

        //println!("\nOutput vec before pushing in the key is: {:?}\n", output);
        output.push(&self.node_key);
        //println!("\nOutput vec before pushing in the key is: {:?}\n", output);

        match self.right {
            Some(ref right_node) => {
                //println!("\nOutput vec before pushing in the right keys is: {:?}\n", output);
                let mut result : &mut Vec<&'a T> = &mut right_node.inorder();
                //println!("Right keys is {:?}",result);
                //while result.len() > 0 {
                output.append(result);
            //}
            }
            None => {

            }
        }

                        //self.root.preorder(output);

                    } // end inorder

pub fn postorder<'a>(&'a self, output: &mut Vec<&'a T>) {



            match self.left {
                Some(ref left_node) => {
                    //println!("\nOutput vec before pushing in the left keys is: {:?}\n", output);
                    let mut result : &mut Vec<&'a T> = &mut left_node.postorder();
                    //println!("Left Keys is : {:?}",result);

                    output.append(result);

                    //while result.len() > 0 {
                    //output.push(result.pop().unwrap());
                    //println!("\nOutput vec after pushing in the left keys is: {:?}\n", output);

                //}
                }
                None => {

                }
            }

            match self.right {
                Some(ref right_node) => {
                    //println!("\nOutput vec before pushing in the right keys is: {:?}\n", output);
                    let mut result : &mut Vec<&'a T> = &mut right_node.postorder();
                    //println!("Right keys is {:?}",result);
                    //while result.len() > 0 {
                    output.append(result);
                //}
                }
                None => {

                }
            }

            //println!("\nOutput vec before pushing in the key is: {:?}\n", output);
            output.push(&self.node_key);
            //println!("\nOutput vec before pushing in the key is: {:?}\n", output);








            //self.root.preorder(output);
} // end postorder



}   // end impl



impl<T: Ord + Debug> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree::<T> {
            root: None,
        }
    }





    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {


        match self.root {
            None => {

                self.root = Some(Box::new( TreeNode{ right:None, left: None, node_key: key }));
                //self.root = Some(TreeNode::new(key));
                return true
            }
            //Some(ref mut tree_node) => {
            Some(ref mut tree_node) => {
                //let node_Key = TreeNode { right: None, left: None, node_key: key };
                //if tree_node.node_key > key {
                    //let left_subtree = TreeNode { right: None, left: None, node_key :key };
                    //TreeNode{ node_key: None, right: None, left : Some(Box::new(Tree{ root: None }))}.insert(Some(key));
                    //TreeNode{ node_key: None, right: None, left : None }.insert(Some(key));
                    //tree_node.left = Some(Box::new( Tree{ root: None }));
                    tree_node.insert(key);
                    return false;
                //} else {
                    //return false
                //}

                /*match tree_node.node_key.cmp(&key) {
                    Ordering::Greater => {
                        tree_node.left.insert(key);
                        return false
                    }*/



                /*if key < TreeNode.node_key {
                    TreeNode.left.insert(key);
                }*/
                //}   // end match
            /*None => {
                self.root = Some(Box::new( TreeNode{ right:None, left: None, node_key: key }));


                return true
            }*/

            }
        }



}       // end insert function




    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root {
            None => {

                //self.root = Some(Box::new( TreeNode{ right:None, left: None, node_key: &key }));
                //self.root = Some(TreeNode::new(key));
                return false
            }
            //Some(ref mut tree_node) => {
            Some(ref tree_node) => {
                //let node_Key = TreeNode { right: None, left: None, node_key: key };
                //if tree_node.node_key > key {
                    //let left_subtree = TreeNode { right: None, left: None, node_key :key };
                    //TreeNode{ node_key: None, right: None, left : Some(Box::new(Tree{ root: None }))}.insert(Some(key));
                    //TreeNode{ node_key: None, right: None, left : None }.insert(Some(key));
                    //tree_node.left = Some(Box::new( Tree{ root: None }));
                    let result = tree_node.find(key);
                    return result
                    //return true
                //} else {
                    //return false
                //}

                /*match tree_node.node_key.cmp(&key) {
                    Ordering::Greater => {
                        tree_node.left.insert(key);
                        return false
                    }*/



                /*if key < TreeNode.node_key {
                    TreeNode.left.insert(key);
                }*/
                //}   // end match
            /*None => {
                self.root = Some(Box::new( TreeNode{ right:None, left: None, node_key: key }));


                return true
            }*/

            }
        }

    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
            let mut result : Vec<&T> = vec![];
            //self.root.preorder(&mut result);
            //self.preorder(self.root, &mut result);
            //TreeNode.preorder(&mut result);
            match self.root {
                None => {
                    unimplemented!();
                }
                Some(ref tree_node) => {
                    //result.push(tree_node);
                    //let output = tree_node.preorder(result);
                    tree_node.preorder(&mut result);
                    result

                }

            } // end match




    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut result : Vec<&T> = vec![];
        //self.root.preorder(&mut result);
        //self.preorder(self.root, &mut result);
        //TreeNode.preorder(&mut result);
        match self.root {
            None => {
                unimplemented!();
            }
            Some(ref tree_node) => {
                //result.push(tree_node);
                //let output = tree_node.preorder(result);
                tree_node.inorder(&mut result);
                result

            }

        } // end match
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut result : Vec<&T> = vec![];
        //self.root.preorder(&mut result);
        //self.preorder(self.root, &mut result);
        //TreeNode.preorder(&mut result);
        match self.root {
            None => {
                unimplemented!();
            }
            Some(ref tree_node) => {
                //result.push(tree_node);
                //let output = tree_node.preorder(result);
                tree_node.postorder(&mut result);
                result

            }

        } // end match
    }
}
  // end activation
