pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
    let mut merged_list = LinkedList::new();
    let mut a_ptr = list_a.start;
    let mut b_ptr = list_b.start;

    while a_ptr.is_some() && b_ptr.is_some() {
        let a_val = unsafe { (*a_ptr.unwrap().as_ptr()).val };
        let b_val = unsafe { (*b_ptr.unwrap().as_ptr()).val };

        if a_val <= b_val {
            merged_list.add(a_val);
            a_ptr = unsafe { (*a_ptr.unwrap().as_ptr()).next };
        } else {
            merged_list.add(b_val);
            b_ptr = unsafe { (*b_ptr.unwrap().as_ptr()).next };
        }
    }

    // Add remaining elements from list_a
    while a_ptr.is_some() {
        let a_val = unsafe { (*a_ptr.unwrap().as_ptr()).val };
        merged_list.add(a_val);
        a_ptr = unsafe { (*a_ptr.unwrap().as_ptr()).next };
    }

    // Add remaining elements from list_b
    while b_ptr.is_some() {
        let b_val = unsafe { (*b_ptr.unwrap().as_ptr()).val };
        merged_list.add(b_val);
        b_ptr = unsafe { (*b_ptr.unwrap().as_ptr()).next };
    }

    merged_list
}
