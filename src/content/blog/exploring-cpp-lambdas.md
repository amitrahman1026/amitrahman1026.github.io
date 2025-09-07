---
title: "Lambdas in C++ 🦙🐑"
description: "A gentle introduction to operators, lambdas and functional programming in C++"
date: "2024-03-24"
tags: ["cpp", "lambdas", "functional-programming"]
draft: false
---

Today I'm doing to give a gentle introduction to operators, lambdas and
functional programming in c++. If you have never used them before in C++ and
always wanted to do so this is a good place to start!

## Prelude

If you haven't been able to tell yet, I'm quite fascinated by programming
languages and I'm learning to appreciate the different paradigms out there. One
thing that has caught my eye in the recent months would be declarative
programming.

This might be a little esoteric for the everyday software engineering job, I do
want to take some time to appreciate the consideration and features behind multi-paradigm programming languages and demonstrate how to use them.

<div style="text-align: center;">
    <img src="/3_paradigm.png" style="width: 50%;" alt="Imperative and Declarative Programming Paradigms">
</div>

In this post, I'll explore how C++ has evolved to embrace functional programming concepts while maintaining its imperative roots. We'll look at lambdas, function objects, and how they can make your code more expressive and maintainable.

## What Are Lambdas?

Lambda expressions in C++ are anonymous function objects that can be defined inline. They were introduced in C++11 and have become an essential tool for modern C++ development.

Here's the basic syntax:
```cpp
auto lambda = [capture](parameters) -> return_type {
    // function body
};
```

## Simple Examples

Let's start with something basic:

```cpp
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    
    // Simple lambda to print each number
    std::for_each(numbers.begin(), numbers.end(), 
        [](int n) { std::cout << n << " "; });
    
    return 0;
}
```

This is much cleaner than defining a separate function just to print numbers!

## Capture Lists

One of the most powerful features of lambdas is their ability to capture variables from their surrounding scope:

```cpp
int multiplier = 10;
auto multiply = [multiplier](int x) { return x * multiplier; };
```

You can capture by value `[multiplier]`, by reference `[&multiplier]`, or capture everything `[=]` or `[&]`.

## Functional Programming Patterns

Lambdas enable functional programming patterns in C++:

```cpp
// Map operation
std::transform(numbers.begin(), numbers.end(), results.begin(),
    [](int x) { return x * x; });

// Filter operation  
auto it = std::copy_if(numbers.begin(), numbers.end(), filtered.begin(),
    [](int x) { return x % 2 == 0; });

// Reduce operation
int sum = std::accumulate(numbers.begin(), numbers.end(), 0,
    [](int a, int b) { return a + b; });
```

## Practical Applications

Lambdas shine in:
- **Event handling**: Clean callback definitions
- **Algorithm customization**: Custom sorting, filtering
- **Async programming**: Task definitions
- **Template metaprogramming**: Type manipulation

## Conclusion

Lambdas in C++ provide a powerful way to write more expressive and functional code. They bridge the gap between imperative and functional programming paradigms, making C++ more versatile and enjoyable to work with.

Next time you find yourself writing a small function just to pass to an algorithm, consider using a lambda instead!