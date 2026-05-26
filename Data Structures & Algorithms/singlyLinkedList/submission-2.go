type LinkedList struct {
    head *Node 
}

type Node struct {
    val int
    next *Node
}

func NewLinkedList() *LinkedList {
    return &LinkedList{}
}

func (ll *LinkedList) Get(index int) int {
    i := 0
    cur := ll.head
    for cur != nil {
        if i == index {
            return cur.val
        }
        cur = cur.next
        i += 1
    }
    return -1 
}

func (ll *LinkedList) InsertHead(val int) {
    tmp := ll.head
    ll.head = &Node{val: val}
    ll.head.next = tmp
}

func (ll *LinkedList) InsertTail(val int) {
    if ll.head == nil {
        ll.InsertHead(val)
        return
    }
    prev := ll.head
    cur := prev.next
    for cur != nil {
        tmp := cur
        cur = cur.next
        prev = tmp
    }
    prev.next = &Node{val: val}
}

func (ll *LinkedList) Remove(index int) bool {
    i := 0
    var prev *Node = nil
    cur := ll.head
    for cur != nil {
        if i == index {
            if prev != nil {
                prev.next = cur.next
            } else {
                ll.head = cur.next
            }
            return true
        }
        tmp := cur
        cur = cur.next
        prev = tmp
        i += 1
    }
    return false
}

func (ll *LinkedList) GetValues() []int {
    res := []int{}
    cur := ll.head
    for cur != nil {
        res = append(res, cur.val)
        cur = cur.next
    }
    return res
}
