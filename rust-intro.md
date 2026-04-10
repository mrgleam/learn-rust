---
marp: true
math: mathjax
theme: gaia
_class: lead
paginate: true
backgroundColor: #121212
color: #e0e0e0
style: |
  section {
    font-family: 'Outfit', sans-serif;
    color: #e0e0e0;
  }
  h1, h2 {
    color: #ff9d00;
  }
  footer {
    color: #888;
  }
  a {
    color: #4da6ff;
  }
  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }
---

![bg blur:2px opacity:40%](assets/rust_cover.png)

# **What is Rust?**
### A modern systems programming language
#### Focus on Reliability, Safety, and Performance

---

# Welcome to Day 1 🦀
### Rust Fundamentals

Today we cover a broad range of topics to build your foundation:

- **Basic Syntax**: Variables, types, enums, structs, references.
- **Functions & Methods**: How to structure logic.
- **Type Inference**: Let the compiler do the work.
- **Control Flow**: Loops, conditionals, and pattern matching.
- **User-Defined Types**: Structs and Enums in depth.

---

# Interactive Learning 💬

This course is designed to be interactive!

- **Ask Questions**: Ask them as they arise, don't wait until the end.
- **Discuss**: Share how Rust compares to languages you already know.
- **Hands-on**: We have exercises at the end of each segment.

---

# Schedule

Including 10 minute breaks, this session should take about 2 hours and 10 minutes. It contains:

| Segment | Duration |
| :--- | :--- |
| **Welcome** | 5 minutes |
| **Hello, World** | 15 minutes |
| **Types and Values** | 40 minutes |
| **Control Flow Basics** | 45 minutes |

---

# What is Rust?

Rust is a relatively new programming language that had its **1.0 release in 2015**.

- **Statically Compiled**: Like C++, it compiles to machine code.
- **LLVM Backend**: Uses the industrial-grade LLVM compiler framework.
- **Modern Syntax**: Combines low-level performance with modern ergonomic features.

---

# Platform & Architecture

Rust is highly portable and supports a vast range of environments.

<div class="columns">
<div>

### Architectures
- **x86** & **x86_64**
- **ARM** (Mobile & Servers)
- **WebAssembly** (WASM)

</div>
<div>

### OS Support
- **Linux**
- **MacOS**
- **Windows**

</div>
</div>

---

# Where is Rust Used?

From the smallest microchips to the largest cloud clusters.

- 🛠️ **Firmware** & Boot loaders
- 📱 **Smart Displays** & Mobile phones
- 💻 **Desktops**
- ☁️ **Servers** & Infrastructure

---

# Why Rust? (vs C++)

Rust fits in the same area as C++ but with modern guarantees.

- **High Flexibility & Control**: Manage memory and hardware directly.
- **No Garbage Collection**: Predictable performance and no runtime overhead.
- **Reliability & Safety**: Prevents memory leaks and crashes at compile time.
- **Scalable**: From microcontrollers to massive cloud systems.

<!--
Rust fits in the same area as C++:

- High flexibility.
- High level of control.
- Can be scaled down to very constrained devices such as microcontrollers.
- Has no runtime or garbage collection.
- Focuses on reliability and safety without sacrificing performance.

PRO TIP: Tailor this to your audience!
- C/C++ Devs: Emphasize memory safety without losing performance/control.
- Java/Python Devs: Emphasize that they get the same safety but with much faster, predictable performance and low-level access.
-->

---


# Benefits: Memory Safety

Rust prevents whole classes of memory bugs **at compile time**.

- ✅ **No NULL Pointers**: "The Billion Dollar Mistake" is gone.
- ✅ **No Double-Frees**: Automatic memory management without a GC.
- ✅ **No Data Races**: Thread safety is guaranteed by the compiler.
- ✅ **No Use-After-Free**: References are tracked and validated.

---

# Benefits: Reliability

What a Rust statement does is **never** left unspecified.

- 🛡️ **Bounds Checking**: Automatic checks for array and slice access.
- 🛡️ **Defined Overflow**: Integer overflow is defined (panic or wrap).
- 🛡️ **No Undefined Behavior**: The language eliminates "nasal demons."

<!--
👉 มันมาจากแนวคิด/ตัวอย่างในหนังสือ Gödel, Escher, Bach: An Eternal Golden Braid (GEB) ของ Douglas Hofstadter

