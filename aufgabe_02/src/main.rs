use std::cell::RefCell;
use std::rc::{Rc, Weak};

/*Option
    bedeutet, dass der Pointer auch leer sein kann, ähnlich wie ein nullptr in C, aber sicherer
*/

/*RC & Weak

RC und Weak sind nicht mutable Zieger (also zeiger die nur Speicher lesen können aber nicht Schreiben)

Link: einen "leeren" Ref. counter (Rc) mit veränderlichen Zugriff (RefCell) vom Typ Node
    hat durch den Ref. Count eine Besitzanforderung auf den Speicher. Dadurch wird das Objekt vom Rc verfolgt.
    Rc liegt auf dem Heap.
    Rc zählt die zugriffe mit, soblad keine Zugriffe (Count = 0) mehr vorhanden sind, wird der Speicher freigegeben
    Bei einem RC-Zeiger, wird der Speicher nur freigegeben, wenn der Ref. Count auf 0 steht, es also keine Referenzen auf den Speicher mehr gibt.

WeakLink: wie link, "leere" Ref. counter (Rc) mit schwachen zugriff vom Type Node.
    hat keine Besitzanforderung auf den Speicher, also zählt die zugriffe nicht mit.
    liegt auch auf dem Heap.
    Weak zählt keine zugriffe, daher wir eine Speicher nicht nur gehalten, wenn ein Weak -Zeiger drauf zeigt.
    also wenn ein Weak-Zeiger auf einen Speicherplatz zeigt (egal was für eine art (i32, RC, Vec, etc)),
    sobald der Besitzer des Speichers sein Scope verlässt, dann wird der Speicher freigegeben, auch wenn der Weak-Pointer noch drauf zeigt.
*/

/*RefCell
RefCell<T> erlaubt es, Daten auch über einen Rc<RefCell<T>> zu verändern,
obwohl Rc keine mutable Referenzen erlaubt.
Weak-Zeiger müssen vorher mit .upgrade() in Rc umgewandelt werden, um Zugriff zu bekommen.
*/
type Link<T> = Rc<RefCell<Node<T>>>;
type WeakLink<T> = Weak<RefCell<Node<T>>>;

/*Datenstruktur für die Node:
<T> bedeutet, dass die struct mit einem beliebigen Datentyp verwendet werden kann,
der aber zur Compile-Zeit festgelegt wird.
Beispiel:
let liste1 = Node::<i32>::new();     // T = i32
let liste2 = Node::<String>::new();  // T = String

*/

/*Aufbau
Item: Speichert den DAten Inhalt
    Da das Item nur bei erstellen die Daten "geändert" werden

next: Zeiger auf nächste Node
    Pointer der auf die nächste Node nach Rechts zeigt. Da sich das ändern kann Link

prev: Zeiger auf vorherige Node
    Pointer auf die vorherige Node nach links. Da Elemente getauscht werden können (siehe DLL_switch) WeakLink

*/

struct InnerNode<T> {
    item: Rc<T>,
    next: Link<T>,
    prev: WeakLink<T>,
}

impl<T> InnerNode<T> {
    fn set_next(&self, link: &Link<T>) {}
}

struct InnerHead<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct InnerTail<T> {
    prev: Option<Rc<RefCell<Node<T>>>>,
}

enum Node<T> {
    Head(Rc<InnerHead<T>>),
    InnerNode(InnerNode<T>),
    Tail(Weak<InnerTail<T>>),
}

impl<T> Node<T> {
    fn get_next(&self) -> Option<Link<T>> {
        match self {
            Self::Head(head) => Some(head.next.clone()),
            Self::InnerNode(innernode) => Some(innernode.next.clone()),
            Self::Tail(tail) => None,
        }
    }

    fn get_prev(&self) -> Option<WeakLink<T>> {
        match self {
            Self::Head(head) => None,
            Self::InnerNode(innernode) => Some(innernode.prev.clone()),
            Self::Tail(tail) => Some(tail.prev.clone()),
        }
    }
}

//Struktur für den Kopf- und Endstueck:

//einen type Link zu einem Type WeakLink umwandeln
fn link_to_weak<T>(link: &Link<T>) -> WeakLink<T> {
    link.as_ref().map(|rc| Rc::downgrade(rc))
}

