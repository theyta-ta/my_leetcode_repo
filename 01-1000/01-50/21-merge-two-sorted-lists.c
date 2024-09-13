/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
    struct ListNode *head, *tail, *l = list1, *r = list2;
    if (l == NULL)
        return r;
    else if (r == NULL)
        return l;
    
    if (l->val > r->val) {
        head = r;
        r = r->next;
    } else {
        head = l;
        l = l->next;
    }
    tail = head;

    while (l != NULL && r != NULL) {
        if (l->val > r->val) {
            tail->next = r;
            r = r->next;
        } else {
            tail->next = l;
            l = l->next;
        }

        tail = tail->next;
    }

    if (l == NULL)
        tail->next = r;
    else
        tail->next = l;
    
    return head;
}
