# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        rem = 0
        ret = ListNode()
        tail = ret

        while l1 is not None or l2 is not None:
            getval = lambda x: x.val if x is not None else 0
            getnext = lambda x: x.next if x is not None else None

            add = getval(l1) + getval(l2) + rem
            rem = add // 10
            
            tail.next = ListNode(add % 10)
            tail = tail.next

            l1 = getnext(l1)
            l2 = getnext(l2)

        if rem != 0:
            tail.next = ListNode(rem)
        
        return ret.next
