
# leetcode
han's leetcode journey

# Complete DSA Problem List

This is a **curated DSA roadmap** with **300 problems**, divided into three stages: Foundations, Core, and Advanced.  
Solve them in order to **build strong problem-solving skills, pattern recognition, and interview readiness**.

## Table of Contents LeetCode and Deep-ML
- [Complete DSA Problem List](#complete-dsa-problem-list)
	- [Foundations (1–2 months) (84)](#foundations-12-months)
		- [Arrays & Strings (23)](#arrays--strings-23)
		- [Two Pointers & Sliding Window (11)](#two-pointers--sliding-window-11)
		- [Hashing (25)](#hashing-25)
		- [Stack & Queue (13)](#stack--queue-8)
		- [Recursion & Basic Math (12)](#recursion--basic-math-9)
	- [Core Algorithms & Structures (2–4 months) (143)](#core-algorithms--structures-24-months)
		- [Linked List (16)](#linked-list-16)
		- [Binary Search (18)](#binary-search-18)
		- [Sorting & Intervals (10)](#sorting--intervals-10)
		- [Two Pointers (10)](#two-pointers-10)
		- [Sliding Window (15)](#sliding-window-15)
		- [Trees (DFS / BFS) (18)](#trees-dfs--bfs-18)
		- [Heap / Priority Queue (15)](#heap--priority-queue-15)
		- [Basic Graphs (10)](#basic-graphs-10)
		- [Greedy (13)](#greedy-13)
		- [Intro Dynamic Programming (18)](#intro-dynamic-programming-19)
	- [Advanced Level (2–3 months) (73)](#advanced-level-23-months)
		- [Advanced Dynamic Programming (17)](#advanced-dynamic-programming-18)
		- [Backtracking (10)](#backtracking-10)
		- [Graph Algorithms (Dijkstra, Union-Find) (18)](#graph-algorithms-dijkstra-union-find-18)
		- [Trie (8)](#trie-8)
		- [Bit Manipulation (11)](#bit-manipulation-12)
		- [Interval Problems (5)](#interval-problems-5)
		- [Monotonic Stack / Queue (4)](#monotonic-stack--queue-5)
- [Notes](#notes)

---

## Foundations (1–2 months)

Goal: indexing, traversal, in-place logic, frequency counting, window control, recursion, and basic math.

> This section will include **~60 problems**.   

### Arrays & Strings (23)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Fizz Buzz](https://leetcode.com/problems/fizz-buzz) | [rust](src/foundations/arrays_and_strings/lc_0412_fizz_buzz.rs) | Easy | Primitive | Basic | 2 |
| 2 | [Two Sum](https://leetcode.com/problems/two-sum/) | [rust](src/foundations/arrays_and_strings/lc_0001_two_sum.rs) | Easy | Array | Hash / Two Pointers | 2 |
| 3 | [Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones) | [rust](src/foundations/arrays_and_strings/lc_0485_max_consecutive_ones.rs) | Easy | Array | Linear scan | 2 |
| 4 | [Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/) | [rust](src/foundations/arrays_and_strings/lc_0121_best_time_to_buy_and_sell_stock.rs) | Easy | Array | Greedy | 2 |
| 5 | [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/) | [rust](src/foundations/arrays_and_strings/lc_0217_contains_duplicate.rs) | Easy | Array | HashSet | 2 |
| 6 | [Find Pivot Index](https://leetcode.com/problems/find-pivot-index/) | [rust](src/foundations/arrays_and_strings/lc_0724_find_pivot_index.rs) | Easy | Array | Prefix Sum | 2 |
| 7 | [Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/) | [rust](src/foundations/arrays_and_strings/lc_0977_squares_of_a_sorted_array.rs) | Easy | Array | Two Pointers | 2 |
| 8 | [Plus One](https://leetcode.com/problems/plus-one/) | [rust](src/foundations/arrays_and_strings/lc_0066_plus_one.rs) | Easy | Array | Simulation | 2 |
| 9 | [Is Subsequence](https://leetcode.com/problems/is-subsequence/) | [rust](src/foundations/arrays_and_strings/lc_0392_is_subsequence.rs) | Easy | String | Two Pointers | 2 |
| 10 | [Move Zeroes](https://leetcode.com/problems/move-zeroes/) | [rust](src/foundations/arrays_and_strings/lc_0283_move_zeroes.rs) | Easy | Array | Two Pointers | 2 |
| 11 | [Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/) | [rust](src/foundations/arrays_and_strings/lc_0014_longest_common_prefix.rs) | Easy | String | Trie / Iteration | 1 |
| 12 | [Largest Number At Least Twice of Others](https://leetcode.com/problems/largest-number-at-least-twice-of-others) | [rust](src/foundations/arrays_and_strings/lc_0747_largest_number_at_least_twice_of_others.rs) | Easy | Array | greedy | 1 |
| 13 | [Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits) | [rust](src/foundations/arrays_and_strings/lc_1295_find_numbers_with_even_number_of_digits.rs) | Easy | Array | Counting | 1 |
| 14 | [Minimum Penalty for a Shop](https://leetcode.com/problems/minimum-penalty-for-a-shop) | [rust](src/foundations/arrays_and_strings/lc_2483_minimum_penalty_for_a_shop.rs) | Medium | Array | Prefix sum | 1 |
| 15 | [Apple Redistribution into Boxes](https://leetcode.com/problems/apple-redistribution-into-boxes) | [rust](src/foundations/arrays_and_strings/lc_3074_apple_redistribution_into_boxes.rs) | Easy | Array | greedy | 1 |
| 16 | [Coupon Code Validator](https://leetcode.com/problems/coupon-code-validator) | [rust](src/foundations/arrays_and_strings/lc_3606_coupon_code_validator.rs) | Easy | Array | Filter | 1 |
| 17 | [Maximize Happiness of Selected Children](https://leetcode.com/problems/maximize-happiness-of-selected-children/description) | [rust](src/foundations/arrays_and_strings/lc_3075_maximize_happiness_of_selected_children.rs) | Medium | Array | greedy | 1 |
| 18 | [N-Repeated Element in Size 2N Array](https://leetcode.com/problems/n-repeated-element-in-size-2n-array) | [rust](src/foundations/arrays_and_strings/lc_0961_n_repeated_element_in_size_2n_array.rs) | Medium | Array | greedy | 1 |
| 19 | [Majority Element](https://leetcode.com/problems/majority-element/) |  [rust](src/foundations/arrays_and_strings/lc_0169_majority_element.rs)| Easy | Array | HashMap / Boyer-Moore | 1 |
| 20 | [String Compression](https://leetcode.com/problems/string-compression/) |  | Medium | String | Two Pointers | 0 |
| 21 | [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/) |  | Medium | Array | Kadane's | 0 |
| 22 | [Rotate Array](https://leetcode.com/problems/rotate-array/) |  | Medium | Array | Reverse / Cyclic | 0 |
| 23 | [Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/) |  | Medium | Array | Prefix Products | 0 |

---

### Two Pointers & Sliding Window (11)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/) | [rust](src/foundations/two_pointers_and_sliding_window/lc_0125_valid_palindrome.rs) | Easy | String | Two Pointers | 1 |
| 2 | [Reverse String](https://leetcode.com/problems/reverse-string/) | [rust](src/foundations/two_pointers_and_sliding_window/lc_0344_reverse_string.rs) | Easy | String | Two Pointers | 1 |
| 3 | [Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/) | [rust](src/foundations/two_pointers_and_sliding_window/lc_0088_merge_sorted_array.rs) | Easy | Array | Two Pointers | 1 |
| 4 | [Remove Element](https://leetcode.com/problems/remove-element) | [rust](src/foundations/two_pointers_and_sliding_window/lc_0027_remove_element.rs) | Easy | Array | Two pointers | 1 |
| 5 | [Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/) | [rust](src/foundations/two_pointers_and_sliding_window/lc_0026_remove_duplicates_from_sorted_array.rs) | Easy | Array | Two Pointers | 1 |
| 6 | [Element Appearing More Than 25% in Sorted Array](https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array) | [rust](src/foundations/two_pointers_and_sliding_window/lc_1287_element_appearing_more_than_25_percent_in_sorted_array.rs) | Easy | Array | Sliding Window | 1 |
| 7 | [Container With Most Water](https://leetcode.com/problems/container-with-most-water/) |  | Medium | Array | Two Pointers | 0 |
| 8 | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) |  | Medium | String | Sliding Window | 0 |
| 9 | [Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/) |  | Medium | Array | Sliding Window | 0 |
| 10 | [Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/) |  | Medium | String | Sliding Window | 0 |
| 11 | [Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii/) |  | Medium | Array | Sliding Window | 0 |

---

### Hashing (25)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Valid Anagram](https://leetcode.com/problems/valid-anagram/) | [rust](src/foundations/arrays_and_strings/lc_0242_valid_anagram.rs) | Easy | String | Counter / HashMap | 1 |
| 2 | [First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/) |  | Easy | String | HashMap | 0 |
| 3 | [Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/) |  | Easy | Array | HashSet | 0 |
| 4 | [Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii/) |  | Easy | Array | HashSet | 0 |
| 5 | [Word Pattern](https://leetcode.com/problems/word-pattern/) |  | Easy | String | HashMap | 0 |
| 6 | [Happy Number](https://leetcode.com/problems/happy-number/) |  | Easy | Integer | HashSet | 0 |
| 7 | [Two Sum II – Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/) |  | Easy | Array | Two Pointers | 0 |
| 8 | [Ransom Note](https://leetcode.com/problems/ransom-note/) |  | Easy | String | HashMap | 0 |
| 9 | [Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/) |  | Easy | HashMap | Mapping | 0 |
| 10 | [Logger Rate Limiter](https://leetcode.com/problems/logger-rate-limiter/) |  | Easy | HashMap | Simulation | 0 |
| 11 | [Check if the Sentence Is Pangram](https://leetcode.com/problems/check-if-the-sentence-is-pangram/) |  | Easy | HashMap | Counting | 0 |
| 12 | [Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences/) |  | Easy | HashMap | Counting | 0 |
| 13 | [Destination City](https://leetcode.com/problems/destination-city/) |  | Easy | HashMap | Graph/Hashing | 0 |
| 14 | [Uncommon Words from Two Sentences](https://leetcode.com/problems/uncommon-words-from-two-sentences/) |  | Easy | HashMap | Counting | 0 |
| 15 | [Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/) |  | Medium | Array | Prefix Sum / HashMap | 0 |
| 16 | [Encode and Decode TinyURL](https://leetcode.com/problems/encode-and-decode-tinyurl/) |  | Medium | HashMap | Design | 0 |
| 17 | [Group Anagrams](https://leetcode.com/problems/group-anagrams/) |  | Medium | String | HashMap | 0 |
| 18 | [Valid Sudoku](https://leetcode.com/problems/valid-sudoku/) |  | Medium | Matrix | HashMap | 0 |
| 19 | [Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/) |  | Medium | HashMap | Hashing + Binary Search | 0 |
| 20 | [LRU Cache](https://leetcode.com/problems/lru-cache/) |  | Medium | HashMap | Design | 0 |
| 21 | [Minimum Operations to Reduce X to Zero](https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/) |  | Medium | HashMap | Prefix Sum | 0 |
| 22 | [Max Number of K-Sum Pairs](https://leetcode.com/problems/max-number-of-k-sum-pairs/) |  | Medium | HashMap | Hashing | 0 |
| 23 | [Subarray Sums Divisible by K](https://leetcode.com/problems/subarray-sums-divisible-by-k/) |  | Medium | HashMap | Prefix Sum | 0 |
| 24 | [Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/) |  | Medium | HashMap | Prefix Sum | 0 |
| 25 | [Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/) |  | Medium | Array | HashSet | 0 |

---

### Stack & Queue (13)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Crawler Log Folder](https://leetcode.com/problems/crawler-log-folder/) |  | Easy | Stack | Stack | 0 |
| 2 | [Final Prices With a Special Discount in a Shop](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/) |  | Easy | Stack | Stack | 0 |
| 3 | [Make The String Great](https://leetcode.com/problems/make-the-string-great/) |  | Easy | Stack/String | Stack | 0 |
| 4 | [Minimum String Length After Removing Substrings](https://leetcode.com/problems/minimum-string-length-after-removing-substrings/) |  | Easy | Stack/String | Stack | 0 |
| 5 | [Clear Digits](https://leetcode.com/problems/clear-digits/) |  | Easy | Stack | Stack | 0 |
| 6 | [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/) | [rust](src/foundations/stack_and_queue/lc_0020_valid_parentheses.rs) | Easy | Stack | Stack | 1 |
| 7 | [Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/) |  | Easy | Stack / Queue | Simulation | 0 |
| 8 | [Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/) |  | Easy | Queue / Stack | Simulation | 0 |
| 9 | [Next Greater Element I](https://leetcode.com/problems/next-greater-element-i/) |  | Easy | Stack | Monotonic Stack | 0 |
| 10 | [Baseball Game](https://leetcode.com/problems/baseball-game/) |  | Easy | Stack | Simulation | 0 |
| 11 | [Remove All Adjacent Duplicates in String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/) |  | Easy | Stack / String | Stack | 0 |
| 12 | [Min Stack](https://leetcode.com/problems/min-stack/) |  | Medium | Stack | Stack | 0 |
| 13 | [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/) |  | Medium | Stack | Monotonic Stack | 0 |

---

### Recursion & Basic Math (12)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Fibonacci Number](https://leetcode.com/problems/fibonacci-number/) | [rust](src/foundations/recursion_and_basic_math/lc_0509_fibonacci_number.rs) | Easy | Integer | Recursion | 1 |
| 2 | [Transpose Matrix](https://leetcode.com/problems/transpose-matrix) | [rust](src/foundations/recursion_and_basic_math/lc_0867_transpose_matrix.rs) | Easy | Array | Math | 1 |
| 3 | [Power of Two](https://leetcode.com/problems/power-of-two/) |  | Easy | Integer | Math | 0 |
| 4 | [Factorial Trailing Zeroes](https://leetcode.com/problems/factorial-trailing-zeroes/) |  | Easy | Integer | Math | 0 |
| 5 | [Roman to Integer](https://leetcode.com/problems/roman-to-integer/) |  | Easy | String | Math / Simulation | 0 |
| 6 | [Convert 1D Array Into 2D Array](https://leetcode.com/problems/convert-1d-array-into-2d-array/) |  | Easy | Array | Math | 0 |
| 7 | [Shift 2D Grid](https://leetcode.com/problems/shift-2d-grid/) |  | Easy | Array | Math | 0 |
| 8 | [Roman to Integer](https://leetcode.com/problems/roman-to-integer/) |  | Easy | String | Math | 0 |
| 9 | [Lucky Numbers in a Matrix](https://leetcode.com/problems/lucky-numbers-in-a-matrix/) |  | Easy | Matrix | Math | 0 |
| 10 | [Largest Local Values in a Matrix](https://leetcode.com/problems/largest-local-values-in-a-matrix/) |  | Easy | Matrix | Math | 0 |
| 11 | [Power of Four](https://leetcode.com/problems/power-of-four/) |  | Easy | Integer | Math | 0 |
| 12 | [Count Primes](https://leetcode.com/problems/count-primes/) |  | Medium | Integer | Sieve / Math | 0 |


---

## Core Algorithms & Structures (2–4 months)

*(Linked List, Binary Search, Sorting, Two Pointers, Sliding Window, Trees, Heap, Graphs, Greedy, Intro DP)*

> This section will include **120 problems**.  

### Linked List (16)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/) |  | Easy | Linked List | Linked List | 0 |
| 2 | [Remove Duplicates From Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/) |  | Easy | Linked List | Linked List | 0 |
| 3 | [Delete N Nodes After M Nodes of a Linked List](https://leetcode.com/problems/delete-n-nodes-after-m-nodes-of-a-linked-list/) |  | Easy | Linked List | Linked List | 0 |
| 4 | [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/) |  | Easy | Linked List | Two Pointers | 0 |
| 5 | [Intersection of Two Linked Lists](https://leetcode.com/problems/intersection-of-two-linked-lists/) |  | Easy | Linked List | Two Pointers | 0 |
| 6 | [Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/) |  | Easy | Linked List | Fast & Slow | 0 |
| 7 | [Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/) |  | Easy | Linked List | Fast & Slow | 0 |
| 8 | [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/) |  | Easy | Linked List | Two Pointers | 0 |
| 9 | [Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/) | [python](src/core/linked_list/lc_0141_linked_list_cycle.py) | Easy | Linked List | Fast & Slow | 1 |
| 10 | [Design Linked List](https://leetcode.com/problems/design-linked-list) | [rust](src/core/linked_list/lc_0707_design_a_linked_list.rs), [safe rust](src/core/linked_list/lc_0707_design_a_linked_list_safe.rs) | Medium | Linked List | Implementation | 1 |
| 11 | [Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/) |  | Medium | Linked List | Pointer Manipulation | 0 |
| 12 | [Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/) |  | Medium | Linked List | Recursion | 0 |
| 13 | [Rotate List](https://leetcode.com/problems/rotate-list/) |  | Medium | Linked List | Pointer Manipulation | 0 |
| 14 | [Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/) |  | Medium | Linked List | Two Pointers | 0 |
| 15 | [Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/) |  | Medium | Linked List | Cycle Detection | 0 |
| 16 | [Add Two Numbers](https://leetcode.com/problems/add-two-numbers/) |  | Medium | Linked List | Simulation | 0 |

### Binary Search (18)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Binary Search](https://leetcode.com/problems/binary-search/) | [rust](src/core/binary_search/lc_0704_binary_search.rs) | Easy | Array | Binary Search | 1 |
| 2 | [Count Negative Numbers in a Sorted Matrix](https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix) | [rust](src/core/binary_search/lc_1351_count_negative_numbers_in_a_sorted_matrix.rs) | Easy | Array | Binary search | 1 |
| 3 | [Search Insert Position](https://leetcode.com/problems/search-insert-position/) |  | Easy | Array | Binary Search | 0 |
| 4 | [Guess Number Higher Or Lower](https://leetcode.com/problems/guess-number-higher-or-lower/) |  | Easy | Integer | Binary Search | 0 |
| 5 | [Arranging Coins](https://leetcode.com/problems/arranging-coins/) |  | Easy | Integer | Binary Search | 0 |
| 6 | [Valid Perfect Square](https://leetcode.com/problems/valid-perfect-square/) |  | Easy | Integer | Binary Search | 0 |
| 7 | [Missing Number In Arithmetic Progression](https://leetcode.com/problems/missing-number-in-arithmetic-progression/) |  | Easy | Array | Binary Search | 0 |
| 8 | [Sqrt(x)](https://leetcode.com/problems/sqrtx/) |  | Easy | Math | Binary Search | 0 |
| 9 | [Check If a Number Is Majority Element in a Sorted Array](https://leetcode.com/problems/check-if-a-number-is-majority-element-in-a-sorted-array/) |  | Easy | Array | Binary Search | 0 |
| 10 | [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/) |  | Medium | Array | Binary Search | 0 |
| 11 | [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/) |  | Medium | Array | Binary Search | 0 |
| 12 | [Find First and Last Position of Element](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/) |  | Medium | Array | Binary Search | 0 |
| 13 | [Peak Element](https://leetcode.com/problems/find-peak-element/) |  | Medium | Array | Binary Search | 0 |
| 14 | [Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/) |  | Medium | Array | Binary Search on Answer | 0 |
| 15 | [Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/) |  | Medium | Array | Binary Search on Answer | 0 |
| 16 | [Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/) |  | Medium | Array | Binary Search | 0 |
| 17 | [Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/) |  | Hard | Array | Binary Search | 0 |
| 18 | [Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum/) |  | Hard | Array | Binary Search on Answer | 0 |

### Sorting & Intervals (10)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Relative Sort Array](https://leetcode.com/problems/relative-sort-array/) |  | Easy | Array | Counting Sort | 0 |
| 2 | [Meeting Rooms](https://leetcode.com/problems/meeting-rooms/) |  | Easy | Interval | Sorting | 0 |
| 3 | [Meeting Rooms II](https://leetcode.com/problems/meeting-rooms-ii/) |  | Medium | Heap | Sorting | 0 |
| 4 | [Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/) |  | Medium | Interval | Greedy | 0 |
| 5 | [Sort List](https://leetcode.com/problems/sort-list/) |  | Medium | Linked List | Merge Sort | 0 |
| 6 | [Largest Number](https://leetcode.com/problems/largest-number/) |  | Medium | Array | Custom Sort | 0 |
| 7 | [Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/) |  | Medium | Interval | Greedy | 0 |
| 8 | [Sort Colors](https://leetcode.com/problems/sort-colors/) |  | Medium | Array | Dutch Flag | 0 |
| 9 | [Merge Intervals](https://leetcode.com/problems/merge-intervals/) |  | Medium | Interval | Sorting | 0 |
| 10 | [Insert Interval](https://leetcode.com/problems/insert-interval/) |  | Medium | Interval | Sorting | 0 |

### Two Pointers (10)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/) |  | Easy | String | Two Pointers | 0 |
| 2 | [Sort Array by Parity](https://leetcode.com/problems/sort-array-by-parity/) |  | Easy | Array | Two Pointers | 0 |
| 3 | [Two Sum II](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/) |  | Medium | Array | Two Pointers | 0 |
| 4 | [3Sum](https://leetcode.com/problems/3sum/) |  | Medium | Array | Two Pointers | 0 |
| 5 | [4Sum](https://leetcode.com/problems/4sum/) |  | Medium | Array | Two Pointers | 0 |
| 6 | [Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/) |  | Medium | Array | Two Pointers | 0 |
| 7 | [Partition Labels](https://leetcode.com/problems/partition-labels/) |  | Medium | String | Greedy | 0 |
| 8 | [Valid Triangle Number](https://leetcode.com/problems/valid-triangle-number/) |  | Medium | Array | Two Pointers | 0 |
| 9 | [Compare Version Numbers](https://leetcode.com/problems/compare-version-numbers/) |  | Medium | String | Simulation | 0 |
| 10 | [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/) |  | Hard | Array | Two Pointers | 0 |

### Sliding Window (15)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Minimum Recolors to Get K Consecutive Black Blocks](https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/) |  | Easy | Array/String | Sliding Window | 0 |
| 2 | [Minimum Difference Between Highest And Lowest of K Scores](https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/) |  | Easy | Array | Sliding Window | 0 |
| 3 | [Defuse the Bomb](https://leetcode.com/problems/defuse-the-bomb/) |  | Easy | Array | Sliding Window | 0 |
| 4 | [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) |  | Medium | HashMap | Sliding Window | 0 |
| 5 | [Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/) |  | Medium | HashMap | Sliding Window | 0 |
| 6 | [Permutation in String](https://leetcode.com/problems/permutation-in-string/) |  | Medium | HashMap | Sliding Window | 0 |
| 7 | [Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii/) |  | Medium | Array | Sliding Window | 0 |
| 8 | [Fruits Into Baskets](https://leetcode.com/problems/fruit-into-baskets/) |  | Medium | HashMap | Sliding Window | 0 |
| 9 | [Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/) |  | Medium | HashMap | Sliding Window | 0 |
| 10 | [Maximum Points You Can Obtain from Cards](https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/) |  | Medium | Array | Sliding Window | 0 |
| 11 | [Longest Subarray of 1s After Deleting One Element](https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/) |  | Medium | Array | Sliding Window | 0 |
| 12 | [Count Number of Nice Subarrays](https://leetcode.com/problems/count-number-of-nice-subarrays/) |  | Medium | HashMap | Prefix Sum | 0 |
| 13 | [Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/) |  | Hard | Deque | Monotonic Queue | 0 |
| 14 | [Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/) |  | Hard | HashMap | Sliding Window | 0 |
| 15 | [Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/) |  | Hard | HashMap | Sliding Window | 0 |

### Trees (DFS / BFS) (18)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/) |  | Easy | Tree | DFS | 0 |
| 2 | [Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/) |  | Easy | Tree | DFS | 0 |
| 3 | [Binary Tree Postorder Traversal](https://leetcode.com/problems/binary-tree-postorder-traversal/) |  | Easy | Tree | DFS | 0 |
| 4 | [Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/) |  | Easy | Tree | DFS | 0 |
| 5 | [Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/) |  | Easy | Tree | DFS | 0 |
| 6 | [Symmetric Tree](https://leetcode.com/problems/symmetric-tree/) |  | Easy | Tree | DFS | 0 |
| 7 | [Same Tree](https://leetcode.com/problems/same-tree/) |  | Easy | Tree | DFS | 0 |
| 8 | [Lowest Common Ancestor of BST](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/) |  | Easy | Tree | DFS | 0 |
| 9 | [Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/) |  | Easy | Tree | DFS | 0 |
| 10 | [Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/) |  | Easy | Tree | DFS | 0 |
| 11 | [Path Sum](https://leetcode.com/problems/path-sum/) |  | Easy | Tree | DFS | 0 |
| 12 | [Path Sum II](https://leetcode.com/problems/path-sum-ii/) |  | Medium | Tree | DFS | 0 |
| 13 | [Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/) |  | Medium | Tree | BFS | 0 |
| 14 | [Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/) |  | Medium | Tree | BFS | 0 |
| 15 | [Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/) |  | Medium | Tree | DFS | 0 |
| 16 | [Construct Binary Tree from Preorder & Inorder](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) |  | Medium | Tree | Divide & Conquer | 0 |
| 17 | [Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/) |  | Medium | Tree | BFS | 0 |
| 18 | [Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/) |  | Hard | Tree | DFS | 0 |

### Heap / Priority Queue (15)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Kth Largest Element In a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/) |  | Easy | Heap | Heap/Priority Queue | 0 |
| 2 | [Last Stone Weight](https://leetcode.com/problems/last-stone-weight/) |  | Easy | Heap | Heap/Priority Queue | 0 |
| 3 | [Take Gifts From the Richest Pile](https://leetcode.com/problems/take-gifts-from-the-richest-pile/) |  | Easy | Heap | Heap/Priority Queue | 0 |
| 4 | [Final Array State After K Multiplication Operations I](https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/) |  | Easy | Heap | Heap/Priority Queue | 0 |
| 5 | [High Five](https://leetcode.com/problems/high-five/) |  | Easy | Heap | Heap/Priority Queue | 0 |
| 6 | [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/) |  | Medium | Heap | Selection | 0 |
| 7 | [Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/) |  | Medium | Heap | Frequency | 0 |
| 8 | [K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/) |  | Medium | Heap | Selection | 0 |
| 9 | [Task Scheduler](https://leetcode.com/problems/task-scheduler/) |  | Medium | Heap | Greedy | 0 |
| 10 | [Reorganize String](https://leetcode.com/problems/reorganize-string/) |  | Medium | Heap | Greedy | 0 |
| 11 | [Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/) |  | Hard | Heap | Two Heaps | 0 |
| 12 | [Merge K Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/) |  | Hard | Heap | K-way Merge | 0 |
| 13 | [Smallest Range Covering K Lists](https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/) |  | Hard | Heap | Sliding Window | 0 |
| 14 | [IPO](https://leetcode.com/problems/ipo/) |  | Hard | Heap | Greedy | 0 |
| 15 | [Sliding Window Median](https://leetcode.com/problems/sliding-window-median/) |  | Hard | Heap | Two Heaps | 0 |

### Basic Graphs (10)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Number of Islands](https://leetcode.com/problems/number-of-islands/) |  | Medium | Graph | DFS/BFS | 0 |
| 2 | [Max Area of Island](https://leetcode.com/problems/max-area-of-island/) |  | Medium | Graph | DFS | 0 |
| 3 | [Clone Graph](https://leetcode.com/problems/clone-graph/) |  | Medium | Graph | DFS | 0 |
| 4 | [Course Schedule](https://leetcode.com/problems/course-schedule/) |  | Medium | Graph | Topological Sort | 0 |
| 5 | [Course Schedule II](https://leetcode.com/problems/course-schedule-ii/) |  | Medium | Graph | Topological Sort | 0 |
| 6 | [Rotting Oranges](https://leetcode.com/problems/rotting-oranges/) |  | Medium | Graph | BFS | 0 |
| 7 | [Pacific Atlantic Water Flow](https://leetcode.com/problems/pacific-atlantic-water-flow/) |  | Medium | Graph | DFS | 0 |
| 8 | [Graph Valid Tree](https://leetcode.com/problems/graph-valid-tree/) |  | Medium | Graph | Union-Find | 0 |
| 9 | [Is Graph Bipartite?](https://leetcode.com/problems/is-graph-bipartite/) |  | Medium | Graph | BFS | 0 |
| 10 | [Shortest Path in Binary Matrix](https://leetcode.com/problems/shortest-path-in-binary-matrix/) |  | Medium | Graph | BFS | 0 |

### Greedy (13)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Buy Two Chocolates](https://leetcode.com/problems/buy-two-chocolates/) |  | Easy | Array/String | Greedy | 0 |
| 2 | [Minimum Number of Moves to Seat Everyone](https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/) |  | Easy | Array | Greedy | 0 |
| 3 | [Maximum Odd Binary Number](https://leetcode.com/problems/maximum-odd-binary-number/) |  | Easy | String | Greedy | 0 |
| 4 | [Maximum Nesting Depth of the Parentheses](https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/) |  | Easy | String | Greedy | 0 |
| 5 | [Check if One String Swap Can Make Strings Equal](https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/) |  | Easy | String | Greedy | 0 |
| 6 | [Lemonade Change](https://leetcode.com/problems/lemonade-change/) |  | Easy | Array | Greedy | 0 |
| 7 | [Jump Game](https://leetcode.com/problems/jump-game/) |  | Medium | Array | Greedy | 0 |
| 8 | [Jump Game II](https://leetcode.com/problems/jump-game-ii/) |  | Medium | Array | Greedy | 0 |
| 9 | [Gas Station](https://leetcode.com/problems/gas-station/) |  | Medium | Array | Greedy | 0 |
| 10 | [Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/) |  | Medium | Array | Greedy | 0 |
| 11 | [Hand of Straights](https://leetcode.com/problems/hand-of-straights/) |  | Medium | HashMap | Greedy | 0 |
| 12 | [Minimum Add to Make Parentheses Valid](https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/) |  | Medium | Stack | Greedy | 0 |
| 13 | [Candy](https://leetcode.com/problems/candy/) | [rust](src/core/greedy/lc_0135_candy.rs) | Hard | Array | Greedy | 1 |

### Intro Dynamic Programming (18)
| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|----------|----------------------|----------------------|------------|----------------|-----------|-----------|
| 1 | [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/) |  | Easy | DP | Fibonacci | 0 |
| 2 | [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/) |  | Easy | DP | Dynamic Programming | 0 |
| 3 | [Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/) |  | Easy | DP | Fibonacci | 0 |
| 4 | [Coin Change](https://leetcode.com/problems/coin-change/) |  | Medium | DP | Unbounded Knapsack | 0 |
| 5 | [Coin Change II](https://leetcode.com/problems/coin-change-ii/) |  | Medium | DP | Unbounded Knapsack | 0 |
| 6 | [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/) |  | Medium | DP | Binary Search | 0 |
| 7 | [Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/) |  | Medium | DP | State Tracking | 0 |
| 8 | [Unique Paths](https://leetcode.com/problems/unique-paths/) |  | Medium | DP | Grid DP | 0 |
| 9 | [Unique Paths II](https://leetcode.com/problems/unique-paths-ii/) |  | Medium | DP | Grid DP | 0 |
| 10 | [Decode Ways](https://leetcode.com/problems/decode-ways/) |  | Medium | DP | State Transition | 0 |
| 11 | [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) |  | Medium | DP | 0/1 Knapsack | 0 |
| 12 | [Word Break](https://leetcode.com/problems/word-break/) |  | Medium | DP | Prefix DP | 0 |
| 13 | [Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/) |  | Medium | DP | Order-sensitive DP | 0 |
| 14 | [House Robber](https://leetcode.com/problems/house-robber/) |  | Medium | DP | State Transition | 0 |
| 15 | [House Robber II](https://leetcode.com/problems/house-robber-ii/) |  | Medium | DP | Circular DP | 0 |
| 16 | [Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/) |  | Medium | DP | 2D DP | 0 |
| 17 | [Target Sum](https://leetcode.com/problems/target-sum/) |  | Medium | DP | Knapsack | 0 |
| 18 | [Edit Distance](https://leetcode.com/problems/edit-distance/) |  | Hard | DP | 2D DP | 0 |


---

# Advanced Level (2–3 months)

This is a curated list of **60 advanced DSA problems** covering **Dynamic Programming, Backtracking, Graphs, Trie, Bit Manipulation, Intervals, and Monotonic Stack/Queue**.  
Solve these to master **advanced patterns, state design, and algorithmic thinking**.

> This section contains **60 problems**.
---

### Advanced Dynamic Programming (17)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Integer Break](https://leetcode.com/problems/integer-break/) |  | Medium | Integer | DP | 0 |
| 2 | [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/) |  | Medium | String | DP / Expand Around Center | 0 |
| 3 | [Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/) |  | Medium | String | DP / Two Pointers | 0 |
| 4 | [Interleaving String](https://leetcode.com/problems/interleaving-string/) |  | Medium | String | DP | 0 |
| 5 | [Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences/) |  | Hard | String | DP | 0 |
| 6 | [Decode Ways II](https://leetcode.com/problems/decode-ways-ii/) |  | Hard | String | DP | 0 |
| 7 | [Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/) |  | Hard | Array | DP (state machine) | 0 |
| 8 | [Best Time to Buy and Sell Stock IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/) |  | Hard | Array | DP | 0 |
| 9 | [Burst Balloons](https://leetcode.com/problems/burst-balloons/) |  | Hard | Array | Interval DP | 0 |
| 10 | [Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/) |  | Hard | String | DP | 0 |
| 11 | [Wildcard Matching](https://leetcode.com/problems/wildcard-matching/) |  | Hard | String | DP | 0 |
| 12 | [Partition to K Equal Sum Subsets](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/) |  | Hard | Array | DP + Bitmask | 0 |
| 13 | [Longest Increasing Subsequence)](https://leetcode.com/problems/longest-increasing-subsequence/) |  | Hard | Array | DP + Binary Search | 0 |
| 14 | [Count Different Palindromic Subsequences](https://leetcode.com/problems/count-different-palindromic-subsequences/) |  | Hard | String | DP | 0 |
| 15 | [Shortest Common Supersequence](https://leetcode.com/problems/shortest-common-supersequence/) |  | Hard | String | DP | 0 |
| 16 | [Minimum Difficulty of a Job Schedule](https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/) |  | Hard | Array | DP | 0 |
| 17 | [Cherry Pickup](https://leetcode.com/problems/cherry-pickup/) |  | Hard | Grid | DP | 0 |

---

### Backtracking (10)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Sum of All Subsets XOR Total](https://leetcode.com/problems/sum-of-all-subsets-xor-totals/) |  | Easy | Array | Backtracking | 0 |
| 2 | [Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/) |  | Medium | String | Backtracking | 0 |
| 3 | [Permutations](https://leetcode.com/problems/permutations/) |  | Medium | Array | Backtracking | 0 |
| 4 | [Permutations II](https://leetcode.com/problems/permutations-ii/) |  | Medium | Array | Backtracking | 0 |
| 5 | [Combination Sum](https://leetcode.com/problems/combination-sum/) |  | Medium | Array | Backtracking | 0 |
| 6 | [Combination Sum II](https://leetcode.com/problems/combination-sum-ii/) |  | Medium | Array | Backtracking | 0 |
| 7 | [Subsets](https://leetcode.com/problems/subsets/) |  | Medium | Array | Backtracking | 0 |
| 8 | [Subsets II](https://leetcode.com/problems/subsets-ii/) |  | Medium | Array | Backtracking | 0 |
| 9 | [Word Search](https://leetcode.com/problems/word-search/) |  | Medium | Grid | Backtracking | 0 |
| 10 | [Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/) |  | Medium | String | Backtracking | 0 |
| 11 | [N-Queens](https://leetcode.com/problems/n-queens/) |  | Hard | Matrix | Backtracking | 0 |

---

### Graph Algorithms (Dijkstra, Union-Find) (18)

| no | title (problem link) | solutio | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Island Perimeter](https://leetcode.com/problems/island-perimeter/) |  | Easy | Grid | Graph | 0 |
| 2 | [Island Perimeter](https://leetcode.com/problems/island-perimeter/) |  | Easy | Grid | Graph | 0 |
| 3 | [Verifying An Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/) |  | Easy | String | Graph | 0 |
| 4 | [Find the Town Judge](https://leetcode.com/problems/find-the-town-judge/) |  | Easy | Graph | Graph | 0 |
| 5 | [Flood Fill](https://leetcode.com/problems/flood-fill/) |  | Easy | Grid | Graph/DFS | 0 |
| 6 | [Verifying An Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/) |  | Easy | String | Graph | 0 |
| 7 | [Find the Town Judge](https://leetcode.com/problems/find-the-town-judge/) |  | Easy | Graph | Graph | 0 |
| 8 | [Flood Fill](https://leetcode.com/problems/flood-fill/) |  | Easy | Grid | Graph/DFS | 0 |
| 9 | [Network Delay Time](https://leetcode.com/problems/network-delay-time/) |  | Medium | Graph | Dijkstra | 0 |
| 10 | [Path With Minimum Effort](https://leetcode.com/problems/path-with-minimum-effort/) |  | Medium | Grid | Dijkstra | 0 |
| 11 | [Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/) |  | Medium | Graph | Dijkstra / BFS | 0 |
| 12 | [Number of Operations to Make Network Connected](https://leetcode.com/problems/number-of-operations-to-make-network-connected/) |  | Medium | Graph | Union-Find | 0 |
| 13 | [Accounts Merge](https://leetcode.com/problems/accounts-merge/) |  | Medium | Graph | Union-Find | 0 |
| 14 | [Evaluate Division](https://leetcode.com/problems/evaluate-division/) |  | Medium | Graph | DFS / Union-Find | 0 |
| 15 | [Redundant Connection](https://leetcode.com/problems/redundant-connection/) |  | Medium | Graph | Union-Find | 0 |
| 16 | [Redundant Connection II](https://leetcode.com/problems/redundant-connection-ii/) |  | Hard | Graph | Union-Find | 0 |
| 17 | [Minimum Cost to Make at Least One Valid Path](https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/) |  | Hard | Grid | Dijkstra | 0 |
| 18 | [Swim in Rising Water](https://leetcode.com/problems/swim-in-rising-water/) |  | Hard | Grid | Dijkstra / Union-Find | 0 |

---

### Trie (8)

| no | title (problem link) | solutio | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Count Prefix and Suffix Pairs I](https://leetcode.com/problems/count-prefix-and-suffix-pairs/) |  | Easy | Trie | Trie | 0 |
| 2 | [Counting Words With a Given Prefix](https://leetcode.com/problems/counting-words-with-a-given-prefix/) |  | Easy | Trie | Trie | 0 |
| 3 | [Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/) |  | Medium | Trie | Implementation | 0 |
| 4 | [Add and Search Word](https://leetcode.com/problems/add-and-search-word-data-structure-design/) |  | Medium | Trie | DFS | 0 |
| 5 | [Maximum XOR of Two Numbers in an Array](https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/) |  | Medium | Trie | Bitwise Trie | 0 |
| 6 | [Replace Words](https://leetcode.com/problems/replace-words/) |  | Medium | Trie | Prefix Matching | 0 |
| 7 | [Longest Word in Dictionary](https://leetcode.com/problems/longest-word-in-dictionary/) |  | Medium | Trie | DFS | 0 |
| 8 | [Word Search II](https://leetcode.com/problems/word-search-ii/) |  | Hard | Trie | DFS + Trie | 0 |

---

### Bit Manipulation (11)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Single Number](https://leetcode.com/problems/single-number/) |  | Easy | Array | Bit Manipulation | 0 |
| 2 | [Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/) |  | Easy | Integer | Bit Manipulation | 0 |
| 3 | [Counting Bits](https://leetcode.com/problems/counting-bits/) |  | Easy | Array | Bit Manipulation | 0 |
| 4 | [Add Binary](https://leetcode.com/problems/add-binary/) |  | Easy | String | Bit Manipulation | 0 |
| 5 | [Minimum Bit Flips to Convert Number](https://leetcode.com/problems/minimum-bit-flips-to-convert-number/) |  | Easy | Integer | Bit Manipulation | 0 |
| 6 | [Reverse Bits](https://leetcode.com/problems/reverse-bits/) |  | Easy | Integer | Bit Manipulation | 0 |
| 7 | [Counting Bits](https://leetcode.com/problems/counting-bits/) |  | Easy | Array | Bit DP | 0 |
| 8 | [Single Number](https://leetcode.com/problems/single-number/) |  | Easy | Integer | XOR | 0 |
| 9 | [Single Number II](https://leetcode.com/problems/single-number-ii/) |  | Medium | Integer | Bit Counting | 0 |
| 10 | [Maximum Product of Word Lengths](https://leetcode.com/problems/maximum-product-of-word-lengths/) |  | Medium | Array | Bitmask | 0 |
| 11 | [Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/) |  | Medium | Array | Bit Manipulation | 0 |

---

### Interval Problems (5)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Missing Ranges](https://leetcode.com/problems/missing-ranges/) |  | Easy | Array | Interval | 0 |
| 2 | [Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/) |  | Medium | Array | Greedy | 0 |
| 3 | [Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/) |  | Medium | Array | Greedy | 0 |
| 4 | [Employee Free Time](https://leetcode.com/problems/employee-free-time/) |  | Hard | Interval | Sweep Line | 0 |
| 5 | [Meeting Rooms III](https://leetcode.com/problems/meeting-rooms-iii/) |  | Hard | Heap | Sorting | 0 |

---

### Monotonic Stack / Queue (4)

| no | title (problem link) | solution | difficulty | data structure | algorithm | solved |
|---|---|---|---|---|---|---|
| 1 | [Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/) |  | Hard | Stack | Monotonic Stack | 0 |
| 2 | [Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/) |  | Hard | Matrix | Stack | 0 |
| 3 | [Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/) |  | Hard | Deque | Monotonic Queue | 0 |
| 4 | [Sum of Subarray Minimums](https://leetcode.com/problems/sum-of-subarray-minimums/) |  | Hard | Stack | Monotonic Stack | 0 |


---

## Notes

- Re-solve problems after **1–2 weeks** to retain patterns.  
- Focus on **deriving solutions**, not memorizing.  
- Use the `solution` column to link your own solution files.  
