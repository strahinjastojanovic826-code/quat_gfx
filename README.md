# Cross-Platform RHI (Rendering Hardware Interface)

A lightweight, high-performance graphics abstraction library written in Rust, supporting **DirectX 12** (Windows), **Vulkan** (Linux / Windows), and **Metal** (macOS). Designed with multi-language interoperability, supporting **Rust**, **C**, **C++**, and **C#**.

## Features

* **Multi-Backend Support:** Abstracted architecture for DirectX 12, Vulkan, WebGPU, OpenGL and Metal (macOS).
* **4-State Quaternary Resource State Machine:** Strict and precise resource state management using base-4 quaternary algebra (`00`, `01`, `10`, `11`).
* **High-Density State Packing:** Efficient packing of 4 quaternary states into a single 8-bit byte (`u8`) via `PackedQuatState`.
* **Bitflags 2.4 Integration:** Modern type-safe bitmask combinations for GPU resource states.
* **Multi-Language Interop:** Full **C-FFI** layer (`extern "C"`) allowing seamless integration with C, C++, and C#.
* **Modular Architecture:** Clean separation of concerns (`state`, `backend`, `device`, `resource`, `command`, `ffi`).
* **Built-in Testing & Safety:** Comprehensive unit tests and null-pointer safety checks for C-FFI bindings.
* **Resource State Transition Validation:** Strict state transition matrix to prevent invalid or unsafe resource state jumps.
* **Thread-Safe FFI Support:** Thread-safe context wrappers (`Arc<Mutex<...>>`) enabling concurrent multi-threaded usage across different languages.
* **Extended Quaternary Operations:** Advanced quaternary arithmetic, bitwise logic, and extended 64-bit multi-quat packing.
* **Zero-Dependency Native Benchmarking:** Built-in high-precision performance measurement suite (`std::time::Instant`) verifying sub-30ns latencies for core state and FFI operations.
* **High-Density State Packing:** Extremely cache-efficient packing of **32 quaternary states** into a single `u64` (`PackedQuatState64`) with custom iterators.

---

## Third-Party Licenses (`bitflags`)

This project utilizes the **`bitflags`** library for generating type-safe bitmask flags. 

