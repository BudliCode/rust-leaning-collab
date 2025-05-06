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
type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

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
    item: Option<T>,
    next: Link<T>,
    prev: WeakLink<T>,
}

impl<T> Node<T> {
    fn new(item: Option<T>) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

//Struktur für den Kopf- und Endstueck:
struct DLList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

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

fn get_element<T>(link: &Link<T>) -> Option<T>{
    link.as_ref().unwrap().borrow_mut().item.take()
}

impl<T: Ord> DLList<T> {
    //Erstellen eine DLL mit Head und Tail
    fn new() -> Self {
        let head_node = Some(Rc::new(RefCell::new(Node::new(None))));
        let tail_node = Some(Rc::new(RefCell::new(Node::new(None))));
        set_next(&head_node, &tail_node);
        set_prev(&tail_node, &link_to_weak(&head_node));

        Self {
            head: head_node,
            tail: tail_node,
            size: 0,
        }
    }

    fn remove(&mut self, wert: T, node: Link<T>) {
        todo!("implementieren");
    }

    fn insert_before(&mut self, wert: T, node_after: Link<T>) {
        let node_before = get_prev(&node_after);

        let new_node = Some(Rc::new(RefCell::new(Node {
            item: Some(wert),
            next: node_after.clone(),
            prev: node_before.clone(),
        })));

        set_prev(&node_after, &link_to_weak(&new_node));
        set_next(&node_before.unwrap().upgrade(), &new_node);
    }

    //Funktion zum einfügen eines Elements am Ende (Rechts):
    fn push_back(&mut self, wert: T) {
        self.insert_before(wert, self.tail.clone());
    }

    fn push_front(&mut self, wert: T) {
        self.insert_before(wert, self.tail.clone().unwrap().borrow_mut().next.clone());
    }

    pub fn push(&mut self, wert: T) {
        // checken ob liste leer -> push front
        todo!("implementieren");
        // checken ob größer als letzen element -> push back
        todo!("implementieren");
        // checken ob kleiner als erstes element -> push front
        todo!("implementieren");

        // über liste iterieren einfügen bei richtiger größe
        let x = self.head.clone();
        while let Some(x) = get_next(&x) {
            if 
            todo!("implementieren");
        }
    }

    //Funktion zum entfernen des letzten Elements (Rechtes Element):
    pub fn pop_back(&mut self) {
        if self.size > 0 {}
        todo!("implementieren");
    }

    //Funktion zum entfernen des ersten Elements (Linkes Element):
    pub fn pop_front(&mut self) {
        if self.size > 0 {}
        todo!("implementieren");
    }

    //Gibt die Anzahl an Elementen im DLL zurück:
    pub fn len(&mut self) -> usize{
        self.size
    }
}