fn get_next<T>(link: &Link<T>) -> Link<T> {
    link.as_ref().unwrap().borrow_mut().next.clone()
}

fn set_next<T>(link: &Link<T>, next: &Link<T>) {
    link.as_ref().map(|node| {
        node.borrow_mut().next = next.clone();
    });
}

fn get_prev<T>(link: &Link<T>) -> WeakLink<T> {
    link.as_ref().unwrap().borrow_mut().prev.clone()
}

fn set_prev<T>(link: &Link<T>, prev: &WeakLink<T>) {
    link.as_ref().map(|node| {
        node.borrow_mut().prev = prev.clone();
    });
}

struct DLList<T> {
    head: Rc<RefCell<Node<T>>>,
    tail: Rc<RefCell<Node<T>>>,
}

impl<T: Ord> DLList<T> {
    //Erstellen eine DLL mit Head und Tail
    fn new() -> Self {
        let itailrc = Rc::new(InnerTail { prev: None });

        let tail = Node::Tail(Rc::downgrade(&itailrc.clone()));
        let final_tail = Rc::new(RefCell::new(tail));

        let headrc = Rc::new(InnerHead {
            next: Some(final_tail.clone()),
        });
        let head = Node::Head(headrc.clone());
        let final_head = Rc::new(RefCell::new(head));

        let final_head_clone = final_head.clone();
        if let Node::Head(mut head_node) = final_head_clone.as_ref().borrow_mut() {
            if let Some(i) = &head_node.next {
                if let Node::Tail(mut tail) = i.as_ref().borrow_mut() {
                    let x = tail.upgrade().unwrap();
                    x.as_ref().prev.replace(final_head.clone());
                }
            }
        }

        Self {
            head: final_head.clone(),
            tail: final_tail.clone(),
        }
    }

    fn remove(&mut self, node: &Link<T>) {
        let before = get_prev(&node);
        let after = get_next(&node);

        set_next(&before.as_ref().unwrap().upgrade(), &after);
        set_prev(&after, &before);
    }

    fn insert_before(&mut self, wert: T, node_after: Link<T>) {
        let node_before = get_prev(&node_after);
    }

    pub fn push(&mut self, wert: T) {
        let mut node: Link<T> = self.head.clone();

        while let Some(n) = node {
            if n.as_ref().get_mut().item >= wert {
                break;
            }
            node = get_next(&node);
        }

        let node_before = get_prev(&node);

        let new_node = Some(Rc::new(RefCell::new(Node {
            item: Some(wert),
            next: node_after.clone(),
            prev: node_before.clone(),
        })));

        set_prev(&node_after, &link_to_weak(&new_node));
        set_next(&node_before.unwrap().upgrade(), &new_node);
    }

    //Funktion zum entfernen des letzten Elements (Rechtes Element):
    pub fn pop_back(&mut self) -> Option<T> {
        if self.size > 0 {
            let to_remove = get_prev(&self.tail).as_ref().unwrap().upgrade();
            self.remove(&to_remove);
            return get_element(&to_remove);
        }
        return None;
    }

    //Funktion zum entfernen des ersten Elements (Linkes Element):
    pub fn pop_front(&mut self) -> Option<T> {
        if self.size > 0 {
            let to_remove = get_next(&self.head);
            self.remove(&to_remove);
            return get_element(&to_remove);
        }
        return None;
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut out_vec: Vec<T> = Vec::new();
        let mut current = get_next(&self.head);

        while let Some(curr_rc) = current {
            if let Some(val) = peek_element(&Some(curr_rc.clone())) {
                out_vec.push(val);
            }
            current = get_next(&Some(curr_rc));
        }

        out_vec
    }
}

#[cfg(test)]
mod tests {
    use crate::DLList;

    #[test]
    fn sorting_test() {
        let mut list = DLList::new();

        list.push(7);
        list.push(18);
        list.push(1);
        list.push(0);
        list.push(7);

        let expt_out = vec![0, 1, 7, 7, 18];
        print!("{:?}", list.to_vec());

        assert_eq!(list.to_vec(), expt_out);
    }
}
