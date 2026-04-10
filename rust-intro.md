---
marp: true
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

<!-- _class: lead -->

# Thank You! 🦀

### Learn more at:
[google.github.io/comprehensive-rust](https://google.github.io/comprehensive-rust/hello-world/what-is-rust.html)

---