ในเล่มนี้ Hofstadter จะชอบใช้คำว่า “demons” แบบเปรียบเทียบ (inspired จาก Maxwell’s demon) เพื่ออธิบาย “ตัวจัดการ/กฎบางอย่าง” ในระบบต่าง ๆ เช่น ภาษา ดนตรี คณิตศาสตร์

แล้ว “nasal demons” คืออะไรในบริบทนี้?

มันคือการพูดเชิงเล่นคำ/เปรียบเทียบว่า:
- เสียงนาสิก (nasal sounds เช่น m, n, ng) = สิ่งที่ทำให้ระบบ “ยุ่งยาก”
- เลยเรียกมันว่า “demons” (ตัวป่วน/ตัวร้าย)
- แล้วบอกว่า language eliminates nasal demons
  👉 หมายถึง “ภาษา (ที่กำลังพูดถึง) ถูกออกแบบให้ไม่มีเสียงนาสิก”

ทำไม Hofstadter ถึงพูดแบบนี้?
เพราะเขากำลังสื่อว่า:
- ระบบหนึ่ง (เช่น ภาษา) สามารถ “กำจัดความซับซ้อนบางอย่าง” ออกไปได้
- โดยการ ตั้งกฎ เช่น
  - ห้ามมีเสียงบางประเภท
  - ลดรูปแบบที่ยุ่งยาก

มันเป็นตัวอย่างของแนวคิดใหญ่ในเล่ม:
“ความซับซ้อนสามารถถูกควบคุมได้ด้วยกฎเชิงระบบ”
-->

---

# Benefits: Developer Experience

As expressive as Python/Swift, as fast as C++.

- 📦 **Cargo**: Built-in dependency manager and build tool.
- 🧪 **Built-in Testing**: First-class support for unit and doc tests.
- 🧩 **Modern Features**: Enums, pattern matching, and generics.
- ⚡ **Zero-Cost Abstractions**: Use high-level abstractions with no overhead.

---

# Rust Playground 🛝

