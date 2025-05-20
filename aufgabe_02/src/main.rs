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

//wird gemacht:
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
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T: Ord> DLList<T> {
    //Erstellen eine DLL mit Head und Tail
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
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

    //wir gemacht
    pub fn push(&mut self, wert: T) {
        let new_node = Node::new(wert);

        let mut node = self.head.clone();

        while let Some(n) = node {
            
            //if n.as_ref().get_mut().item >= wert {
            //    break;
            //}

            if n.as_ref().item >= wert {
                break;
            }

            node = get_next(&node);
        }

        let node_before = get_prev(&node);
        let node_after =  get_next(&node);

        let new_node = Some(Rc::new(RefCell::new(Node {
            item: Some(wert),
            next: Some(node_after),
            prev: Some(node_before),
        })));

        set_prev(&node_after, &link_to_weak(&new_node));
        set_next(&node_before.unwrap().upgrade(), &new_node);
    }

    //Funktion zum entfernen des ersten Elements (Linkes Element):
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // Wert entnehmen
            let old_head_ref = Rc::try_unwrap(old_head).ok().expect("Multiple references").into_inner();
            let result = old_head_ref.item;

            // Nächsten Knoten zum neuen Head machen
            if let Some(next_node) = old_head_ref.next {
                // prev des neuen Heads auf None setzen
                next_node.borrow_mut().prev = None;
                self.head = Some(next_node);
            } else {
                // Liste wird leer
                self.tail = None;
            }

            result
        })
    }

    //Funktion zum entfernen des letzten Elements (Rechtes Element):
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail_rc| {
            let old_tail = Rc::try_unwrap(old_tail_rc)
                .ok()
                .expect("Tail still has other references")
                .into_inner();
            let result = old_tail.item;

            if let Some(prev_weak) = old_tail.prev {
                if let Some(prev_rc) = prev_weak.upgrade() {
                    // Trenne die Verbindung zum alten Tail
                    prev_rc.borrow_mut().next = None;
                    self.tail = Some(prev_rc);
                }
            } else {
                // Liste wird leer
                self.head = None;
            }

            result
        })
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut out_vec: Vec<T> = Vec::new();
        let mut current = self.head;

        while let Some(curr_rc) = current {
            if let Some(val) = self.pop_front() {
                out_vec.push(val);
            }
            current = Some(get_next(&Some(curr_rc)));
        }

        out_vec
    }

    pub fn contains(&mut self, element: &T) -> bool {

        let mut current = self.head.clone();
        
        while let Some(ref curr) = current {

            if curr.borrow().item == *element {
                return true;
            }

            current = Some(get_next(curr));

        }

        return false;
    }
}

pub fn main(){

}

