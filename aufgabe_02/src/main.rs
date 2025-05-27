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

struct Node<T> {
    item: T,
    next: Option<Link<T>>,
    prev: Option<WeakLink<T>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

//Struktur für den Kopf- und Endstueck:

fn to_weak<T>(node: &Option<Link<T>>) -> Option<WeakLink<T>> {
    match node {
        Some(link) => Some(Rc::downgrade(&link)),
        None => None,
    }
}

fn get_next<T>(link: &Link<T>) -> Option<Link<T>> {
    link.as_ref().borrow_mut().next.clone()
}

fn set_next<T>(link: &Link<T>, next: Option<Link<T>>) {
    link.as_ref().borrow_mut().next = next;
}

fn get_prev<T>(link: &Link<T>) -> Option<WeakLink<T>> {
    link.as_ref().borrow_mut().prev.clone()
}

fn set_prev<T>(link: &Link<T>, prev: Option<WeakLink<T>>) {
    link.as_ref().borrow_mut().prev = prev;
}

struct DLList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T: Ord> DLList<T> {
    //Erstellen eine DLL mit Head und Tail
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, wert: T) {
        let mut node = self.head.clone();

        while let Some(ref n) = node {
            if n.as_ref().borrow().item >= wert {
                break;
            }
            node = get_next(n);
        }

        let new_node = Rc::new(RefCell::new(Node::new(wert)));
        let new_node_opt = Some(new_node.clone());

        //Node is None -> am ende einfügen

        match node.clone() {
            None => {
                match self.tail.clone() {
                    // Liste ist Leer
                    None => {
                        self.head = new_node_opt.clone();
                        self.tail = new_node_opt;
                    }
                    Some(tail_node) => {
                        // am Ende einfügen
                        set_next(&tail_node, new_node_opt.clone());
                        set_prev(&new_node, to_weak(&self.tail));
                        self.tail = new_node_opt;
                    }
                };
            }
            Some(node_after) => {
                match get_prev(&node_after).clone() {
                    //Insert at beginning
                    None => {
                        set_prev(&node_after, to_weak(&new_node_opt.clone()));
                        set_next(&new_node, Some(node_after.clone()));
                        self.head = new_node_opt.clone();
                    }

                    //Insert between two nodes
                    Some(weak_node_before) => {
                        let node_before = weak_node_before.upgrade().unwrap();
                        set_prev(&node_after, to_weak(&new_node_opt.clone()));
                        set_next(&node_before, new_node_opt.clone());

                        set_prev(&new_node.clone(), Some(weak_node_before.clone()));
                        set_next(&new_node.clone(), node.clone());
                    }
                };
            }
        }
    }

    //Funktion zum entfernen des ersten Elements (Linkes Element):
    pub fn pop_front(&mut self) -> Option<T> {
        //Wenn die Liste leer ist, wird "none" zurückgegeben:
        if self.head.is_none() {
            return None;
        }

        //aktuellen Head übetragen und nächsten Knoten holen
        let old_head = self.head.take().unwrap();
        let next = get_next(&old_head);
        set_next(&old_head, None);

        //Wenn du nächste Konten leer ist, dann ist die Liste komplett leer und
        //head und Tail werden auf none gesetzt, wenn nicht wird der prev von der
        //nächsten Node gesetzt und der head auf den neuen Head gesetzt.
        match next {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(next_node) => {
                set_prev(&next_node, None);
                self.head = Some(next_node);
            }
        };

        //den Wert des alten Head ausgeben:
        Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().item)
    }

    //Funktion zum entfernen des letzten Elements (Rechtes Element):
    pub fn pop_back(&mut self) -> Option<T> {
        // Wenn die Liste leer ist, wird "None" zurückgegeben
        if self.tail.is_none() {
            return None;
        }

        // aktuellen Tail referenzieren und vorherigen Knoten holen
        let old_tail = self.tail.take().unwrap();
        let prev = get_prev(&old_tail);
        set_prev(&old_tail, None);

        // Wenn der vorherige Knoten leer ist, war das Element das einzige in der Liste
        // -> head und tail werden auf None gesetzt
        // Wenn nicht, dann wird der next-Zeiger des vorigen Knotens auf None gesetzt,
        // und der tail entsprechend aktualisiert

        match prev {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(prev_node) => {
                if let Some(prev_strong) = prev_node.upgrade() {
                    set_next(&prev_strong, None);
                    self.tail = Some(prev_strong);
                }
            }
        };

        // den Wert des alten Tails ausgeben
        Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().item)
        /*
                match Rc::try_unwrap(old_tail) {
                    Ok(node_cell) => {
                        let nodeS = node_cell.into_inner();
                        Some(nodeS.item)
                    }
                    Err(rc) => {
                        // Wenn noch andere Referenzen existieren, extrahieren wir trotzdem den Wert
                        let node_ref = rc.borrow();
                        //Some(node_ref.item.clone()) // T: Clone nötig
                        Some(node_ref.item)
                    }
                }
        */
    }

    pub fn to_vec(&mut self) -> Vec<T> {
        let mut out_vec: Vec<T> = Vec::new();

        while let Some(val) = self.pop_front() {
            out_vec.push(val);
        }

        out_vec
    }

    pub fn contains(&mut self, element: &T) -> bool {
        let mut current = self.head.clone();

        while let Some(ref curr) = current {
            if curr.borrow().item == *element {
                return true;
            }

            current = get_next(&curr);
        }

        return false;
    }
}

