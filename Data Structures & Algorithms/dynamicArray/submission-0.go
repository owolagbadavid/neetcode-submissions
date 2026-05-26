type DynamicArray struct {
    capacity int;
    size int;
    list []int;
}

func NewDynamicArray(capacity int) *DynamicArray {
    return &DynamicArray{
        capacity: capacity,
        size: 0,
        list: make([]int, capacity),
    }
}

func (da *DynamicArray) Get(i int) int {
    return da.list[i]
}

func (da *DynamicArray) Set(i int, n int) {
    da.list[i] = n
}

func (da *DynamicArray) Pushback(n int) {
    if da.size == da.capacity {
        da.resize()
    }
    da.list[da.size] = n
    da.size += 1
}

func (da *DynamicArray) Popback() int {
    da.size -= 1
    res := da.list[da.size]
    return res
}

func (da *DynamicArray) resize() {
    da.capacity = da.capacity*2
    newList := make([]int, da.capacity)
    for i := range da.list {
        newList[i] = da.list[i]
    }
    da.list = newList
}

func (da *DynamicArray) GetSize() int {
    return da.size
}

func (da *DynamicArray) GetCapacity() int {
    return da.capacity
}