The [Rust Playground](https://play.rust-lang.org/) is the easiest way to run Rust code without installing anything.

- ✨ **Formatting**: Use `Tools > rustfmt` to keep your code standard.
- ⚙️ **Profiles**: Switch between **Debug** (for speed) and **Release** (for performance).
- 🔍 **Inspecting**: Use `... > ASM` to see the generated assembly code.
- 🔗 **Sharing**: Perfect for sharing snippets and bug reports.

<!--
As students head into the break, encourage them to open up the playground and experiment a little. Encourage them to keep the tab open and try things out during the rest of the course. 

This is particularly helpful for advanced students who want to know more about Rust’s optimizations or generated assembly.
-->

---

# Types and Values 💎

<div class="columns">
<div>

This segment provides the foundational building blocks of Rust's type system.

**Total Duration**: ~40m

</div>
<div>

| Slide | Duration |
| :--- | :--- |
| **Hello, World** | 5m |
| **Variables** | 5m |
| **Values** | 5m |
| **Arithmetic** | 3m |
| **Type Inference** | 3m |
| **Exercise: Fibonacci** | 15m |

</div>
</div>

---

# Scalar Types: Integers 🔢

Rust provides a rich set of integer types with explicit widths.

<div class="columns">
<div>

### Signed (`i`)
- `i8`, `i16`, `i32`, `i64`, `i128`
- `isize` (pointer width)

</div>
<div>

### Unsigned (`u`)
- `u8`, `u16`, `u32`, `u64`, `u128`
- `usize` (pointer width)

</div>
</div>

> `isize` and `usize` depend on the architecture (e.g., 64-bit on a modern Mac/PC).

---

# Scalar Types: Float, Char, Bool

Beyond integers, Rust handles decimals and logic with precision.

- 📏 **Floating-Point**: `f32` (single precision) and `f64` (double precision).
- 🔠 **Characters**: `char` is **32 bits wide** (Unicode scalar value).
- ✅ **Boolean**: `bool` takes `true` or `false` (**8 bits wide**).

---

# Literal Syntax & Widths

Rust allows flexible syntax for specifying values and types.

- 💎 **Legibility**: Use underscores for large numbers: `1_000_000`.
- 🏷️ **Type Suffixes**: Explicitly type literals: `123_u32` or `3.14_f32`.
- 📐 **Bit Widths**:
  - `iN`/`uN`/`fN` are **N** bits wide.
  - `char` is always **4 bytes** (to handle all Unicode).
  - `bool` is **1 byte**.

---

# Exercise: Fibonacci (1/3) 🔄

The Fibonacci sequence begins with **0, 1**. For $n > 1$, the next number is the sum of the previous two ($F_n = F_{n-1} + F_{n-2}$).

- F(0) = 0
- F(1) = 1
- F(2) = 1
- F(3) = 2
- F(4) = 3
- ...

---

# Exercise: Fibonacci (2/3) 🔄

Write a function `fib(n)` that calculates the nth Fibonacci number. 

```rust
fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return todo!("Implement this");
    } else {
        // The recursive case.
        return todo!("Implement this");
    }
}
```

---

# Exercise: Fibonacci (3/3) 🔄

Test your implementation with `main`:

```rust
fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
```

<!--
- This exercise is a classic introduction to recursion.
- Encourage students to think about the base cases and the recursive step.
- The question “When will this function panic?” is a hint to think about integer overflow. The Fibonacci sequence grows quickly!
- Students might come up with an iterative solution as well, which is a great opportunity to discuss the trade-offs between recursion and iteration (e.g., performance, stack overflow for deep recursion).
-->

---

# Exercise: Collatz Sequence (1/2) 🔄

For an arbitrary $n$ greater than zero:
- If $n$ is **1**, the sequence terminates.
- If $n$ is **even**, the next value is $n / 2$.
- If $n$ is **odd**, the next value is $3 * n + 1$.

**Example (n = 3)**:
$3 \to 10 \to 5 \to 16 \to 8 \to 4 \to 2 \to 1$ (Length: 8)

---

# Exercise: Collatz Sequence (2/2) 🔄

Write a function to calculate the length of the Collatz sequence for a given `n`.

```rust
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    todo!("Implement this")
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
```

---

# Welcome Back 👋

Including 10 minute breaks, this session should take about 2 hours and 45 minutes. It contains:

| Segment | Duration |
| :--- | :--- |
| **Tuples and Arrays** | 35 minutes |
| **References** | 55 minutes |
| **User-Defined Types** | 1 hour |

---

# Exercise: Array Transpose (1/2) 🔄

Implement `transpose` to turn rows into columns.

<div class="columns">
<div>

### Example
```
101 102 103    101 201 301
201 202 203 -> 102 202 302
301 302 303    103 203 303
```

</div>
<div>

```rust
fn transpose(
    matrix: [[i32; 3]; 3]
) -> [[i32; 3]; 3] {
    todo!()
}
```

</div>
</div>

---

# Exercise: Array Transpose (2/2) 🔄

Test your implementation with the following `main` function:

```rust
fn main() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    for row in transposed {
        println!("{row:?}");
    }
}
```

---

# Exercise: Geometry (1/2) 📐

Create utility functions for 3-dimensional geometry, representing a point as `[f64; 3]`.

### Magnitude
Calculate the magnitude of a vector by summing the squares of its coordinates and taking the square root. Use the `sqrt()` method to calculate the square root, like `v.sqrt()`.

```rust
fn magnitude(v: &[f64; 3]) -> f64 {
    todo!()
}
```

---

# Exercise: Geometry (2/2) 📐

### Normalize
Normalize a vector by calculating its magnitude and dividing all of its coordinates by that magnitude.

```rust
fn normalize(v: &mut [f64; 3]) {
    todo!()
}

fn main() {
    println!("Magnitude of unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
```

---

# Constants: `const` (1/3) 💎

Constants are evaluated at compile time and their values are **inlined** wherever they are used.

- 🏗️ **Compile-time**: Evaluated when you build your program.
- 📂 **Inlined**: The value is copied into every place it's referenced.
- ⚙️ **`const fn`**: Functions marked `const` can be called at compile time.
- 🔄 **Runtime**: `const fn` can also be used like regular functions at runtime.

---

# Constants: `const` (2/3) 💎

Constants can be initialized with results from `const fn`.

```rust
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}
```

---

# Constants: `const` (3/3) 💎

Constants are often used for fixed-size arrays and global configuration.

```rust
fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello world");
    println!("digest: {digest:?}");
}
```

---


<!-- _class: lead -->

# Thank You! 🦀

### Learn more at:
[google.github.io/comprehensive-rust](https://google.github.io/comprehensive-rust/hello-world/what-is-rust.html)

---