type DropLink<T> = Rc<RefCell<DropNode<T>>>;

struct DropNode<T> {
    item: T,
    next: Option<DropLink<T>>,
    prev: Option<DropLink<T>>,
}

impl<T> DropNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

struct DLListDrop<T> {
    head: Option<DropLink<T>>,
    tail: Option<DropLink<T>>,
}

impl<T> Drop for DLListDrop<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

impl<T: Ord> DLListDrop<T> {
    //Erstellen eine DLL mit Head und Tail
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn push(&mut self, wert: T) {
        let mut node = self.head.clone();

        while let Some(ref n) = node.clone() {
            if n.as_ref().borrow().item >= wert {
                break;
            }
            node = n.borrow().next.clone();
        }

        let new_node = Rc::new(RefCell::new(DropNode::new(wert)));
        let new_node_opt = Some(new_node.clone());

        //Node is None -> am ende einfügen

        match node.clone() {
            None => {
                match self.tail.clone() {
                    // Liste ist Leer
                    None => {
                        self.head = new_node_opt.clone();
                        self.tail = new_node_opt;
                    }
                    Some(tail_node) => {
                        // am Ende einfügen
                        tail_node.borrow_mut().next = new_node_opt.clone();
                        new_node.borrow_mut().prev = self.tail.clone();
                        self.tail = new_node_opt;
                    }
                };
            }
            Some(node_after) => {

                /*
                Hier gab es ein Panik, weil der  die node_after danach mehrfach referenziert wird, aber der mut noch im scope war.
                durch das let ... wird der Scope beendert und der borrwo_mut kann auf die node_after zugreifen.
                 */
                let maybe_node_after = node_after.borrow().prev.clone();

                match  maybe_node_after{
                    //Insert at beginning
                    None => {
                        node_after.borrow_mut().prev = new_node_opt.clone();
                        new_node.borrow_mut().next = Some(node_after.clone());
                        self.head = new_node_opt.clone();
                    }

                    //Insert between two nodes
                    Some(node_before) => {
                        node_after.borrow_mut().prev = new_node_opt.clone();
                        node_before.borrow_mut().next = new_node_opt.clone();

                        new_node.borrow_mut().prev = Some(node_before);
                        new_node.borrow_mut().next = node;
                    }
                };
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        //Wenn die Liste leer ist, wird "none" zurückgegeben:
        if self.head.is_none() {
            return None;
        }

        //aktuellen Head übetragen und nächsten Knoten holen
        let old_head = self.head.take().unwrap();
        let next = old_head.borrow().next.clone();

        //Wenn du nächste Konten leer ist, dann ist die Liste komplett leer und
        //head und Tail werden auf none gesetzt, wenn nicht wird der prev von der
        //nächsten Node gesetzt und der head auf den neuen Head gesetzt.
        match next {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(next_node) => {
                next_node.borrow_mut().prev = None;
                self.head = Some(next_node);
            }
        };

        //den Wert des alten Head ausgeben:
        Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().item)
    }
    pub fn pop_back(&mut self) -> Option<T> {
        // Wenn die Liste leer ist, wird "None" zurückgegeben
        if self.tail.is_none() {
            return None;
        }

        // aktuellen Tail referenzieren und vorherigen Knoten holen
        let old_tail = self.tail.take().unwrap();
        let prev = old_tail.borrow().prev.clone();

        // Wenn der vorherige Knoten leer ist, war das Element das einzige in der Liste
        // -> head und tail werden auf None gesetzt
        // Wenn nicht, dann wird der next-Zeiger des vorigen Knotens auf None gesetzt,
        // und der tail entsprechend aktualisiert

        match prev {
            None => {
                self.head = None;
                self.tail = None;
            }
            Some(prev_node) => {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            }
        };

        // den Wert des alten Tails ausgeben
        Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().item)
    }
    pub fn to_vec(&mut self) -> Vec<T> {
        let mut out_vec: Vec<T> = Vec::new();

        while let Some(val) = self.pop_front() {
            out_vec.push(val);
        }

        out_vec
    }
    

