# Cross-Platform RHI (Rendering Hardware Interface)

A lightweight, high-performance graphics abstraction library written in Rust, supporting **DirectX 12** (Windows), **Vulkan** (Linux / Windows), and **Metal** (macOS). Designed with multi-language interoperability, supporting **Rust**, **C**, **C++**, and **C#**.

## Features

* **Multi-Backend Support:** Abstracted architecture for DirectX 12, Vulkan, and Metal (macOS).
* **4-State Quaternary Resource State Machine:** Strict and precise resource state management using base-4 quaternary algebra (`00`, `01`, `10`, `11`).
* **High-Density State Packing:** Efficient packing of 4 quaternary states into a single 8-bit byte (`u8`) via `PackedQuatState`.
* **Bitflags 2.4 Integration:** Modern type-safe bitmask combinations for GPU resource states.
* **Multi-Language Interop:** Full **C-FFI** layer (`extern "C"`) allowing seamless integration with C, C++, and C#.
* **Modular Architecture:** Clean separation of concerns (`state`, `backend`, `device`, `resource`, `command`, `ffi`).
* **Built-in Testing & Safety:** Comprehensive unit tests and null-pointer safety checks for C-FFI bindings.

---

## Project Structure

- `src/lib.rs` - Library entry point and module exports.
- `src/backend.rs` - Graphics backend enum definitions (DX12, Vulkan, Metal).
- `src/state.rs` - 4-state resource management system (`00`, `01`, `10`, `11`).
- `src/resource.rs` - Resource descriptors and tracking (Buffers, Textures).
- `src/command.rs` - Command lists and GPU barrier recording.
- `src/device.rs` - Core device manager and hardware interface traits.
- `src/ffi.rs` - C-compatible Foreign Function Interface for external languages.

---

## Building the Library

To build the library for Rust and generate dynamic/static libraries for C/C++/C# (`.dll`, `.so`, `.dylib`), run:

```bash
cargo build --release

```

---

Running Tests

To execute the built-in test suite covering the 4-state logic and backend creation:

```bash
cargo test

```

---

## Usage in Rust

```rust
use quat_gfx::backend::GraphicsBackend;
use quat_gfx::device::{DeviceManager, GpuDevice};
use quat_gfx::resource::{ResourceDesc, ResourceType};
use quat_gfx::state::{ResourceState, ResourceStateFlags, QuatDigit, PackedQuatState};

fn main() {
    let mut device = DeviceManager::new(GraphicsBackend::Vulkan);
    let mut cmd_list = device.create_command_list();

    let desc = ResourceDesc {
        resource_type: ResourceType::Buffer,
        size_in_bytes: 2048,
        width: 0,
        height: 0,
        initial_state: ResourceState::State00,
    };

    let mut resource = device.create_resource(desc);

    // Tranzicija stanja kroz kvartarni sistem (State00 -> State01)
    device.transition_resource(&mut cmd_list, &mut resource, ResourceState::State01);

    // Rad sa spakovanim kvatskim stanjima (4 stanja u 1 u8 bajtu)
    let mut packed_state = PackedQuatState(0);
    packed_state.set_quat(0, QuatDigit::Q1);
    packed_state.set_quat(1, QuatDigit::Q3);
}
```

---

# Cross-Platform RHI (Rendering Hardware Interface)

## Building the Library

---

## For C

```c
#include <stdio.h>
#include <stdint.h>

// Definicije pokazivača na uređaj i komandnu listu
typedef void* DeviceManagerPtr;
typedef void* CommandListPtr;

// Eksterne funkcije koje dolaze iz Rust (cdylib) biblioteke (quat_gfx)
// backend_type: 0 = DirectX 12, 1 = Vulkan, 2 = Metal
DeviceManagerPtr rhi_create_device(uint8_t backend_type);
void rhi_destroy_device(DeviceManagerPtr device);

CommandListPtr rhi_create_command_list(DeviceManagerPtr device);
void rhi_destroy_command_list(CommandListPtr cmd_list);

// Funkcija za prelaz stanja (4 kvartarna stanja: 0, 1, 2, 3)
void rhi_command_list_barrier(CommandListPtr cmd_list, uint32_t resource_id, uint8_t from_state, uint8_t to_state);
void rhi_command_list_flush(CommandListPtr cmd_list);

int main() {
    printf("--- RHI Graphics Library Test from C ---\n");

    // Na Linuxu koristimo Vulkan (1), na Windowsu DX12 (0), na Mac-u Metal (2)
    DeviceManagerPtr device = rhi_create_device(1);
    CommandListPtr cmd_list = rhi_create_command_list(device);

    uint32_t resource_id = 77;

    // Testiranje 4 kvartarna stanja (0, 1, 2, 3)
    rhi_command_list_barrier(cmd_list, resource_id, 0, 1); // State 0 -> State 1
    rhi_command_list_barrier(cmd_list, resource_id, 1, 2); // State 1 -> State 2
    rhi_command_list_barrier(cmd_list, resource_id, 2, 3); // State 2 -> State 3

    // Izvršavanje komandi
    rhi_command_list_flush(cmd_list);

    // Čišćenje memorije
    rhi_destroy_command_list(cmd_list);
    rhi_destroy_device(device);

    return 0;
}
```

