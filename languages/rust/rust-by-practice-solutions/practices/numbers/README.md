# Numbers - Integer Types

- **Signed integer**: Can represent both **positive** and **negative** integers
- **Unsigned integer**: Always **positive** integers

---

## All Possible Number Types

| Length  | Signed | Unsigned |
| :-----: | :----: | :------: |
|  8-bit  |   i8   |    u8    |
| 16-bit  |  i16   |   u16    |
| 32-bit  |  i32   |   u32    |
| 64-bit  |  i64   |   u64    |
| 128-bit |  i128  |   u128   |
|  arch   | isize  |  usize   |

## Numbers - Default Types

- Integers: **i32**
- Floats: **f64**

## Numbers - usize & isize

- Architecture dependent
- On 32-bit architecture: **32-bit**
- On 64-bit architecture: **64-bit**
- This is called Pointer sized integer type, matches size of a
  **word** in given platform
- **usize** gives the guarantee to be always big
  enough to hold any pointer or any offset in
  a data structure

### What is a word?

- Processor does NOT read 1 byte at a time from memory, reads 1 word at a time
- In a **32-bit** processor it can access 4 bytes (32-bits) at a time
- In a **64-bit** processor it can access 8 bytes (64-bits) at a time

## Numbers - Floating Point

According to IEEE-754 specification:

- f32 - size of 32 bits
- f64 - size of 64 bits

## Numbers - Boolean Logic

- Boolean logic deals with values that are either "true" or "false" (1 or 0)
- Three Basic operations: AND, OR, NOT

### Numbers - AND(&&)

|   A   |   B   |   **Q**   |
| :---: | :---: | :-------: |
| false | false | **false** |
| false | true  | **false** |
| true  | false | **false** |
| true  | true  | **true**  |

### Numbers - OR(||)

|   A   |   B   |   **Q**   |
| :---: | :---: | :-------: |
| false | false | **false** |
| false | true  | **true**  |
| true  | false | **true**  |
| true  | true  | **true**  |

### Numbers - NOT(!)

|  ! A   |   **Q**   |
| :----: | :-------: |
| !false | **true**  |
| !true  | **false** |

## Numbers - Bitwise Operations

- Operations that manipulate individual bits that make up a binary number
- Treating each bit of a binary number as a seperate unit and perform logical operations on them
- AND(&), OR(|), XOR(^) bitwise shifting

### Numbers - AND(&)

|  A  |  B  | **Q** |
| :-: | :-: | :---: |
|  0  |  0  | **0** |
|  0  |  1  | **0** |
|  1  |  0  | **0** |
|  1  |  1  | **0** |

### Numbers - OR(|)

|  A  |  B  | **Q** |
| :-: | :-: | :---: |
|  0  |  0  | **0** |
|  0  |  1  | **1** |
|  1  |  0  | **1** |
|  1  |  1  | **1** |

### Numbers - XOR(^)

|  A  |  B  | **Q** |
| :-: | :-: | :---: |
|  0  |  0  | **0** |
|  0  |  1  | **1** |
|  1  |  0  | **1** |
|  1  |  1  | **0** |