    pub fn contains(&mut self, element: &T) -> bool {
        let mut current = self.head.clone();

        while let Some(ref curr) = current.clone() {
            if curr.borrow_mut().item == *element {
                return true;
            }

            current = curr.borrow().next.clone();
        }

        return false;
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

//Weak tests:

    #[test]
    fn sort_test_weak() {
        let mut dll = DLList::<i32>::new();

        let value_vec = vec![8, 6, 17, 35, 888, 1, 0];

        for ele in value_vec {
            dll.push(ele);
        }

        let exp_vec = vec![0, 1, 6, 8, 17, 35, 888];

        assert_eq!(dll.to_vec(), exp_vec);
    }

    #[test]
    fn empty_list_function_test_weak() {
        let mut dll = DLList::<i32>::new();

        assert_eq!(dll.to_vec(), vec![]);
        assert_eq!(dll.pop_back(), None);
        assert_eq!(dll.pop_front(), None);
    }

    #[test]
    fn pop_front_pop_back_weak() {
        let mut dll = DLList::<i32>::new();

        let value_vec = vec![8, 6, 17, 35, 888, 1, 0];

        for ele in value_vec {
            dll.push(ele);
        }

        assert_eq!(dll.pop_front(), Some(0));
        assert_eq!(dll.pop_back(), Some(888));
        assert_eq!(dll.pop_back(), Some(35));
        assert_eq!(dll.pop_front(), Some(1));
    }

    #[test]
    fn contains_test_weak() {
        let mut dll = DLList::<i32>::new();

        //Test bei Leerer Liste
        assert_eq!(dll.contains(&18), false);

        let value_vec = vec![8, 6, 17, 35, 888, 1, 0];

        for ele in value_vec {
            dll.push(ele);
        }

        //Test bei voller Liste
        assert_eq!(dll.contains(&17), true);
        assert_eq!(dll.contains(&18), false);
    }

    #[test]
    fn stress_test_weak() {
        let mut dll = DLList::<i32>::new();

        //Liste mit Werten füllen
        for ele in 0..1000 {
            dll.push(ele);
        }

        let expected: Vec<_> = (0..1000).collect();
        assert_eq!(dll.to_vec(), expected);
    }

    #[test]
    fn memory_leak_weak() {
        let mut list = DLList::<i32>::new();
        list.push(10);
        list.push(20);
        list.push(30);

        // Zugriff auf Knoten für Überwachung
        let second_node = list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().clone();
        let weak_second = Rc::downgrade(&second_node);

        assert_eq!(Rc::strong_count(&second_node), 2);
        assert_eq!(Rc::weak_count(&second_node), 2); // durch prev

        // Entferne manuell alle Verbindungen (simuliert clear/drop)
        drop(second_node);
        drop(list); // falls du eine eigene clear() hast

        // Sollte jetzt kein Upgrade mehr möglich sein
        assert!(weak_second.upgrade().is_none());
    }

//Drop Test:
    #[test]
    fn sort_test_drop() {
        let mut dll = DLListDrop::<i32>::new();

        let value_vec = vec![8, 6, 17, 35, 888, 1, 0];

        for ele in value_vec {
            dll.push(ele);
        }

        let exp_vec = vec![0, 1, 6, 8, 17, 35, 888];

        assert_eq!(dll.to_vec(), exp_vec);
    }

    #[test]
    fn empty_list_function_test_drop() {
        let mut dll = DLListDrop::<i32>::new();

        assert_eq!(dll.to_vec(), vec![]);
        assert_eq!(dll.pop_back(), None);
        assert_eq!(dll.pop_front(), None);
    }

    #[test]
    fn pop_front_pop_back_drop() {
        let mut dll = DLListDrop::<i32>::new();

        let value_vec = vec![8, 6, 17, 35, 888, 1, 0];

        for ele in value_vec {
            dll.push(ele);
        }

        assert_eq!(dll.pop_front(), Some(0));
        assert_eq!(dll.pop_back(), Some(888));
        assert_eq!(dll.pop_back(), Some(35));
        assert_eq!(dll.pop_front(), Some(1));
    }

    #[test]
    fn contains_test_drop() {
        let mut dll = DLListDrop::<i32>::new();

        //Test bei Leerer Liste
        assert_eq!(dll.contains(&18), false);

        let value_vec = vec![8, 6, 17, 35, 888, 1, 0];

        for ele in value_vec {
            dll.push(ele);
        }

        //Test bei voller Liste
        assert_eq!(dll.contains(&17), true);
        assert_eq!(dll.contains(&18), false);
    }

    #[test]
    fn stress_test_drop() {
        let mut dll = DLListDrop::<i32>::new();

        //Liste mit Werten füllen
        for ele in 0..1000 {
            dll.push(ele);
        }

        let expected: Vec<_> = (0..1000).collect();
        assert_eq!(dll.to_vec(), expected);
    }

    #[test]
    fn memory_leak_drop() {
        let mut list = DLList::<i32>::new();
        list.push(10);
        list.push(20);
        list.push(30);

        // Zugriff auf Knoten für Überwachung
        let second_node = list.head.as_ref().unwrap().borrow().next.as_ref().unwrap().clone();
        //let weak_second = Rc::downgrade(&second_node);

        assert_eq!(Rc::strong_count(&second_node), 2);
        //assert_eq!(Rc::weak_count(&second_node), 2); // durch prev

        // Entferne manuell alle Verbindungen (simuliert clear/drop)
        drop(second_node);
        drop(list); // falls du eine eigene clear() hast

        // Sollte jetzt kein Upgrade mehr möglich sein
        //assert!(weak_second.upgrade().is_none());
    }

}
