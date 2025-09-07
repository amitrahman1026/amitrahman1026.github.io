---
title: "Dynamic Dispatch"
description: "A quick tutorial on Dynamic Dispatch and Function Overloading in C++ and Rust"
date: "2024-04-20"
tags: ["cpp", "rust", "dynamic-dispatch", "polymorphism"]
draft: false
---

Hi! Welcome to a quick no nonsense tutorial on Dynamic Dispatch and Function
Overloading. We'll take a look at how C++ and Rust does it!

<div style="text-align: center;">
    <img src="/4_mascot.png" style="width: 50%;" alt="Ferris doing cute stuff">
</div>

## Dynamic Dispatch

Dynamic dispatch is a **runtime** mechanism that allows a program to call a
function or method based on the actual type of the object, rather than the
declared type of the object. This is achieved through the use of virtual
functions and virtual method tables (vtables).

## C++ Implementation

In C++, dynamic dispatch is implemented through virtual functions:

```cpp
class Animal {
public:
    virtual void make_sound() = 0;  // Pure virtual function
    virtual ~Animal() = default;
};

class Dog : public Animal {
public:
    void make_sound() override {
        std::cout << "Woof!" << std::endl;
    }
};

class Cat : public Animal {
public:
    void make_sound() override {
        std::cout << "Meow!" << std::endl;
    }
};

void animal_sounds(Animal* animal) {
    animal->make_sound();  // Dynamic dispatch occurs here
}
```

The `virtual` keyword tells the compiler to use dynamic dispatch. At runtime, the correct `make_sound()` method is called based on the actual object type.

## Rust Implementation

Rust uses trait objects for dynamic dispatch:

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn animal_sounds(animal: &dyn Animal) {
    animal.make_sound();  // Dynamic dispatch occurs here
}
```

The `dyn` keyword indicates that this is a trait object using dynamic dispatch.

## Performance Considerations

Dynamic dispatch comes with a runtime cost:

1. **Vtable lookup**: Extra indirection to find the correct method
2. **Memory overhead**: Storing vtable pointers
3. **Optimization challenges**: Compiler can't inline virtual calls as easily

## When to Use Dynamic Dispatch

Use dynamic dispatch when:
- You need polymorphism at runtime
- The exact type isn't known at compile time
- You're working with collections of different types implementing the same interface

Avoid it when:
- Performance is critical and types are known at compile time
- You can use static dispatch (templates in C++, generics in Rust)

## Static vs Dynamic Dispatch

**Static Dispatch** (compile-time):
```rust
// Rust - generics provide static dispatch
fn animal_sounds<T: Animal>(animal: &T) {
    animal.make_sound();  // Resolved at compile time
}
```

**Dynamic Dispatch** (runtime):
```rust
// Rust - trait objects provide dynamic dispatch  
fn animal_sounds(animal: &dyn Animal) {
    animal.make_sound();  // Resolved at runtime
}
```

Here's a sequence diagram illustrating the dynamic dispatch process:

<div style="text-align: center;">
    <img src="/4_seq.png" style="width: 100%;" alt="dynamic dispatch sequence diagram">
</div>

Pretty much the same thing happens in Rust.

## Conclusion

Dynamic dispatch is a powerful tool for polymorphism, enabling flexible and extensible code. Both C++ and Rust provide elegant mechanisms for it, though with different syntax and trade-offs. Understanding when and how to use dynamic dispatch is crucial for writing effective systems code in either language.