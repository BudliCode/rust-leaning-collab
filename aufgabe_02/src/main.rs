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
struct Node<T>  {
    item: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

//Struktur für den Kopf- und Endstueck:
struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

fn main() {
    println!("Hello, world!");
}

impl<T> LinkedList<T>{

    //Erstellen eine DLL mit leerem Head und Leerem Tail
    fn new() -> Self {
        Self { head: (None), tail: (None) }
    }

    //Eine neue Node erstellen:
    fn create_new_node(item_node: T, next_node: Link<T>, prev_node: WeakLink<T>) -> Link<T>{
        
        Some(Rc::new(RefCell::new(Node{
            item: item_node,
            next: next_node,
            prev: prev_node,
        })))   
    }

    //einen type Link zu einem Type WeakLink umwandeln
    fn link_to_weak(link: &Link<T>) -> WeakLink<T> {
        link.as_ref().map(|rc| Rc::downgrade(rc))
    }

    //Funktion zum einfügen eines Elements am Ende (Rechts):
    pub fn DLL_push(&mut self, wert: T) {

        //Erstes Element in die leere DLL:
        if self.head.is_none() && self.tail.is_none(){

            //Neue Node Element erstellen
            let new_node = Rc::new(RefCell::new(Node{
                item: wert,
                next: None,
                prev: None
            }));

            self.head = Some(new_node);
        }
        //Zweites Element in die DLL einhängen:
        else if let Some(ref mut act_head) = self.head {

            //Neue Node Element erstellen
            let new_node = Rc::new(RefCell::new(Node{
                item: wert,
                next: None,
                prev: None,
            }));
            
            new_node.borrow_mut().prev = Self::link_to_weak(act_head.clone());
            act_head.borrow_mut().next = Some(new_node.clone());

            self.tail = Some(new_node);
        }
        

        
        

    }

    //Funktion zum entfernen des letzten Elements (Rechtes Element):
    pub fn DLL_pop() {}

    //Tauscht ein gegebenen Wert mit einem Element an beliebiger Stelle. Gibt das gewechselte Element zurück
    pub fn DLL_switch() {}

    //Gibt die Anzahl an Elementen im DLL zurück:
    pub fn DLL_len() {}

    fn iter_forward() {}

    
}
