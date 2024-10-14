use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    pub first_element: usize,
    pub last_element: usize,
    pub current_page: usize,
    pub last_page: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paginator {
    item_number: usize,
    item_per_page: usize,
}

impl Paginator {
    pub fn new(item_number: usize, item_per_page: usize) -> Self {
        Self {
            item_number,
            item_per_page,
        }
    }
    pub fn page(&self, current_page: usize) -> Option<Page> {
        // no meaningful result in that case
        if self.item_per_page.eq(&0) || self.item_number.eq(&0) {
            return None;
        }
        let last_page = if self.item_number % self.item_per_page == 0 {
            self.item_number / self.item_per_page
        } else {
            self.item_number / self.item_per_page + 1
        };
        let first_element = self.item_per_page * (current_page - 1);
        match current_page.cmp(&last_page) {
            Ordering::Greater => None,
            Ordering::Equal => {
                let last_element = if self.item_number % self.item_per_page != 0 {
                    first_element + (self.item_number % self.item_per_page) - 1
                } else {
                    first_element + self.item_per_page - 1
                };
                Some(Page {
                    first_element,
                    last_element,
                    current_page,
                    last_page,
                })
            }
            Ordering::Less => {
                let last_element = first_element + self.item_per_page - 1;
                Some(Page {
                    first_element,
                    last_element,
                    current_page,
                    last_page,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_paginator_no_items() {
        let paginator = Paginator::new(0, 10);
        let page = paginator.page(1);
        assert_eq!(page, None);
    }
    #[test]
    fn test_paginator_no_items_no_items_per_page() {
        let paginator = Paginator::new(0, 0);
        let page = paginator.page(1);
        assert_eq!(page, None);
    }
    #[test]
    fn test_paginator_no_items_per_page() {
        let paginator = Paginator::new(100, 0);
        let page = paginator.page(1);
        assert_eq!(page, None);
    }
    #[test]
    fn test_paginator_after_last_page() {
        let paginator = Paginator::new(100, 30);
        let page = paginator.page(10);
        assert_eq!(page, None);
    }
    #[test]
    fn test_paginator_less_items_than_items_per_page() {
        let paginator = Paginator::new(10, 50);
        let page = paginator.page(1);
        let expected_page = Page {
            first_element: 0,
            last_element: 9,
            current_page: 1,
            last_page: 1,
        };
        assert_eq!(page, Some(expected_page));
    }
    #[test]
    fn test_paginator_more_items_than_items_per_page() {
        let paginator = Paginator::new(60, 50);
        let page1 = paginator.page(1);
        let page2 = paginator.page(2);
        let expected_page1 = Page {
            first_element: 0,
            last_element: 49,
            current_page: 1,
            last_page: 2,
        };
        let expected_page2 = Page {
            first_element: 50,
            last_element: 59,
            current_page: 2,
            last_page: 2,
        };
        assert_eq!(page1, Some(expected_page1));
        assert_eq!(page2, Some(expected_page2));
    }
    #[test]
    fn test_paginator_item_number_multiple_of_page_number_last_page() {
        let paginator = Paginator::new(100, 50);
        let page1 = paginator.page(1);
        let page2 = paginator.page(2);
        let expected_page1 = Page {
            first_element: 0,
            last_element: 49,
            current_page: 1,
            last_page: 2,
        };
        let expected_page2 = Page {
            first_element: 50,
            last_element: 99,
            current_page: 2,
            last_page: 2,
        };
        assert_eq!(page1, Some(expected_page1));
        assert_eq!(page2, Some(expected_page2));
    }
    #[test]
    fn test_paginator_more_then_two_page_item_number_multiple_of_page_number_last_page() {
        let paginator = Paginator::new(150, 50);
        let page1 = paginator.page(1);
        let page2 = paginator.page(2);
        let page3 = paginator.page(3);
        let expected_page1 = Page {
            first_element: 0,
            last_element: 49,
            current_page: 1,
            last_page: 3,
        };
        let expected_page2 = Page {
            first_element: 50,
            last_element: 99,
            current_page: 2,
            last_page: 3,
        };
        let expected_page3 = Page {
            first_element: 100,
            last_element: 149,
            current_page: 3,
            last_page: 3,
        };
        assert_eq!(page1, Some(expected_page1));
        assert_eq!(page2, Some(expected_page2));
        assert_eq!(page3, Some(expected_page3));
    }
    #[test]
    fn test_paginator_more_then_two_page_item_number_not_multiple_of_page_number_last_page() {
        let paginator = Paginator::new(140, 50);
        let page1 = paginator.page(1);
        let page2 = paginator.page(2);
        let page3 = paginator.page(3);
        let expected_page1 = Page {
            first_element: 0,
            last_element: 49,
            current_page: 1,
            last_page: 3,
        };
        let expected_page2 = Page {
            first_element: 50,
            last_element: 99,
            current_page: 2,
            last_page: 3,
        };
        let expected_page3 = Page {
            first_element: 100,
            last_element: 139,
            current_page: 3,
            last_page: 3,
        };
        assert_eq!(page1, Some(expected_page1));
        assert_eq!(page2, Some(expected_page2));
        assert_eq!(page3, Some(expected_page3));
    }
}