---

## For C++

```cpp
#include <iostream>
#include <cstdint>

extern "C" {
    void* rhi_create_device(uint8_t backend_type);
    void rhi_destroy_device(void* device);
    void* rhi_create_command_list(void* device);
    void rhi_destroy_command_list(void* cmd_list);
    void rhi_command_list_barrier(void* cmd_list, uint32_t resource_id, uint8_t from_state, uint8_t to_state);
    void rhi_command_list_flush(void* cmd_list);
}

class GraphicsDevice {
    void* m_device{nullptr};
public:
    explicit GraphicsDevice(uint8_t backend) : m_device(rhi_create_device(backend)) {}
    ~GraphicsDevice() { 
        if (m_device) { 
            rhi_destroy_device(m_device); 
            m_device = nullptr; 
        } 
    }

    // Onemogućavamo kopiranje radi RAII sigurnosti
    GraphicsDevice(const GraphicsDevice&) = delete;
    GraphicsDevice& operator=(const GraphicsDevice&) = delete;

    [[nodiscard]] void* get() const { return m_device; }
};

class CommandList {
    void* m_cmd_list{nullptr};
public:
    explicit CommandList(const GraphicsDevice& device) 
        : m_cmd_list(rhi_create_command_list(device.get())) {}

    ~CommandList() { 
        if (m_cmd_list) { 
            rhi_destroy_command_list(m_cmd_list); 
            m_cmd_list = nullptr; 
        } 
    }

    void Barrier(uint32_t res_id, uint8_t from, uint8_t to) {
        rhi_command_list_barrier(m_cmd_list, res_id, from, to);
    }

    void Flush() {
        rhi_command_list_flush(m_cmd_list);
    }
};

int main() {
    std::cout << "--- RHI Graphics Library Test from C++ ---\n";

    GraphicsDevice device(1); // 1 = Vulkan
    CommandList cmd_list(device);

    // Testiranje kvartarnih tranzicija stanja (0b00=0, 0b01=1, 0b10=2, 0b11=3)
    cmd_list.Barrier(99, 0b00, 0b11);
    cmd_list.Flush();

    return 0;
}
```

---

## For C#

```csharp
using System;
using System.Runtime.InteropServices;

class Program
{
    // Naziv tvoje kompajlirane Rust biblioteke (quat_gfx)
    const string DllName = "quat_gfx";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr rhi_create_device(byte backendType);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void rhi_destroy_device(IntPtr devicePtr);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr rhi_create_command_list(IntPtr devicePtr);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void rhi_destroy_command_list(IntPtr cmdListPtr);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void rhi_command_list_barrier(IntPtr cmdListPtr, uint resourceId, byte fromState, byte toState);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void rhi_command_list_flush(IntPtr cmdListPtr);

    static void Main(string[] args)
    {
        Console.WriteLine("--- RHI Graphics Library Test from C# ---");

        // Kreiranje uređaja (1 = Vulkan)
        IntPtr device = rhi_create_device(1);
        IntPtr cmdList = rhi_create_command_list(device);

        uint resourceId = 55;

        // Prelazak stanja preko sistema sa 4 kvartarna stanja (0, 1, 2, 3)
        rhi_command_list_barrier(cmdList, resourceId, 0b00, 0b10); // State 0 -> State 2
        rhi_command_list_barrier(cmdList, resourceId, 0b10, 0b11); // State 2 -> State 3

        // Izvršavanje
        rhi_command_list_flush(cmdList);

        // Oslobađanje memorije
        rhi_destroy_command_list(cmdList);
        rhi_destroy_device(device);
    }
}
```