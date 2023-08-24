#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug + std::marker::Copy> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add(&mut self, element: T) {
        // Incorrect way to add an element
        // We would move the value head to the match statement
        // Which is not what we want
        //
        // match self.head {
        //     None => {
        //         self.head = Some(Box::new(Node { element, next: None }));
        //     }
        //     Some(previous_head) => {
        //         let new_head = Some(
        //             Box::new(Node {
        //                 element,
        //                 next: Some(previous_head),
        //             })
        //         );
        //         self.head = new_head;
        //     }
        // }

        let previous_head = self.head.take();

        let new_head = Box::new(Node {
            element,
            next: previous_head,
        });

        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(H) => Some(H.element),
            None => None,
        }
    }

    fn printing(&self) {
        let mut list_traversal = &self.head;
        loop {
            match list_traversal {
                Some(node) => {
                    println!("{:?}", node.element);
                    list_traversal = &node.next;
                }
                None => {
                    break;
                }
            }
        }
    }
}

pub fn linked_list_example() {
    let mut list = LinkedList::new();

    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    println!("The current list is {:?}", list);

    list.remove();
    println!("The after remove {:?}", list);
    println!("Peek {:?}", list.peek());
    list.printing();
}