* **Library Link:** [https://github.com/bitflags/bitflags](https://github.com/bitflags/bitflags)
* **Copyright / Owners:** Ashley Mannix, rust-lang-owner
* **License Terms:** `bitflags` is dual-licensed, meaning you may use it under the terms of either the **MIT License** or the **Apache License (Version 2.0)**.

---

### Option A: MIT License

Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

---

### Option B: Apache License (Version 2.0)

Copyright (c) 2014 The Rust Project Developers

Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

"License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.

"Licensor" shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.

"Legal Entity" shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, "control" means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.

"You" (or "Your") shall mean an individual or Legal Entity exercising permissions granted by this License.

"Source" form shall mean the preferred form for making modifications, including but not limited to software source code, documentation source, and configuration files.

"Object" form shall mean any form resulting from mechanical transformation or translation of a Source form, including but not limited to compiled object code, generated documentation, and conversions to other media types.

"Work" shall mean the work of authorship, whether in Source or Object form, made available under the License, as indicated by a copyright notice that is included in or attached to the work (an example is provided in the Appendix below).

"Derivative Works" shall mean any work, whether in Source or Object form, that is based on (or derived from) the Work and for which the editorial revisions, annotations, elaborations, or other modifications represent, as a whole, an original work of authorship. For the purposes of this License, Derivative Works shall not include works that remain separable from, or merely link (or bind by name) to the interfaces of, the Work and Derivative Works thereof.

"Contribution" shall mean any work of authorship, including the original version of the Work and any modifications or additions to that Work or Derivative Works thereof, that is intentionally submitted to Licensor for inclusion in the Work by the copyright owner or by an individual or Legal Entity authorized to submit on behalf of the copyright owner. For the purposes of this definition, "submitted" means any form of electronic, verbal, or written communication sent to the Licensor or its representatives, including but not limited to communication on electronic mailing lists, source code control systems, and issue tracking systems that are managed by, or on behalf of, the Licensor for the purpose of discussing and improving the Work, but excluding communication that is conspicuously marked or otherwise designated in writing by the copyright owner as "Not a Contribution."

"Contributor" shall mean Licensor and any individual or Legal Entity on behalf of whom a Contribution has been received by Licensor and subsequently incorporated within the Work.

2. Grant of Copyright License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare Derivative Works of, publicly display, publicly perform, sublicense, and distribute the Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable (except as stated in this section) patent license to make, have made, use, offer to sell, sell, import, and otherwise transfer the Work, where such license applies only to those patent claims licensable by such Contributor that are necessarily infringed by their Contribution(s) alone or by combination of their Contribution(s) with the Work to which such Contribution was submitted. If You institute patent litigation against any entity (including a cross-claim or counterclaim in a lawsuit) alleging that the Work or a Contribution incorporated within the Work constitutes direct or contributory patent infringement, then any patent licenses granted to You under this License for that Work shall terminate as of the date such litigation is filed.

4. Redistribution. You may reproduce and distribute copies of the Work or Derivative Works thereof in any medium, with or without modifications, and in Source or Object form, provided that You meet the following conditions:

(a) You must give any other recipients of the Work or Derivative Works a copy of this License; and
(b) You must cause any modified files to carry prominent notices stating that You changed the files; and
(c) You must retain, in the Source form of any Derivative Works that You distribute, all copyright, patent, trademark, and attribution notices from the Source form of the Work, excluding those notices that do not pertain to any part of the Derivative Works; and
(d) If the Work includes a "NOTICE" text file as part of its distribution, then any Derivative Works that You distribute must include a readable copy of the attribution notices contained within such NOTICE file, excluding those notices that do not pertain to any part of the Derivative Works, in at least one of the following places: within a NOTICE text file distributed as part of the Derivative Works; within the Source form or documentation, if provided along with the Derivative Works; or, within a display generated by the Derivative Works, if and wherever such third-party notices normally appear. The contents of the NOTICE file are for informational purposes only and do not modify the License. You may add Your own attribution notices within Derivative Works that You distribute, alongside or as an addendum to the NOTICE text from the Work, provided that such additional attribution notices cannot be construed as modifying the License.

You may add Your own copyright statement to Your modifications and may provide additional or different license terms and conditions for use, reproduction, or distribution of Your modifications, or for any such Derivative Works as a whole, provided Your use, reproduction, and distribution of the Work otherwise complies with the conditions stated in this License.

5. Submission of Contributions. Unless You explicitly stated otherwise, any Contribution intentionally submitted for inclusion in the Work by You to the Licensor shall be under the terms and conditions of this License, without any additional terms or conditions. Notwithstanding the above, nothing herein shall supersede or modify the terms of any separate license agreement you may have executed with Licensor regarding such Contributions.

6. Trademarks. This License does not grant permission to use the trade names, trademarks, service marks, or product names of the Licensor, except as required for reasonable and customary use in describing the origin of the Work and reproducing the content of the NOTICE file.

7. Disclaimer of Warranty. Unless required by applicable law or agreed to in writing, Licensor provides the Work (and each Contributor provides its Contributions) on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied, including, without limitation, any warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining the appropriateness of using or redistributing the Work and assume any risks associated with Your exercise of permissions under this License.

8. Limitation of Liability. In no event and under no legal theory, whether in tort (including negligence), contract, or otherwise, unless required by applicable law (such as deliberate and grossly negligent acts) or agreed to in writing, shall any Contributor be liable to You for damages, including any direct, indirect, special, incidental, or consequential damages of any character arising as a result of this License or out of the use or inability to use the Work (including but not limited to damages for loss of goodwill, work stoppage, computer failure or malfunction, or any and all other commercial and personal damages), even if such Contributor has been advised of the possibility of such damages.

9. Accepting Warranty or Additional Liability. While redistributing the Work or Derivative Works thereof, You may choose to charge a fee for, and offer additional, support, warranty, indemnity, or other liability obligations and/or rights consistent with this License. However, in accepting such obligations, You may act only on Your own behalf and on Your sole responsibility, not on behalf of any other Contributor, and only if You agree to indemnify, defend, and hold each Contributor harmless for any liability incurred by, or claims asserted against, such Contributor by reason of your accepting any such warranty or additional liability.

END OF TERMS AND CONDITIONS

---

## Benchmark

| Test / Operation Name | Iteration Count | Total Time | Average per Call / Iteration |
| :--- | :--- | :--- | :--- |
| `test_backends_creation` | 50,000 | 2.8829 ms | -[cite: 29] |
| `test_bitflags_2_4_integration` | 100,000 | 3.4496 ms | -[cite: 29] |
| `test_packed_quat_state_64` | 100,000 | 3.3901 ms | -[cite: 29] |
| `test_quaternary_masking_overflow` | 100,000 | 3.4582 ms | -[cite: 29] |
| `test_packed_quat_state_packing` | 100,000 | 3.9497 ms | -[cite: 29] |
| `test_ffi_null_pointer_handling` | 100,000 | 5.0141 ms | -[cite: 29] |
| `test_invalid_resource_transition` | 100,000 | 5.6855 ms | -[cite: 29] |
| `test_resource_states_4_modes` | 100,000 | 5.4967 ms | -[cite: 29] |
| `test_resource_transition_pipeline` | 50,000 | 4.5308 ms | -[cite: 29] |
| `test_ffi_safe_lifecycle` | 50,000 | 11.3419 ms | -[cite: 29] |
| `FFI State Set` | 1,000,000 | 15.5431 ms | 15 ns / call[cite: 29] |
| `FFI Validation` | 1,000,000 | 29.6438 ms | 29 ns / call[cite: 29] |
| `Matrix Validation` | 10,000,000 | 81.1385 ms | 8 ns / call[cite: 29] |
| `PackedQuatState64` (all 32 quats) | 1,000,000 | 299.6717 ms | 299 ns / loop[cite: 29] |

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_gfx` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

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
use quat_gfx::state::{ResourceState, ResourceStateFlags, QuatDigit, PackedQuatState64};

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
    device.safe_transition_resource(&mut cmd_list, &mut resource, ResourceState::State01).unwrap();

    // Rad sa naprednim spakovanim kvatskim stanjima (32 stanja u 1 u64 registru)
    let mut packed_state = PackedQuatState64::new();
    packed_state.set_quat(0, QuatDigit::Q1);
    packed_state.set_quat(1, QuatDigit::Q3);

    // Primer korišćenja novih WebGPU ili OpenGL bekenda:
    let webgpu_device = DeviceManager::new(GraphicsBackend::WebGPU);

    // Primer korišćenja bitflags integracije za kombinovana stanja:
    let mut flags = ResourceStateFlags::NONE;
    flags.insert(ResourceStateFlags::SHADER_READ | ResourceStateFlags::COPY_DEST);
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
// backend_type: 0 = DirectX 12, 1 = Vulkan, 2 = Metal, 3 = WebGPU, 4 = OpenGL
DeviceManagerPtr rhi_create_device(uint8_t backend_type);
void rhi_destroy_device(DeviceManagerPtr device);

CommandListPtr rhi_create_command_list(DeviceManagerPtr device);
void rhi_destroy_command_list(CommandListPtr cmd_list);

// Funkcija za prelaz stanja (4 kvartarna stanja: 0, 1, 2, 3)
void rhi_command_list_barrier(CommandListPtr cmd_list, uint32_t resource_id, uint8_t from_state, uint8_t to_state);
void rhi_command_list_flush(CommandListPtr cmd_list);

int main() {
    printf("--- RHI Graphics Library Test from C ---\n");

    // Na Linuxu koristimo Vulkan (1), na Windowsu DX12 (0), na Mac-u Metal (2), ili WebGPU (3) / OpenGL (4)
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
    void* rhi_destroy_command_list(void* cmd_list);
    void rhi_command_list_barrier(void* cmd_list, uint32_t resource_id, uint8_t from_state, uint8_t to_state);
    void rhi_command_list_flush(void* cmd_list);
}

class GraphicsDevice {
    void* m_device{nullptr};
public:
    // backend: 0 = DX12, 1 = Vulkan, 2 = Metal, 3 = WebGPU, 4 = OpenGL
    explicit GraphicsDevice(uint8_t backend) : m_device(rhi_create_device(backend)) {}
    ~GraphicsDevice() {
        if (m_device) {
            rhi_destroy_device(m_device);
            m_device = nullptr;
        }
    }

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

    // 0 = DX12, 1 = Vulkan, 2 = Metal, 3 = WebGPU, 4 = OpenGL
    GraphicsDevice device(1); 
    CommandList cmd_list(device);

    // Testiranje kvartarnih tranzicija stanja (0b00=0, 0b01=1, 0b10=2, 0b11=3)
    cmd_list.Barrier(99, 0b00, 0b01);
    cmd_list.Barrier(99, 0b01, 0b10);
    cmd_list.Barrier(99, 0b10, 0b11);
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
    const string DllName = "quat_gfx";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdec1)]
    public static extern IntPtr rhi_create_device(byte backendType);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdec1)]
    public static extern void rhi_destroy_device(IntPtr devicePtr);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdec1)]
    public static extern IntPtr rhi_create_command_list(IntPtr devicePtr);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdec1)]
    public static extern void rhi_destroy_command_list(IntPtr cmdListPtr);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdec1)]
    public static extern void rhi_command_list_barrier(IntPtr cmdListPtr, uint resourceId, byte fromState, byte toState);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdec1)]
    public static extern void rhi_command_list_flush(IntPtr cmdListPtr);

    static void Main(string[] args)
    {
        Console.WriteLine("--- RHI Graphics Library Test from C# ---");

        // Kreiranje uređaja (0 = DX12, 1 = Vulkan, 2 = Metal, 3 = WebGPU, 4 = OpenGL)
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