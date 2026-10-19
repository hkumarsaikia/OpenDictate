# OpenDictate Third-Party Software Licenses & Notices

**Application:** OpenDictate v2.0.3  
**Application License:** MIT License (Copyright (c) 2026 H. K. Saikia)  
**Commit:** `7a73366e2da882e05db3a05b189a05b6d08bc477`  

This document contains the complete third-party intellectual property, licensing, and attribution notices for all direct and transitive dependencies, bundled assets, native libraries, and speech models utilized in the OpenDictate distribution.

---

## 1. Special Legal Disclosures & Compliance Notices

### 1.1 Mozilla Public License 2.0 (MPL-2.0) — `option-ext` v0.2.0
OpenDictate incorporates the `option-ext` crate (version 0.2.0, authored by Simon Ochsenreither), which is licensed under the **Mozilla Public License, Version 2.0 (MPL-2.0)**.

- **Nature of License:** MPL-2.0 is a file-level weak copyleft license. Under Section 3.3 (Larger Works), OpenDictate is distributed as a Larger Work under the MIT License while maintaining all required notices for MPL-covered files.
- **Source Code Availability:** The complete upstream source code of `option-ext` v0.2.0 is freely available at:
  `https://github.com/soc/option-ext.git` (or on crates.io at `https://crates.io/crates/option-ext/0.2.0`).
- **Modifications:** OpenDictate incorporates `option-ext` unmodified.
- **License Text:** The full text of the MPL-2.0 is included in [Section 4.3](#43-mozilla-public-license-version-20-mpl-20) below and in `licenses/LICENSE-MPL-2.0.txt`.

### 1.2 Tri-Licensed Dependencies — `r-efi` v6.0.0
The crate `r-efi` (version 6.0.0) is tri-licensed under `MIT OR Apache-2.0 OR LGPL-2.1-or-later`.

- **Elected License:** In accordance with the disjunctive licensing terms, OpenDictate formally elects the **MIT License** for `r-efi`.
- **Notice & Attribution:** Copyright (C) 2017-2023 Red Hat, Inc., Microsoft Corporation, and David Rheinsberg. See [Section 3.1](#31-active-linux-runtime-dependencies) for complete author notices.

### 1.3 Native Audio Library — `alsa-lib` 1.2.12 (LGPL-2.1-or-later)
OpenDictate interfaces with the Advanced Linux Sound Architecture (`alsa-lib` version 1.2.12), licensed under the **GNU Lesser General Public License, Version 2.1 or later (LGPL-2.1+)**.

- **Linkage Compliance:** OpenDictate dynamically links against `libasound.so.2` via shared object linkage. In Flatpak, `libasound.so.2.0.0` is provided as a shared library in `/app/lib/`; in Snap, it is dynamically loaded from `$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/`.
- **User Modification & Reverse Engineering:** Dynamic linking satisfies Section 6(a) of the LGPL-2.1, allowing users to modify or replace `libasound.so.2` independently.
- **License Text:** The full LGPL-2.1 text is installed in the package at `/app/share/licenses/io.github.hkumarsaikia.OpenDictate/alsa-lib/COPYING` and reproduced in [Section 4.4](#44-gnu-lesser-general-public-license-version-21-lgpl-21).

### 1.4 Whisper C/C++ Engine — `whisper.cpp` (MIT License)
The embedded speech inference engine `whisper.cpp` (bundled via `whisper-rs-sys`) is authored by Georgi Gerganov and licensed under the **MIT License** (Copyright (c) 2023 Georgi Gerganov).

### 1.5 OpenAI Whisper Model Weights (`ggml-tiny.en.bin`, etc.)
The default speech recognition models supported by OpenDictate are based on OpenAI's Whisper neural network architecture.

- **Model Source:** Trained and published by OpenAI (Radford et al., 2022) under the **MIT License** (`Copyright (c) 2022 OpenAI`).
- **Conversion Source:** Converted to GGML binary format by Georgi Gerganov under the **MIT License** (`Copyright (c) 2023 Georgi Gerganov`).
- **Distribution Architecture:** Model weights are **not bundled** into the application binary or release packages. They are fetched on demand by the user over HTTPS directly into `$XDG_DATA_HOME/opendictate/models/`.

---

## 2. License Breakdown Summary (447 Cargo Crates)

| Declared SPDX License | Package Count | License Category | Scope |
| :--- | :---: | :--- | :--- |
| **MIT OR Apache-2.0** | 216 | Dual / Multi-License | 133 active on Linux |
| **MIT** | 91 | Permissive | 78 active on Linux |
| **Apache-2.0 OR MIT** | 33 | Dual / Multi-License | 29 active on Linux |
| **MIT/Apache-2.0** | 20 | Dual / Multi-License | 13 active on Linux |
| **Unicode-3.0** | 18 | Permissive | 18 active on Linux |
| **Apache-2.0** | 9 | Permissive | 7 active on Linux |
| **Apache-2.0/MIT** | 8 | Dual / Multi-License | 7 active on Linux |
| **Unlicense OR MIT** | 7 | Public Domain / Zero-Clause | 5 active on Linux |
| **Zlib OR Apache-2.0 OR MIT** | 6 | Dual / Multi-License | 1 active on Linux |
| **Unlicense** | 3 | Public Domain / Zero-Clause | 3 active on Linux |
| **ISC** | 3 | Permissive | 1 active on Linux |
| **Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT** | 3 | Dual / Multi-License | 2 active on Linux |
| **BSD-3-Clause** | 2 | Permissive | 1 active on Linux |
| **BSL-1.0** | 2 | Permissive | 0 active on Linux |
| **Apache-2.0 OR ISC OR MIT** | 2 | Dual / Multi-License | 0 active on Linux |
| **MIT OR Zlib OR Apache-2.0** | 2 | Dual / Multi-License | 2 active on Linux |
| **BSD-3-Clause OR Apache-2.0** | 2 | Dual / Multi-License | 2 active on Linux |
| **Zlib** | 2 | Permissive | 1 active on Linux |
| **BSD-3-Clause OR MIT OR Apache-2.0** | 2 | Dual / Multi-License | 0 active on Linux |
| **Unlicense/MIT** | 2 | Public Domain / Zero-Clause | 0 active on Linux |
| **BSD-2-Clause OR Apache-2.0 OR MIT** | 2 | Dual / Multi-License | 1 active on Linux |
| **MIT OR Apache-2.0 OR Zlib** | 2 | Dual / Multi-License | 0 active on Linux |
| **0BSD OR MIT OR Apache-2.0** | 1 | Public Domain / Zero-Clause | 1 active on Linux |
| **(Apache-2.0 OR MIT) AND BSD-3-Clause** | 1 | Dual / Multi-License | 1 active on Linux |
| **Apache-2.0 / MIT** | 1 | Dual / Multi-License | 1 active on Linux |
| **BSD-2-Clause OR MIT OR Apache-2.0** | 1 | Dual / Multi-License | 0 active on Linux |
| **MPL-2.0** | 1 | Weak Copyleft (File-Level) | 1 active on Linux |
| **MIT OR Apache-2.0 OR LGPL-2.1-or-later** | 1 | Strong Copyleft | 0 active on Linux |
| **Apache-2.0 AND ISC** | 1 | Permissive | 0 active on Linux |
| **Apache-2.0 OR BSL-1.0** | 1 | Dual / Multi-License | 1 active on Linux |
| **Apache-2.0 WITH LLVM-exception** | 1 | Permissive | 1 active on Linux |
| **(MIT OR Apache-2.0) AND Unicode-3.0** | 1 | Dual / Multi-License | 1 active on Linux |

---

## 3. Complete Rust Dependency Inventory

### 3.1 Active Linux Runtime Dependencies (311 Crates)

The following crates are directly compiled or linked into the OpenDictate Linux binary for `x86_64-unknown-linux-gnu`:

#### `adler2` v2.0.1
- **SPDX License:** `0BSD OR MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/oyvindln/adler2
- **Authors / Contributors:** Jonas Schievink <jonasschievink@gmail.com>, oyvindln <oyvindln@users.noreply.github.com>
- **Description:** A simple clean-room implementation of the Adler-32 checksum
- **Included Notice / License Content:**
  - *File: `LICENSE-0BSD`*
    ```text
    Copyright (C) Jonas Schievink <jonasschievink@gmail.com>

Permission to use, copy, modify, and/or distribute this software for
any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT
OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/LICENSE-2.0

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `ahash` v0.8.12
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/tkaitchuck/ahash
- **Authors / Contributors:** Tom Kaitchuck <Tom.Kaitchuck@gmail.com>
- **Description:** A non-cryptographic hash function using AES-NI for high performance
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018 Tom Kaitchuck

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `aho-corasick` v1.1.5
- **SPDX License:** `Unlicense OR MIT`
- **Repository / Source:** https://github.com/BurntSushi/aho-corasick
- **Authors / Contributors:** Andrew Gallant <jamslam@gmail.com>
- **Description:** Fast multiple substring searching.
- **Included Notice / License Content:**
  - *File: `COPYING`*
    ```text
    This project is dual-licensed under the Unlicense and MIT licenses.

You may use this code under the terms of either license.
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```
  - *File: `UNLICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `alsa` v0.9.1
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/diwic/alsa-rs
- **Authors / Contributors:** David Henningsson <coding@diwic.se>
- **Description:** Thin but safe wrappers for ALSA (Linux sound API)
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Copyright (c) 2015-2021 David Henningsson, and other contributors.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `alsa-sys` v0.3.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/diwic/alsa-sys
- **Authors / Contributors:** Pierre Krieger <pierre.krieger1708@gmail.com>, David Henningsson <coding@diwic.se>
- **Description:** FFI bindings for the ALSA project (Advanced Linux Sound Architecture)
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) 2018 diwic

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `ansi_term` v0.12.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/ogham/rust-ansi-term
- **Authors / Contributors:** ogham@bsago.me, Ryan Scheel (Havvy) <ryan.havvy@gmail.com>, Josh Triplett <josh@joshtriplett.org>
- **Description:** Library for ANSI terminal colours and styles (bold, underline)
- **Included Notice / License Content:**
  - *File: `LICENCE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2014 Benjamin Sago

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `anstream` v1.0.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/anstyle.git
- **Description:** IO stream adapters for writing colored text that will gracefully degrade according to your terminal's capabilities.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `anstyle` v1.0.14
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/anstyle.git
- **Description:** ANSI text styling
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `anstyle-parse` v1.0.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/anstyle.git
- **Description:** Parse ANSI Style Escapes
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `anstyle-query` v1.1.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/anstyle.git
- **Description:** Look up colored console capabilities
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `arboard` v3.6.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/1Password/arboard
- **Description:** Image and text handling for the OS clipboard.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE.txt`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT.txt`*
    ```text
    MIT License

Copyright (c) 2022 The Arboard contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `ashpd` v0.9.3
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/bilelmoussaoui/ashpd
- **Authors / Contributors:** Bilal Elmoussaoui <bil.elmoussaoui@gmail.com>
- **Description:** XDG portals wrapper in Rust using zbus
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) 2020 Bilal Elmoussaoui

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `async-broadcast` v0.7.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/smol-rs/async-broadcast
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, Yoshua Wuyts <yoshuawuyts@gmail.com>, Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** Async broadcast channels
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 175 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2020 Yoshua Wuyts

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `async-channel` v2.5.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-channel
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** Async multi-producer multi-consumer channel
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-executor` v1.14.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-executor
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, John Nunley <dev@notgull.net>
- **Description:** Async executor
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-fs` v2.2.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-fs
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** Async filesystem primitives
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-io` v2.6.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-io
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** Async I/O and timers
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-lock` v3.4.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-lock
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** Async synchronization primitives
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-net` v2.0.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-net
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** Async networking primitives for TCP/UDP/Unix communication
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-task` v4.7.1
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/async-task
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** Task abstraction for building executors
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `async-trait` v0.1.92
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/async-trait
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Type erasure for async trait methods
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `atomic-waker` v1.1.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/atomic-waker
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, Contributors to futures-rs
- **Description:** A synchronization primitive for task wakeup
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```
  - *File: `LICENSE-THIRD-PARTY`*
    ```text
    ===============================================================================

Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
    [... truncated 30 lines ...]
    ```

#### `atty` v0.2.14
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/softprops/atty
- **Authors / Contributors:** softprops <d.tangren@gmail.com>
- **Description:** A simple interface for querying atty
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2015-2019 Doug Tangren

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    [... truncated 5 lines ...]
    ```

#### `autocfg` v1.5.1
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/cuviper/autocfg
- **Authors / Contributors:** Josh Stone <cuviper@gmail.com>
- **Description:** Automatic cfg for Rust compiler features
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018 Josh Stone

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `base64` v0.22.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/marshallpierce/rust-base64
- **Authors / Contributors:** Marshall Pierce <marshall@mpierce.org>
- **Description:** encodes and decodes base64 as bytes or utf8
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Alice Maz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `base64` v0.23.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/marshallpierce/rust-base64
- **Authors / Contributors:** Marshall Pierce <marshall@mpierce.org>
- **Description:** encodes and decodes base64 as bytes or utf8
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2025 Alice Maz, Marshall Pierce

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `bindgen` v0.72.1
- **SPDX License:** `BSD-3-Clause`
- **Repository / Source:** https://github.com/rust-lang/rust-bindgen
- **Authors / Contributors:** Jyun-Yan You <jyyou.tw@gmail.com>, Emilio Cobos Álvarez <emilio@crisal.io>, Nick Fitzgerald <fitzgen@gmail.com>, The Servo project developers
- **Description:** Automatically generates Rust FFI bindings to C and C++ libraries.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    BSD 3-Clause License

Copyright (c) 2013, Jyun-Yan You
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

    [... truncated 14 lines ...]
    ```

#### `bitflags` v1.3.2
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/bitflags/bitflags
- **Authors / Contributors:** The Rust Project Developers
- **Description:** A macro to generate structures which behave like bitflags.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `bitflags` v2.13.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/bitflags/bitflags
- **Authors / Contributors:** The Rust Project Developers
- **Description:** A macro to generate structures which behave like bitflags.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `block-buffer` v0.10.4
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/RustCrypto/utils
- **Authors / Contributors:** RustCrypto Developers
- **Description:** Buffer type for block processing of data
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018-2019 The RustCrypto Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `blocking` v1.7.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/blocking
- **Description:** A thread pool for isolating blocking I/O in async programs
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `bytemuck` v1.25.2
- **SPDX License:** `Zlib OR Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/Lokathor/bytemuck
- **Authors / Contributors:** Lokathor <zefria@gmail.com>
- **Description:** A crate for mucking around with piles of bytes.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

    1. Definitions.

        "License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.

        "Licensor" shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.

        "Legal Entity" shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, "control" means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.

        "You" (or "Your") shall mean an individual or Legal Entity exercising permissions granted by this License.
    [... truncated 46 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Copyright (c) 2019 Daniel "Lokathor" Gee.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
    ```
  - *File: `LICENSE-ZLIB`*
    ```text
    Copyright (c) 2019 Daniel "Lokathor" Gee.

This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

3. This notice may not be removed or altered from any source distribution.
    ```

#### `byteorder-lite` v0.1.0
- **SPDX License:** `Unlicense OR MIT`
- **Repository / Source:** https://github.com/image-rs/byteorder-lite
- **Description:** Library for reading/writing numbers in big-endian and little-endian.
- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```
  - *File: `UNLICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `bytes` v1.12.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/bytes
- **Authors / Contributors:** Carl Lerche <me@carllerche.com>, Sean McArthur <sean@seanmonstar.com>
- **Description:** Types and traits for working with bytes
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2018 Carl Lerche

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `cairo-rs` v0.20.12
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the Cairo library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `cairo-sys-rs` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libcairo
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `cc` v1.5.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/cc-rs
- **Description:** A build-time dependency for Cargo build scripts to assist in invoking the native
C compiler to compile native C code into a static archive to be linked into Rust
code.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `cexpr` v0.6.0
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/jethrogb/rust-cexpr
- **Authors / Contributors:** Jethro Beekman <jethro@jbeekman.nl>
- **Description:** A C expression parser and evaluator
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    (C) Copyright 2016 Jethro G. Beekman

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `cfg-expr` v0.20.9
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/EmbarkStudios/cfg-expr
- **Authors / Contributors:** Embark <opensource@embark-studios.com>, Jake Shadle <jake.shadle@embark-studios.com>
- **Description:** A parser and evaluator for Rust `cfg()` expressions.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2019 Embark Studios

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `cfg-if` v1.0.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/cfg-if
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>
- **Description:** A macro to ergonomically define an item depending on a large number of #[cfg]
parameters. Structured like an if-else chain, the first matching branch is the
item that gets emitted.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `cfg_aliases` v0.2.2
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/katharostech/cfg_aliases
- **Authors / Contributors:** Zicklag <zicklag@katharostech.com>
- **Description:** A tiny utility to help save you a lot of effort with long winded `#[cfg()]` checks.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) 2020 Katharos Technology

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
    ```
  - *File: `NOTICES.md`*
    ```text
    # 3rd Party Notices

The `cfg_aliases!` macro uses a lot of the code from [`tectonic_cfg_support::target_cfg!`] macro which is under the following license:

[`tectonic_cfg_support::target_cfg!`]: https://github.com/tectonic-typesetting/tectonic/blob/f2439b936470ad27bdf92882064bc4702ee01899/cfg_support/src/lib.rs#L166

    tectonic_cfg_support is licensed under the MIT License.

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the “Software”), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    [... truncated 11 lines ...]
    ```

#### `clang-sys` v1.9.1
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://github.com/KyleMayes/clang-sys
- **Authors / Contributors:** Kyle Mayes <kyle@mayeses.com>
- **Description:** Rust bindings for libclang.
- **Included Notice / License Content:**
  - *File: `LICENSE.txt`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```

#### `clap` v2.34.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/clap-rs/clap
- **Authors / Contributors:** Kevin K. <kbknapp@gmail.com>
- **Description:** A simple to use, efficient, and full-featured Command Line Argument Parser

- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015-2016 Kevin B. Knapp

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `cmake` v0.1.58
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/cmake-rs
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>
- **Description:** A build dependency for running `cmake` to build a native library

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `colorchoice` v1.0.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/anstyle.git
- **Description:** Global override of color control
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `concurrent-queue` v2.5.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/concurrent-queue
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, Taiki Endo <te316e89@gmail.com>, John Nunley <dev@notgull.net>
- **Description:** Concurrent multi-producer multi-consumer queue
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `core_detect` v1.0.0
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/thomcc/core_detect
- **Authors / Contributors:** Thom Chiovoloni <chiovolonit@gmail.com>
- **Description:** A `no_std` version of the `std::is_x86_feature_detected!` macro.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017-2020 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `cpal` v0.15.3
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://github.com/rustaudio/cpal
- **Description:** Low-level cross-platform audio I/O library in pure Rust.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```

#### `cpufeatures` v0.2.17
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/RustCrypto/utils
- **Authors / Contributors:** RustCrypto Developers
- **Description:** Lightweight runtime CPU feature detection for aarch64, loongarch64, and x86/x86_64 targets, 
with no_std support and support for mobile targets including Android and iOS

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2020-2025 The RustCrypto Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `crc32fast` v1.5.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/srijs/rust-crc32fast
- **Authors / Contributors:** Sam Rijs <srijs@airpost.net>, Alex Crichton <alex@alexcrichton.com>
- **Description:** Fast, SIMD-accelerated CRC32 (IEEE) checksum computation
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Copyright (c) 2018 Sam Rijs, Alex Crichton and contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `crossbeam-utils` v0.8.23
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/crossbeam-rs/crossbeam
- **Description:** Utilities for concurrent programming
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2019 The Crossbeam Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
    [... truncated 12 lines ...]
    ```

#### `crypto-common` v0.1.7
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/RustCrypto/traits
- **Authors / Contributors:** RustCrypto Developers
- **Description:** Common cryptographic traits
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2021 RustCrypto Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `dasp_sample` v0.11.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rustaudio/sample.git
- **Authors / Contributors:** mitchmindtree <mitchell.nordine@gmail.com>
- **Description:** An abstraction for audio PCM DSP samples, along with useful conversions and operations.

#### `dbus` v0.9.12
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/diwic/dbus-rs
- **Authors / Contributors:** David Henningsson <diwic@ubuntu.com>
- **Description:** Bindings to D-Bus, which is a bus commonly used on Linux for inter-process communication.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014-2018 David Henningsson <diwic@ubuntu.com> and other contributors

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
    [... truncated 4 lines ...]
    ```

#### `dbus-codegen` v0.9.1
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/diwic/dbus-rs
- **Authors / Contributors:** David Henningsson <diwic@ubuntu.com>
- **Description:** Binary crate to generate Rust code from XML introspection data
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014-2018 David Henningsson <diwic@ubuntu.com> and other contributors

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
    [... truncated 4 lines ...]
    ```

#### `dbus-tree` v0.9.2
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/diwic/dbus-rs
- **Authors / Contributors:** David Henningsson <diwic@ubuntu.com>
- **Description:** Framework for writing D-Bus method handlers (legacy)
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014-2018 David Henningsson <diwic@ubuntu.com> and other contributors

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
    [... truncated 4 lines ...]
    ```

#### `digest` v0.10.7
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/RustCrypto/traits
- **Authors / Contributors:** RustCrypto Developers
- **Description:** Traits for cryptographic hash functions and message authentication codes
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 Artyom Pavlov

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `directories` v5.0.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/soc/directories-rs
- **Authors / Contributors:** Simon Ochsenreither <simon@ochsenreither.de>
- **Description:** A tiny mid-level library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows and macOS by leveraging the mechanisms defined by the XDG base/user directory specifications on Linux, the Known Folder API on Windows, and the Standard Directory guidelines on macOS.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 159 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018 directories-rs contributors

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
    [... truncated 4 lines ...]
    ```

#### `dirs-sys` v0.4.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dirs-dev/dirs-sys-rs
- **Authors / Contributors:** Simon Ochsenreither <simon@ochsenreither.de>
- **Description:** System-level helper functions for the dirs and directories crates.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 159 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018-2019 dirs-rs contributors

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
    [... truncated 4 lines ...]
    ```

#### `displaydoc` v0.2.7
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/yaahc/displaydoc
- **Authors / Contributors:** Jane Lusby <jlusby@yaah.dev>
- **Description:** A derive macro for implementing the display Trait via a doc comment and string interpolation

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `either` v1.18.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rayon-rs/either
- **Description:** The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `encoding_rs` v0.8.42
- **SPDX License:** `(Apache-2.0 OR MIT) AND BSD-3-Clause`
- **Repository / Source:** https://github.com/hsivonen/encoding_rs
- **Authors / Contributors:** Henri Sivonen <hsivonen@hsivonen.fi>
- **Description:** A Gecko-oriented implementation of the Encoding Standard
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    encoding_rs is copyright Mozilla Foundation.

Licensed under the Apache License, Version 2.0
<LICENSE-APACHE or
https://www.apache.org/licenses/LICENSE-2.0> or the MIT
license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
at your option. All files in the project carrying such
notice may not be copied, modified, or distributed except
according to those terms.

This crate includes data derived from the data files supplied
with the WHATWG Encoding Standard, which, when incorporated into
source code, are licensed under the BSD 3-Clause License
<LICENSE-WHATWG>.

    [... truncated 2 lines ...]
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright Mozilla Foundation

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```
  - *File: `LICENSE-WHATWG`*
    ```text
    Copyright © WHATWG (Apple, Google, Mozilla, Microsoft).

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.
    [... truncated 11 lines ...]
    ```

#### `endi` v1.1.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/zeenix/endi
- **Authors / Contributors:** Zeeshan Ali Khan <zeenix@gmail.com>
- **Description:** A simple endian-handling library
- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `enumflags2` v0.7.12
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/meithecatte/enumflags2
- **Authors / Contributors:** maik klein <maikklein@googlemail.com>, Maja Kądziołka <maya@compilercrim.es>
- **Description:** Enum-based bit flags
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License

Version 2.0, January 2004

http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

"License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.

"Licensor" shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.

"Legal Entity" shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, "control" means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.
    [... truncated 53 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017-2023 Maik Klein, Maja Kądziołka

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `enumflags2_derive` v0.7.12
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/meithecatte/enumflags2
- **Authors / Contributors:** maik klein <maikklein@googlemail.com>, Maja Kądziołka <maya@compilercrim.es>
- **Description:** Do not use directly, use the reexport in the `enumflags2` crate. This allows for better compatibility across versions.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License

Version 2.0, January 2004

http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

"License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.

"Licensor" shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.

"Legal Entity" shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, "control" means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.
    [... truncated 53 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 Maik Klein

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `env_filter` v2.0.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/env_logger
- **Description:** Filter log events using environment variables

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `env_logger` v0.11.11
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-cli/env_logger
- **Description:** A logging implementation for `log` which is configured via an environment
variable.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `equivalent` v1.0.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/indexmap-rs/equivalent
- **Description:** Traits for key comparison in maps.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016--2023

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `errno` v0.3.14
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/lambda-fairy/rust-errno
- **Authors / Contributors:** Chris Wong <lambda.fairy@gmail.com>, Dan Gohman <dev@sunfishcode.online>
- **Description:** Cross-platform interface to the `errno` variable.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Chris Wong

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `event-listener` v5.4.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/event-listener
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, John Nunley <dev@notgull.net>
- **Description:** Notify async tasks or threads
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `event-listener-strategy` v0.5.4
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/event-listener-strategy
- **Authors / Contributors:** John Nunley <dev@notgull.net>
- **Description:** Block or poll on event_listener easily
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `eventsource-stream` v0.2.3
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/jpopesculian/eventsource-stream
- **Authors / Contributors:** Julian Popescu <jpopesculian@gmail.com>
- **Description:** A basic building block for building an Eventsource from a Stream of bytes

#### `fallible-iterator` v0.3.0
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/sfackler/rust-fallible-iterator
- **Authors / Contributors:** Steven Fackler <sfackler@gmail.com>
- **Description:** Fallible iterator traits
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 The rust-openssl-verify Developers

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
    [... truncated 4 lines ...]
    ```

#### `fallible-streaming-iterator` v0.1.9
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/sfackler/fallible-streaming-iterator
- **Authors / Contributors:** Steven Fackler <sfackler@gmail.com>
- **Description:** Fallible streaming iteration
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 The fallible-streaming-iterator Developers

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
    [... truncated 4 lines ...]
    ```

#### `fastrand` v2.5.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/fastrand
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>
- **Description:** A simple and fast random number generator
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `fdeflate` v0.3.7
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/image-rs/fdeflate
- **Authors / Contributors:** The image-rs Developers
- **Description:** Fast specialized deflate implementation
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `field-offset` v0.3.6
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/Diggsey/rust-field-offset
- **Authors / Contributors:** Diggory Blake <diggsey@googlemail.com>
- **Description:** Safe pointer-to-member implementation
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Copyright (c) 2016-2021 Diggory Blake, and other contributors.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `find-msvc-tools` v0.1.14
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/cc-rs
- **Description:** Find windows-specific tools, read MSVC versions from the registry and from COM interfaces
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `flate2` v1.1.10
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/flate2-rs
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>, Josh Triplett <josh@joshtriplett.org>
- **Description:** DEFLATE compression and decompression exposed as Read/BufRead/Write streams.
Supports miniz_oxide and multiple zlib implementations. Supports zlib, gzip,
and raw deflate streams.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014-2026 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `flume` v0.11.1
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/zesterer/flume
- **Authors / Contributors:** Joshua Barretto <joshua.s.barretto@gmail.com>
- **Description:** A blazingly fast multi-producer channel
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `fnv` v1.0.7
- **SPDX License:** `Apache-2.0 / MIT`
- **Repository / Source:** https://github.com/servo/rust-fnv
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>
- **Description:** Fowler–Noll–Vo hash function
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `foreign-types` v0.3.2
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/sfackler/foreign-types
- **Authors / Contributors:** Steven Fackler <sfackler@gmail.com>
- **Description:** A framework for Rust wrappers over C APIs
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 The foreign-types Developers

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
    [... truncated 4 lines ...]
    ```

#### `foreign-types-shared` v0.1.1
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/sfackler/foreign-types
- **Authors / Contributors:** Steven Fackler <sfackler@gmail.com>
- **Description:** An internal crate used by foreign-types
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 The foreign-types Developers

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
    [... truncated 4 lines ...]
    ```

#### `form_urlencoded` v1.2.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/servo/rust-url
- **Authors / Contributors:** The rust-url developers
- **Description:** Parser and serializer for the application/x-www-form-urlencoded syntax, as used by HTML forms.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2013-2016 The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `fragile` v2.1.0
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://github.com/mitsuhiko/fragile
- **Authors / Contributors:** Armin Ronacher <armin.ronacher@active-4.com>
- **Description:** Provides wrapper types for sending non-send values to other threads.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```

#### `fs_extra` v1.3.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/webdesus/fs_extra
- **Authors / Contributors:** Denis Kurilenko <webdesus@gmail.com>
- **Description:** Expanding std::fs and std::io. Recursively copy folders with information about process and much more.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) 2017 Denis Kurilenko

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `futures` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** An implementation of futures and streams featuring zero allocations,
composability, and iterator-like interfaces.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-channel` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** Channels for asynchronous communication using futures-rs.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-core` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** The core traits and types in for the `futures` library.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-executor` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** Executors for asynchronous tasks based on the futures-rs library.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-io` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** The `AsyncRead`, `AsyncWrite`, `AsyncSeek`, and `AsyncBufRead` traits for the futures-rs library.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-lite` v2.6.1
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/futures-lite
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, Contributors to futures-rs
- **Description:** Futures, streams, and async I/O combinators
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```
  - *File: `LICENSE-THIRD-PARTY`*
    ```text
    ===============================================================================

Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
    [... truncated 30 lines ...]
    ```

#### `futures-macro` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** The futures-rs procedural macro implementations.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-sink` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** The asynchronous `Sink` trait for the futures-rs library.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-task` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** Tools for working with tasks.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `futures-util` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/futures-rs
- **Description:** Common utilities and extension traits for the futures-rs library.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 187 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Alex Crichton
Copyright (c) 2017 The Tokio Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `gdk-pixbuf` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the GdkPixbuf library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gdk-pixbuf-sys` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libgdk_pixbuf-2.0
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gdk4` v0.9.6
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings of the GDK 4 library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gdk4-sys` v0.9.6
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings of GDK 4
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `generic-array` v0.14.7
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/fizyk20/generic-array.git
- **Authors / Contributors:** Bartłomiej Kamiński <fizyk20@gmail.com>, Aaron Trent <novacrazy@gmail.com>
- **Description:** Generic types implementing functionality of arrays
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Bartłomiej Kamiński

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `gethostname` v1.1.0
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://codeberg.org/swsnr/gethostname.rs.git
- **Authors / Contributors:** Sebastian Wiesner <sebastian@swsnr.de>
- **Description:** gethostname for all platforms
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```

#### `getrandom` v0.2.17
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-random/getrandom
- **Authors / Contributors:** The Rand Project Developers
- **Description:** A small cross-platform library for retrieving random data from system source
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018-2024 The rust-random Project Developers
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `getrandom` v0.4.3
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-random/getrandom
- **Authors / Contributors:** The Rand Project Developers
- **Description:** A small cross-platform library for retrieving random data from system source
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018-2026 The rust-random Project Developers
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `gio` v0.20.12
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the Gio library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gio-sys` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libgio-2.0
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `glib` v0.20.12
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the GLib library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `glib-macros` v0.20.12
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the GLib library, proc macros crate
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `glib-sys` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libglib-2.0
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `glob` v0.3.4
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/glob
- **Authors / Contributors:** The Rust Project Developers
- **Description:** Support for matching file paths against Unix shell style patterns.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `gobject-sys` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libgobject-2.0
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `graphene-rs` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the Graphene library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `graphene-sys` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libgraphene-1.0
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gsk4` v0.9.6
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings of the GSK 4 library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gsk4-sys` v0.9.6
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings of GSK 4
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gtk4` v0.9.7
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings of the GTK 4 library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gtk4-macros` v0.9.5
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Macros helpers for GTK 4 bindings
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `gtk4-sys` v0.9.6
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk4-rs
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings of GTK 4
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `h2` v0.4.19
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/hyperium/h2
- **Authors / Contributors:** Carl Lerche <me@carllerche.com>, Sean McArthur <sean@seanmonstar.com>
- **Description:** An HTTP/2 client and server
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2017 h2 authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `hashbrown` v0.14.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/hashbrown
- **Authors / Contributors:** Amanieu d'Antras <amanieu@gmail.com>
- **Description:** A Rust port of Google's SwissTable hash map
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Amanieu d'Antras

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `hashbrown` v0.17.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/hashbrown
- **Description:** A Rust port of Google's SwissTable hash map
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Amanieu d'Antras

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `hashlink` v0.9.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/kyren/hashlink
- **Authors / Contributors:** kyren <kerriganw@gmail.com>
- **Description:** HashMap-like containers that hold their key-value pairs in a user controllable order
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    This work is derived in part from the `linked-hash-map` crate, Copyright (c)
2015 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `heck` v0.5.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/withoutboats/heck
- **Description:** heck is a case conversion library.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `hex` v0.4.3
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/KokaKiwi/rust-hex
- **Authors / Contributors:** KokaKiwi <kokakiwi@kokakiwi.net>
- **Description:** Encoding and decoding data into/from hexadecimal representation.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2013-2014 The Rust Project Developers.
Copyright (c) 2015-2020 The rust-hex Developers

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
    [... truncated 5 lines ...]
    ```

#### `hound` v3.5.1
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://github.com/ruuda/hound
- **Authors / Contributors:** Ruud van Asseldonk <dev@veniogames.com>
- **Description:** A wav encoding and decoding library
- **Included Notice / License Content:**
  - *File: `license`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```

#### `http` v1.5.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/hyperium/http
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>, Carl Lerche <me@carllerche.com>, Sean McArthur <sean@seanmonstar.com>
- **Description:** A set of types for representing HTTP requests and responses.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 http-rs authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `http-body` v1.1.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/hyperium/http-body
- **Authors / Contributors:** Carl Lerche <me@carllerche.com>, Lucio Franco <luciofranco14@gmail.com>, Sean McArthur <sean@seanmonstar.com>
- **Description:** Trait representing an asynchronous, streaming, HTTP request or response body.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019-2026 Sean McArthur & Hyper Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `http-body-util` v0.1.5
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/hyperium/http-body
- **Authors / Contributors:** Carl Lerche <me@carllerche.com>, Lucio Franco <luciofranco14@gmail.com>, Sean McArthur <sean@seanmonstar.com>
- **Description:** Combinators and adapters for HTTP request or response bodies.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019-2026 Sean McArthur & Hyper Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `httparse` v1.10.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/seanmonstar/httparse
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** A tiny, safe, speedy, zero-copy HTTP/1.x parser.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015-2025 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `hyper` v1.11.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/hyperium/hyper
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** A protective and efficient HTTP library for all.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2014-2026 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `hyper-tls` v0.6.0
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/hyperium/hyper-tls
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** Default TLS implementation for use with hyper
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `hyper-util` v0.1.21
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/hyperium/hyper-util
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** hyper utilities
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2023-2025 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `icu_collections` v2.3.0
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Collection of API for use in ICU libraries.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `icu_locale_core` v2.3.0
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** API for managing Unicode Language and Locale Identifiers
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `icu_normalizer` v2.3.0
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** API for normalizing text into Unicode Normalization Forms
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `icu_normalizer_data` v2.3.0
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Data for the icu_normalizer crate
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `icu_properties` v2.3.0
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Definitions for Unicode properties
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `icu_properties_data` v2.3.0
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Data for the icu_properties crate
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `icu_provider` v2.3.1
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Trait and struct definitions for the ICU data provider
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `idna` v1.1.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/servo/rust-url/
- **Authors / Contributors:** The rust-url developers
- **Description:** IDNA (Internationalizing Domain Names in Applications) and Punycode.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2013-2025 The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `idna_adapter` v1.2.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/hsivonen/idna_adapter
- **Authors / Contributors:** The rust-url developers
- **Description:** Back end adapter for idna
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `image` v0.25.10
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/image-rs/image
- **Authors / Contributors:** The image-rs Developers
- **Description:** Imaging library. Provides basic image processing and encoders/decoders for common image formats.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `indexmap` v2.14.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/indexmap-rs/indexmap
- **Description:** A hash table with consistent order and fast iteration.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016--2017

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `ipnet` v2.12.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/krisprice/ipnet
- **Authors / Contributors:** Kris Price <kris@krisprice.nz>
- **Description:** Provides types and useful methods for working with IPv4 and IPv6 network addresses, commonly called IP prefixes. The new `IpNet`, `Ipv4Net`, and `Ipv6Net` types build on the existing `IpAddr`, `Ipv4Addr`, and `Ipv6Addr` types already provided in Rust's standard library and align to their design to stay consistent. The module also provides useful traits that extend `Ipv4Addr` and `Ipv6Addr` with methods for `Add`, `Sub`, `BitAnd`, and `BitOr` operations. The module only uses stable feature so it is guaranteed to compile using the stable toolchain.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2017 Juniper Networks, Inc.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
    ```

#### `is_terminal_polyfill` v1.70.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/polyfill-rs/is_terminal_polyfill
- **Description:** Polyfill for `is_terminal` stdlib feature for use with older MSRVs
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `itertools` v0.13.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-itertools/itertools
- **Authors / Contributors:** bluss
- **Description:** Extra iterator adaptors, iterator methods, free functions, and macros.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `itoa` v1.0.18
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/itoa
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Fast integer primitive to string conversion
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `jiff` v0.2.37
- **SPDX License:** `Unlicense OR MIT`
- **Repository / Source:** https://github.com/BurntSushi/jiff
- **Authors / Contributors:** Andrew Gallant <jamslam@gmail.com>
- **Description:** A date-time library that encourages you to jump into the pit of success.

This library is heavily inspired by the Temporal project.

- **Included Notice / License Content:**
  - *File: `COPYING`*
    ```text
    This project is dual-licensed under the Unlicense and MIT licenses.

You may use this code under the terms of either license.
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```
  - *File: `UNLICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `jiff-core` v0.1.1
- **SPDX License:** `Unlicense OR MIT`
- **Repository / Source:** https://github.com/BurntSushi/jiff
- **Authors / Contributors:** Andrew Gallant <jamslam@gmail.com>
- **Description:** Low level datetime primitives for the Jiff library.
- **Included Notice / License Content:**
  - *File: `COPYING`*
    ```text
    This project is dual-licensed under the Unlicense and MIT licenses.

You may use this code under the terms of either license.
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```
  - *File: `UNLICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `ksni` v0.2.2
- **SPDX License:** `Unlicense`
- **Repository / Source:** https://github.com/iovxw/ksni
- **Authors / Contributors:** iovxw <iovxw@outlook.com>
- **Description:** A Rust implementation of the KDE/freedesktop StatusNotifierItem specification

- **Included Notice / License Content:**
  - *File: `UNLICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `libadwaita` v0.7.2
- **SPDX License:** `MIT`
- **Repository / Source:** https://gitlab.gnome.org/World/Rust/libadwaita-rs
- **Authors / Contributors:** Bilal Elmoussaoui <bil.elmoussaoui@gmail.com>
- **Description:** Rust bindings for libadwaita
- **Included Notice / License Content:**
  - *File: `LICENCE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `libadwaita-sys` v0.7.2
- **SPDX License:** `MIT`
- **Repository / Source:** https://gitlab.gnome.org/World/Rust/libadwaita-rs
- **Authors / Contributors:** Bilal Elmoussaoui <bil.elmoussaoui@gmail.com>
- **Description:** FFI bindings for libadwaita
- **Included Notice / License Content:**
  - *File: `LICENCE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `libc` v0.2.189
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/libc
- **Description:** Raw FFI bindings to platform libraries like libc.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `libdbus-sys` v0.2.7
- **SPDX License:** `Apache-2.0/MIT`
- **Repository / Source:** https://github.com/diwic/dbus-rs
- **Authors / Contributors:** David Henningsson <diwic@ubuntu.com>
- **Description:** FFI bindings to libdbus.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014-2018 David Henningsson <diwic@ubuntu.com> and other contributors

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
    [... truncated 4 lines ...]
    ```

#### `libloading` v0.8.9
- **SPDX License:** `ISC`
- **Repository / Source:** https://github.com/nagisa/rust_libloading/
- **Authors / Contributors:** Simonas Kazlauskas <libloading@kazlauskas.me>
- **Description:** Bindings around the platform's dynamic library loading primitives with greatly improved memory safety.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright © 2015, Simonas Kazlauskas

Permission to use, copy, modify, and/or distribute this software for any purpose with or without
fee is hereby granted, provided that the above copyright notice and this permission notice appear
in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
    ```

#### `libsqlite3-sys` v0.28.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/rusqlite/rusqlite
- **Authors / Contributors:** The rusqlite developers
- **Description:** Native bindings to the libsqlite3 library
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2014-2021 The rusqlite developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `linux-raw-sys` v0.12.1
- **SPDX License:** `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/sunfishcode/linux-raw-sys
- **Authors / Contributors:** Dan Gohman <dev@sunfishcode.online>
- **Description:** Generated bindings for Linux's userspace API
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Short version for non-lawyers:

`linux-raw-sys` is triple-licensed under Apache 2.0 with the LLVM Exception,
Apache 2.0, and MIT terms.


Longer version:

Copyrights in the `linux-raw-sys` project are retained by their contributors.
No copyright assignment is required to contribute to the `linux-raw-sys`
project.

Some files include code derived from Rust's `libstd`; see the comments in
the code for details.

    [... truncated 14 lines ...]
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-Apache-2.0_WITH_LLVM-exception`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 203 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `litemap` v0.8.3
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** A key-value Map implementation based on a flat, sorted Vec.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `lock_api` v0.4.14
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/Amanieu/parking_lot
- **Authors / Contributors:** Amanieu d'Antras <amanieu@gmail.com>
- **Description:** Wrappers to create fully-featured Mutex and RwLock types. Compatible with no_std.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `log` v0.4.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/log
- **Authors / Contributors:** The Rust Project Developers
- **Description:** A lightweight logging facade for Rust

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `memchr` v2.8.3
- **SPDX License:** `Unlicense OR MIT`
- **Repository / Source:** https://github.com/BurntSushi/memchr
- **Authors / Contributors:** Andrew Gallant <jamslam@gmail.com>, bluss
- **Description:** Provides extremely fast (uses SIMD on x86_64, aarch64 and wasm32) routines for
1, 2 or 3 byte search and single substring search.

- **Included Notice / License Content:**
  - *File: `COPYING`*
    ```text
    This project is dual-licensed under the Unlicense and MIT licenses.

You may use this code under the terms of either license.
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Andrew Gallant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```
  - *File: `UNLICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `memoffset` v0.9.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/Gilnaa/memoffset
- **Authors / Contributors:** Gilad Naaman <gilad.naaman@gmail.com>
- **Description:** offset_of functionality for Rust structs.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2017 Gilad Naaman

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
    [... truncated 4 lines ...]
    ```

#### `mime` v0.3.17
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/hyperium/mime
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** Strongly Typed Mimes
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `mime_guess` v2.0.5
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/abonander/mime_guess
- **Authors / Contributors:** Austin Bonander <austin.bonander@gmail.com>
- **Description:** A simple crate for detection of a file's MIME type by its extension.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Austin Bonander

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `minimal-lexical` v0.2.1
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/Alexhuszagh/minimal-lexical
- **Authors / Contributors:** Alex Huszagh <ahuszagh@gmail.com>
- **Description:** Fast float parsing conversion routines.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```
  - *File: `LICENSE.md`*
    ```text
    Minimal-lexical is dual licensed under the Apache 2.0 license as well as the MIT
license. See the LICENCE-MIT and the LICENCE-APACHE files for the licenses.

---

`src/bellerophon.rs` is loosely based off the Golang implementation,
found [here](https://github.com/golang/go/blob/b10849fbb97a2244c086991b4623ae9f32c212d0/src/strconv/extfloat.go).
That code (used if the `compact` feature is enabled) is subject to a
[3-clause BSD license](https://github.com/golang/go/blob/b10849fbb97a2244c086991b4623ae9f32c212d0/LICENSE):

Copyright (c) 2009 The Go Authors. All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:
    [... truncated 22 lines ...]
    ```

#### `miniz_oxide` v0.8.9
- **SPDX License:** `MIT OR Zlib OR Apache-2.0`
- **Repository / Source:** https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide
- **Authors / Contributors:** Frommi <daniil.liferenko@gmail.com>, oyvindln <oyvindln@users.noreply.github.com>, Rich Geldreich richgel99@gmail.com
- **Description:** DEFLATE compression and decompression library rewritten in Rust based on miniz
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2017 Frommi
Copyright (c) 2017-2024 oyvindln


Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

    [... truncated 10 lines ...]
    ```
  - *File: `LICENSE-APACHE.md`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT.md`*
    ```text
    MIT License

Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2017 Frommi
Copyright (c) 2017-2024 oyvindln

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
    [... truncated 9 lines ...]
    ```
  - *File: `LICENSE-ZLIB.md`*
    ```text
    Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2020 Frommi
Copyright (c) 2017-2024 oyvindln

This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

3. This notice may not be removed or altered from any source distribution.
    ```

#### `miniz_oxide` v0.9.1
- **SPDX License:** `MIT OR Zlib OR Apache-2.0`
- **Repository / Source:** https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide
- **Authors / Contributors:** Frommi <daniil.liferenko@gmail.com>, oyvindln <oyvindln@users.noreply.github.com>, Rich Geldreich richgel99@gmail.com
- **Description:** DEFLATE compression and decompression library rewritten in Rust based on miniz
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2017 Frommi
Copyright (c) 2017-2024 oyvindln


Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

    [... truncated 10 lines ...]
    ```
  - *File: `LICENSE-APACHE.md`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT.md`*
    ```text
    MIT License

Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2017 Frommi
Copyright (c) 2017-2024 oyvindln

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
    [... truncated 9 lines ...]
    ```
  - *File: `LICENSE-ZLIB.md`*
    ```text
    Copyright 2013-2014 RAD Game Tools and Valve Software
Copyright 2010-2014 Rich Geldreich and Tenacious Software LLC
Copyright (c) 2020 Frommi
Copyright (c) 2017-2024 oyvindln

This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

3. This notice may not be removed or altered from any source distribution.
    ```

#### `mio` v1.2.3
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/mio
- **Authors / Contributors:** Carl Lerche <me@carllerche.com>, Thomas de Zeeuw <thomasdezeeuw@gmail.com>, Tokio Contributors <team@tokio.rs>
- **Description:** Lightweight non-blocking I/O.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2014 Carl Lerche and other MIO contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `moxcms` v0.8.1
- **SPDX License:** `BSD-3-Clause OR Apache-2.0`
- **Repository / Source:** https://github.com/awxkee/moxcms.git
- **Authors / Contributors:** Radzivon Bartoshyk
- **Description:** Simple Color Management in Rust
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE.md`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE.md`*
    ```text
    Copyright (c) Radzivon Bartoshyk. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

1.  Redistributions of source code must retain the above copyright notice, this
    list of conditions and the following disclaimer.

2.  Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation
    and/or other materials provided with the distribution.

3.  Neither the name of the copyright holder nor the names of its
    contributors may be used to endorse or promote products derived from
    this software without specific prior written permission.
    [... truncated 11 lines ...]
    ```

#### `multiversion_no_op` v1.0.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/hsivonen/multiversion_no_op
- **Authors / Contributors:** Henri Sivonen <hsivonen@hsivonen.fi>
- **Description:** Provides a pass-through function attribute called multiversion for optimizing build times
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Copyright Mozilla Foundation

Licensed under the Apache License (Version 2.0), or the MIT license,
(the "Licenses") at your option. You may not use this file except in
compliance with one of the Licenses. You may obtain copies of the
Licenses at:

   https://www.apache.org/licenses/LICENSE-2.0
   https://opensource.org/licenses/MIT

Unless required by applicable law or agreed to in writing, software
distributed under the Licenses is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the Licenses for the specific language governing permissions and
limitations under the Licenses.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright Mozilla Foundation

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `nanorand` v0.7.0
- **SPDX License:** `Zlib`
- **Repository / Source:** https://github.com/Absolucy/nanorand-rs
- **Authors / Contributors:** Lucy <lucy@absolucy.moe>
- **Description:** A tiny, fast, zero-dep library for random number generation.
- **Included Notice / License Content:**
  - *File: `LICENSE.md`*
    ```text
    The zlib/libpng License
=======================

Copyright (c) 2021 lucy

This software is provided 'as-is', without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

1.  The origin of this software must not be misrepresented; you must not claim
    that you wrote the original software. If you use this software in a product,
    [... truncated 7 lines ...]
    ```

#### `native-tls` v0.2.18
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-native-tls/rust-native-tls
- **Authors / Contributors:** Steven Fackler <sfackler@gmail.com>
- **Description:** A wrapper over a platform's native TLS implementation
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 The rust-native-tls Developers

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
    [... truncated 4 lines ...]
    ```

#### `nix` v0.29.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/nix-rust/nix
- **Authors / Contributors:** The nix-rust Project Developers
- **Description:** Rust friendly bindings to *nix APIs
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Carl Lerche + nix-rust Authors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `nom` v7.1.3
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/Geal/nom
- **Authors / Contributors:** contact@geoffroycouprie.com
- **Description:** A byte-oriented, zero-copy, parser combinators library
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2014-2019 Geoffroy Couprie

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    [... truncated 5 lines ...]
    ```

#### `num-complex` v0.4.6
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-num/num-complex
- **Authors / Contributors:** The Rust Project Developers
- **Description:** Complex numbers implementation for Rust
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `num-integer` v0.1.47
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-num/num-integer
- **Authors / Contributors:** The Rust Project Developers
- **Description:** Integer traits and functions
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `num-traits` v0.2.19
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-num/num-traits
- **Authors / Contributors:** The Rust Project Developers
- **Description:** Numeric traits for generic mathematics
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `once_cell` v1.21.4
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/matklad/once_cell
- **Authors / Contributors:** Aleksey Kladov <aleksey.kladov@gmail.com>
- **Description:** Single assignment cells and lazy values.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `openssl` v0.10.81
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://github.com/rust-openssl/rust-openssl
- **Authors / Contributors:** Steven Fackler <sfackler@gmail.com>
- **Description:** OpenSSL bindings
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright 2011-2017 Google Inc.
          2013 Jack Lloyd
          2013-2014 Steven Fackler

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```

#### `openssl-macros` v0.1.1
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://crates.io/crates/openssl-macros
- **Description:** Internal macros used by the openssl crate.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2022 Steven Fackler

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
    [... truncated 4 lines ...]
    ```

#### `openssl-probe` v0.2.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rustls/openssl-probe
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>
- **Description:** A library for helping to find system-wide trust anchor ("root") certificate
locations based on paths typically used by `openssl`.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `openssl-sys` v0.9.117
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/rust-openssl/rust-openssl
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>, Steven Fackler <sfackler@gmail.com>
- **Description:** FFI bindings to OpenSSL
- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `option-ext` v0.2.0
- **SPDX License:** `MPL-2.0`
- **Repository / Source:** https://github.com/soc/option-ext.git
- **Authors / Contributors:** Simon Ochsenreither <simon@ochsenreither.de>
- **Description:** Extends `Option` with additional operations
- **Included Notice / License Content:**
  - *File: `LICENSE.txt`*
    ```text
    Mozilla Public License Version 2.0
==================================

1. Definitions
--------------

1.1. "Contributor"
    means each individual or legal entity that creates, contributes to
    the creation of, or owns Covered Software.

1.2. "Contributor Version"
    means the combination of the Contributions of others (if any) used
    by a Contributor and that particular Contributor's Contribution.

1.3. "Contribution"
    [... truncated 358 lines ...]
    ```

#### `ordered-stream` v0.2.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/danieldg/ordered-stream
- **Authors / Contributors:** Daniel De Graaf <code@danieldg.net>, Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** Streams that are ordered relative to external events
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `pango` v0.20.12
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** Rust bindings for the Pango library
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    The gtk-rs Project is licensed under the MIT license, see the LICENSE file or
<http://opensource.org/licenses/MIT>.

Copyrights in the gtk-rs Project project are retained by their contributors.
No copyright assignment is required to contribute to the gtk-rs Project
project.

For full authorship information, see the version control history.

This project provides interoperability with various GNOME libraries but
doesn't distribute any parts of them. Distributing compiled libraries and
executables that link to those libraries may be subject to terms of the GNU
LGPL or other licenses. For more information check the license of each GNOME
library.
    ```
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `pango-sys` v0.20.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/gtk-rs/gtk-rs-core
- **Authors / Contributors:** The gtk-rs Project Developers
- **Description:** FFI bindings to libpango-1.0
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
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
    [... truncated 2 lines ...]
    ```

#### `parking` v2.2.1
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/parking
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, The Rust Project Developers
- **Description:** Thread parking and unparking
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```
  - *File: `LICENSE-THIRD-PARTY`*
    ```text
    ===============================================================================

Copyright 2014-2020 The Rust Project Developers

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. All files in the project carrying such notice may not be
copied, modified, or distributed except according to those terms.
    ```

#### `parking_lot` v0.12.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/Amanieu/parking_lot
- **Authors / Contributors:** Amanieu d'Antras <amanieu@gmail.com>
- **Description:** More compact and efficient implementations of the standard synchronization primitives.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `parking_lot_core` v0.9.12
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/Amanieu/parking_lot
- **Authors / Contributors:** Amanieu d'Antras <amanieu@gmail.com>
- **Description:** An advanced API for creating custom synchronization primitives.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `percent-encoding` v2.3.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/servo/rust-url/
- **Authors / Contributors:** The rust-url developers
- **Description:** Percent encoding and decoding
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2013-2025 The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `pin-project-lite` v0.2.17
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/taiki-e/pin-project-lite
- **Description:** A lightweight version of pin-project written with declarative macros.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `piper` v0.2.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/smol-rs/piper
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, John Nunley <dev@notgull.net>
- **Description:** An asynchronous single-consumer single-producer pipe for bytes.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `pkg-config` v0.3.34
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/pkg-config-rs
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>
- **Description:** A library to run the pkg-config system tool at build time in order to be used in
Cargo build scripts.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `png` v0.18.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/image-rs/image-png
- **Authors / Contributors:** The image-rs Developers
- **Description:** PNG decoding and encoding library in pure Rust
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 nwin

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `polling` v3.11.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/smol-rs/polling
- **Authors / Contributors:** Stjepan Glavina <stjepang@gmail.com>, John Nunley <dev@notgull.net>
- **Description:** Portable interface to epoll, kqueue, event ports, and IOCP
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `potential_utf` v0.1.6
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Unvalidated string and character types
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `ppv-lite86` v0.2.21
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/cryptocorrosion/cryptocorrosion
- **Authors / Contributors:** The CryptoCorrosion Contributors
- **Description:** Cross-platform cryptography-oriented low-level SIMD library.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2019 The CryptoCorrosion Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `prettyplease` v0.2.37
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/prettyplease
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** A minimal `syn` syntax tree pretty-printer
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `primal-check` v0.3.4
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/huonw/primal
- **Authors / Contributors:** Huon Wilson <dbau.pp@gmail.com>
- **Description:** Fast standalone primality testing.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Huon Wilson

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `proc-macro-crate` v3.5.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/bkchr/proc-macro-crate
- **Authors / Contributors:** Bastian Köcher <git@kchr.de>
- **Description:** Replacement for crate (macro_rules keyword) in proc-macros

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/LICENSE-2.0

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `proc-macro2` v1.0.107
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/proc-macro2
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>, Alex Crichton <alex@alexcrichton.com>
- **Description:** A substitute implementation of the compiler's `proc_macro` API to decouple token-based libraries from the procedural macro use case.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `pxfm` v0.1.30
- **SPDX License:** `BSD-3-Clause OR Apache-2.0`
- **Repository / Source:** https://github.com/awxkee/pxfm
- **Authors / Contributors:** Radzivon Bartoshyk
- **Description:** Fast and accurate math
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE.md`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE.md`*
    ```text
    Copyright (c) Radzivon Bartoshyk. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

1.  Redistributions of source code must retain the above copyright notice, this
    list of conditions and the following disclaimer.

2.  Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation
    and/or other materials provided with the distribution.

3.  Neither the name of the copyright holder nor the names of its
    contributors may be used to endorse or promote products derived from
    this software without specific prior written permission.
    [... truncated 11 lines ...]
    ```

#### `quote` v1.0.47
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/quote
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Quasi-quoting macro quote!(...)
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `rand` v0.8.8
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-random/rand
- **Authors / Contributors:** The Rand Project Developers, The Rust Project Developers
- **Description:** Random number generators and other randomness functionality.

- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Copyrights in the Rand project are retained by their contributors. No
copyright assignment is required to contribute to the Rand project.

For full authorship information, see the version control history.

Except as otherwise noted (below and/or in individual files), Rand is
licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
<http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT> or <http://opensource.org/licenses/MIT>, at your option.

The Rand project includes code from the Rust project
published under these same licenses.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2018 Developers of the Rand project
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `rand_chacha` v0.3.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-random/rand
- **Authors / Contributors:** The Rand Project Developers, The Rust Project Developers, The CryptoCorrosion Contributors
- **Description:** ChaCha random number generator

- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Copyrights in the Rand project are retained by their contributors. No
copyright assignment is required to contribute to the Rand project.

For full authorship information, see the version control history.

Except as otherwise noted (below and/or in individual files), Rand is
licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
<http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT> or <http://opensource.org/licenses/MIT>, at your option.

The Rand project includes code from the Rust project
published under these same licenses.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2018 Developers of the Rand project
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `rand_core` v0.6.4
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-random/rand
- **Authors / Contributors:** The Rand Project Developers, The Rust Project Developers
- **Description:** Core random number generator traits and tools for implementation.

- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Copyrights in the Rand project are retained by their contributors. No
copyright assignment is required to contribute to the Rand project.

For full authorship information, see the version control history.

Except as otherwise noted (below and/or in individual files), Rand is
licensed under the Apache License, Version 2.0 <LICENSE-APACHE> or
<http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT> or <http://opensource.org/licenses/MIT>, at your option.

The Rand project includes code from the Rust project
published under these same licenses.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     https://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 172 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2018 Developers of the Rand project
Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
    [... truncated 11 lines ...]
    ```

#### `realfft` v3.5.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/HEnquist/realfft
- **Authors / Contributors:** HEnquist <henrik.enquist@gmail.com>
- **Description:** Real-to-complex forward FFT and complex-to-real inverse FFT for Rust

#### `regex` v1.13.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/regex
- **Authors / Contributors:** The Rust Project Developers, Andrew Gallant <jamslam@gmail.com>
- **Description:** An implementation of regular expressions for Rust. This implementation uses
finite automata and guarantees linear time matching on all inputs.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `regex-automata` v0.4.18
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/regex
- **Authors / Contributors:** The Rust Project Developers, Andrew Gallant <jamslam@gmail.com>
- **Description:** Automata construction and matching using regular expressions.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `regex-syntax` v0.8.11
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/regex
- **Authors / Contributors:** The Rust Project Developers, Andrew Gallant <jamslam@gmail.com>
- **Description:** A regular expression parser.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `relm4` v0.9.1
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/Relm4/Relm4
- **Authors / Contributors:** Aaron Erhardt <aaron.erhardt@t-online.de>
- **Description:** An idiomatic GUI library inspired by Elm and based on gtk4-rs
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `relm4-css` v0.9.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/Relm4/Relm4
- **Authors / Contributors:** Aaron Erhardt <aaron.erhardt@t-online.de>
- **Description:** An idiomatic GUI library inspired by Elm and based on gtk4-rs

#### `relm4-macros` v0.9.1
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/Relm4/Relm4
- **Authors / Contributors:** Aaron Erhardt <aaron.erhardt@t-online.de>
- **Description:** An idiomatic GUI library inspired by Elm and based on gtk4-rs
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `reqwest` v0.12.28
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/seanmonstar/reqwest
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** higher level HTTP client library
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016-2025 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `rubato` v0.15.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/HEnquist/rubato
- **Authors / Contributors:** HEnquist <henrik.enquist@gmail.com>
- **Description:** Asynchronous resampling library intended for audio data
- **Included Notice / License Content:**
  - *File: `LICENSE.txt`*
    ```text
    Copyright (c) 2020 Henrik Enquist

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
    [... truncated 4 lines ...]
    ```

#### `rusqlite` v0.31.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/rusqlite/rusqlite
- **Authors / Contributors:** The rusqlite developers
- **Description:** Ergonomic wrapper for SQLite
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2014-2021 The rusqlite developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `rustc-hash` v2.1.3
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/rust-lang/rustc-hash
- **Authors / Contributors:** The Rust Project Developers
- **Description:** A speedy, non-cryptographic hashing algorithm used by rustc
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `rustc_version` v0.4.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/djc/rustc-version-rs
- **Description:** A library for querying the version of a installed rustc compiler
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `rustfft` v6.4.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/ejmahler/RustFFT
- **Authors / Contributors:** Allen Welkie <allen.welkie at gmail>, Elliott Mahler <join.together at gmail>
- **Description:** High-performance FFT library written in pure Rust.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 The RustFFT Developers

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
    [... truncated 4 lines ...]
    ```

#### `rustix` v1.1.5
- **SPDX License:** `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/bytecodealliance/rustix
- **Authors / Contributors:** Dan Gohman <dev@sunfishcode.online>, Jakub Konka <kubkon@jakubkonka.com>
- **Description:** Safe Rust bindings to POSIX/Unix/Linux/Winsock-like syscalls
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Short version for non-lawyers:

`rustix` is triple-licensed under Apache 2.0 with the LLVM Exception,
Apache 2.0, and MIT terms.


Longer version:

Copyrights in the `rustix` project are retained by their contributors.
No copyright assignment is required to contribute to the `rustix`
project.

Some files include code derived from Rust's `libstd`; see the comments in
the code for details.

    [... truncated 14 lines ...]
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-Apache-2.0_WITH_LLVM-exception`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 203 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `rustls-pki-types` v1.15.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rustls/pki-types
- **Description:** Shared types for the rustls PKI ecosystem
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2023 Dirkjan Ochtman <dirkjan@ochtman.nl>

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `rustversion` v1.0.23
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/rustversion
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Conditional compilation according to rustc compiler version
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `ryu` v1.0.23
- **SPDX License:** `Apache-2.0 OR BSL-1.0`
- **Repository / Source:** https://github.com/dtolnay/ryu
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Fast floating point to string conversion
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-BOOST`*
    ```text
    Boost Software License - Version 1.0 - August 17th, 2003

Permission is hereby granted, free of charge, to any person or organization
obtaining a copy of the software and accompanying documentation covered by
this license (the "Software") to use, reproduce, display, distribute,
execute, and transmit the Software, and to prepare derivative works of the
Software, and to permit third-parties to whom the Software is furnished to
do so, all subject to the following:

The copyright notices in the Software and this entire statement, including
the above license grant, this restriction and the following disclaimer,
must be included in all copies of the Software, in whole or in part, and
all derivative works of the Software, unless such copies or derivative
works are solely in the form of machine-executable object code generated by
a source language processor.
    [... truncated 8 lines ...]
    ```

#### `scopeguard` v1.2.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/bluss/scopeguard
- **Authors / Contributors:** bluss
- **Description:** A RAII scope guard that will run a given closure when it goes out of scope,
even if the code between panics (assuming unwinding panic).

Defines the macros `defer!`, `defer_on_unwind!`, `defer_on_success!` as
shorthands for guards with one of the implemented strategies.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016-2019 Ulrik Sverdrup "bluss" and scopeguard developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `semver` v1.0.28
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/semver
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Parser and evaluator for Cargo's flavor of Semantic Versioning
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `serde` v1.0.229
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/serde-rs/serde
- **Authors / Contributors:** Erick Tryzelaar <erick.tryzelaar@gmail.com>, David Tolnay <dtolnay@gmail.com>
- **Description:** A generic serialization/deserialization framework
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `serde_core` v1.0.229
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/serde-rs/serde
- **Authors / Contributors:** Erick Tryzelaar <erick.tryzelaar@gmail.com>, David Tolnay <dtolnay@gmail.com>
- **Description:** Serde traits only, with no support for derive -- use the `serde` crate instead
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `serde_derive` v1.0.229
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/serde-rs/serde
- **Authors / Contributors:** Erick Tryzelaar <erick.tryzelaar@gmail.com>, David Tolnay <dtolnay@gmail.com>
- **Description:** Macros 1.1 implementation of #[derive(Serialize, Deserialize)]
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `serde_json` v1.0.151
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/serde-rs/json
- **Authors / Contributors:** Erick Tryzelaar <erick.tryzelaar@gmail.com>, David Tolnay <dtolnay@gmail.com>
- **Description:** A JSON serialization file format
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `serde_repr` v0.1.21
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/serde-repr
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Derive Serialize and Deserialize that delegates to the underlying repr of a C-like enum.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `serde_spanned` v1.1.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/toml-rs/toml
- **Description:** Serde-compatible spanned Value
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `serde_urlencoded` v0.7.1
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/nox/serde_urlencoded
- **Authors / Contributors:** Anthony Ramine <n.oxyde@gmail.com>
- **Description:** `x-www-form-urlencoded` meets Serde
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Anthony Ramine

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `sha1` v0.10.7
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/RustCrypto/hashes
- **Authors / Contributors:** RustCrypto Developers
- **Description:** SHA-1 hash function
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2006-2009 Graydon Hoare
Copyright (c) 2009-2013 Mozilla Foundation
Copyright (c) 2016 Artyom Pavlov

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
    [... truncated 12 lines ...]
    ```

#### `shlex` v1.3.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/comex/rust-shlex
- **Authors / Contributors:** comex <comexk@gmail.com>, Fenhl <fenhl@fenhl.net>, Adrian Taylor <adetaylor@chromium.org>, Alex Touchet <alextouchet@outlook.com>, Daniel Parks <dp+git@oxidized.org>, Garrett Berg <googberg@gmail.com>
- **Description:** Split a string into shell words, like Python's shlex.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Copyright 2015 Nicholas Allegra (comex).

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Nicholas Allegra (comex).

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `shlex` v2.0.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/comex/rust-shlex
- **Authors / Contributors:** comex <comexk@gmail.com>, Fenhl <fenhl@fenhl.net>, Adrian Taylor <adetaylor@chromium.org>, Alex Touchet <alextouchet@outlook.com>, Daniel Parks <dp+git@oxidized.org>, Garrett Berg <googberg@gmail.com>
- **Description:** Split a string into shell words, like Python's shlex.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Copyright 2015 Nicholas Allegra (comex).

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Nicholas Allegra (comex).

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `signal-hook-registry` v1.4.8
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/vorner/signal-hook
- **Authors / Contributors:** Michal 'vorner' Vaner <vorner@vorner.cz>, Masaki Hara <ackie.h.gmai@gmail.com>
- **Description:** Backend crate for signal-hook
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 tokio-jsonrpc developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `simd-adler32` v0.3.10
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/mcountryman/simd-adler32
- **Authors / Contributors:** Marvin Countryman <me@maar.vin>
- **Description:** A SIMD-accelerated Adler-32 hash algorithm implementation.
- **Included Notice / License Content:**
  - *File: `LICENSE.md`*
    ```text
    MIT License

Copyright (c) [2021] [Marvin Countryman]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `simdutf8` v0.1.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rusticstuff/simdutf8
- **Authors / Contributors:** Hans Kratz <hans@appfour.com>
- **Description:** SIMD-accelerated UTF-8 validation.
- **Included Notice / License Content:**
  - *File: `LICENSE-Apache`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

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
    [... truncated 4 lines ...]
    ```

#### `slab` v0.4.12
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/slab
- **Authors / Contributors:** Carl Lerche <me@carllerche.com>
- **Description:** Pre-allocated storage for a uniform data type
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Carl Lerche

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `smallvec` v1.16.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/servo/rust-smallvec
- **Authors / Contributors:** The Servo Project Developers
- **Description:** 'Small vector' optimization: store up to a small number of items on the stack
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018 The Servo Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `socket2` v0.6.5
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/rust-lang/socket2
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>, Thomas de Zeeuw <thomasdezeeuw@gmail.com>
- **Description:** Utilities for handling networking sockets with a maximal amount of configuration
possible intended.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `spin` v0.9.9
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/mvdnes/spin-rs.git
- **Authors / Contributors:** Mathijs van de Nes <git@mathijs.vd-nes.nl>, John Ericson <git@JohnEricson.me>, Joshua Barretto <joshua.s.barretto@gmail.com>
- **Description:** Spin-based synchronization primitives
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2014 Mathijs van de Nes

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `stable_deref_trait` v1.2.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/storyyeller/stable_deref_trait
- **Authors / Contributors:** Robert Grosse <n210241048576@gmail.com>
- **Description:** An unsafe marker trait for types like Box and Rc that dereference to a stable address even when moved, and hence can be used with libraries such as owning_ref and rental.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 Robert Grosse

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `static_assertions` v1.1.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/nvzqz/static-assertions-rs
- **Authors / Contributors:** Nikolai Vazquez
- **Description:** Compile-time assertions to ensure that invariants are met.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    MIT License

Copyright (c) 2017 Nikolai Vazquez

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `strength_reduce` v0.2.4
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** http://github.com/ejmahler/strength_reduce
- **Authors / Contributors:** Elliott Mahler <join.together@gmail.com>
- **Description:** Faster integer division and modulus operations
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 The RustFFT Developers

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
    [... truncated 4 lines ...]
    ```

#### `strsim` v0.8.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dguo/strsim-rs
- **Authors / Contributors:** Danny Guo <dannyguo91@gmail.com>
- **Description:** Implementations of string similarity metrics.
Includes Hamming, Levenshtein, OSA, Damerau-Levenshtein, Jaro, and Jaro-Winkler.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2015 Danny Guo
Copyright (c) 2016 Titus Wormer <tituswormer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

    [... truncated 7 lines ...]
    ```

#### `syn` v2.0.119
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/syn
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Parser for Rust source code
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `syn` v3.0.6
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/syn
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Parser for Rust source code
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `sync_wrapper` v1.0.2
- **SPDX License:** `Apache-2.0`
- **Repository / Source:** https://github.com/Actyx/sync_wrapper
- **Authors / Contributors:** Actyx AG <developer@actyx.io>
- **Description:** A tool for enlisting the compiler's help in proving the absence of concurrency
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```

#### `synstructure` v0.14.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/mystor/synstructure
- **Authors / Contributors:** Nika Layzell <nika@thelayzells.com>
- **Description:** Helper methods and macros for custom derives
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright 2016 Nika Layzell

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
    ```

#### `system-deps` v7.0.8
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/gdesmott/system-deps
- **Authors / Contributors:** Guillaume Desmottes <guillaume.desmottes@collabora.com>, Josh Triplett <josh@joshtriplett.org>
- **Description:** Discover and configure system dependencies from declarative dependencies in Cargo.toml
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `target-lexicon` v0.13.5
- **SPDX License:** `Apache-2.0 WITH LLVM-exception`
- **Repository / Source:** https://github.com/bytecodealliance/target-lexicon
- **Authors / Contributors:** Dan Gohman <sunfish@mozilla.com>
- **Description:** LLVM target triple types
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 203 lines ...]
    ```

#### `textwrap` v0.11.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/mgeisler/textwrap
- **Authors / Contributors:** Martin Geisler <martin@geisler.net>
- **Description:** Textwrap is a small library for word wrapping, indenting, and
dedenting strings.

You can use it to format strings (such as help and error messages) for
display in commandline applications. It is designed to be efficient
and handle Unicode characters correctly.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) 2016 Martin Geisler

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `thiserror` v1.0.69
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/thiserror
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** derive(Error)
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `thiserror` v2.0.21
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/thiserror
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** derive(Error)
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `thiserror-impl` v1.0.69
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/thiserror
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Implementation detail of the `thiserror` crate
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `thiserror-impl` v2.0.21
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/dtolnay/thiserror
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Implementation detail of the `thiserror` crate
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `tinystr` v0.8.4
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** A small ASCII-only bounded length string representation.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `tokio` v1.53.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tokio
- **Authors / Contributors:** Tokio Contributors <team@tokio.rs>
- **Description:** An event-driven, non-blocking I/O platform for writing asynchronous I/O
backed applications.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) Tokio Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `tokio-macros` v2.7.2
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tokio
- **Authors / Contributors:** Tokio Contributors <team@tokio.rs>
- **Description:** Tokio's proc macros.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) 2019 Yoshua Wuyts
Copyright (c) Tokio Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

    [... truncated 7 lines ...]
    ```

#### `tokio-native-tls` v0.3.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tls
- **Authors / Contributors:** Tokio Contributors <team@tokio.rs>
- **Description:** An implementation of TLS/SSL streams for Tokio using native-tls giving an implementation of TLS
for nonblocking I/O streams.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tokio Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tokio-util` v0.7.19
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tokio
- **Authors / Contributors:** Tokio Contributors <team@tokio.rs>
- **Description:** Additional utilities for working with Tokio.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT License

Copyright (c) Tokio Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `toml` v1.1.6+spec-1.1.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/toml-rs/toml
- **Description:** A native Rust encoder and decoder of TOML-formatted files and streams. Provides
implementations of the standard Serialize/Deserialize traits for TOML data to
facilitate deserializing and serializing Rust structures.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `toml_datetime` v1.1.1+spec-1.1.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/toml-rs/toml
- **Description:** A TOML-compatible datetime type
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `toml_edit` v0.25.15+spec-1.1.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/toml-rs/toml
- **Description:** Yet another format-preserving TOML parser.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `toml_parser` v1.1.3+spec-1.1.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/toml-rs/toml
- **Description:** Yet another format-preserving TOML parser.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) Individual contributors

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
    [... truncated 4 lines ...]
    ```

#### `tower` v0.5.3
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tower-rs/tower
- **Authors / Contributors:** Tower Maintainers <team@tower-rs.com>
- **Description:** Tower is a library of modular and reusable components for building robust
clients and servers.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tower Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tower-http` v0.6.11
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tower-rs/tower-http
- **Authors / Contributors:** Tower Maintainers <team@tower-rs.com>
- **Description:** Tower middleware and utilities for HTTP clients and servers
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019-2021 Tower Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tower-layer` v0.3.3
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tower-rs/tower
- **Authors / Contributors:** Tower Maintainers <team@tower-rs.com>
- **Description:** Decorates a `Service` to allow easy composition between `Service`s.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tower Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tower-service` v0.3.3
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tower-rs/tower
- **Authors / Contributors:** Tower Maintainers <team@tower-rs.com>
- **Description:** Trait representing an asynchronous, request / response based, client or server.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tower Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tracing` v0.1.44
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tracing
- **Authors / Contributors:** Eliza Weisman <eliza@buoyant.io>, Tokio Contributors <team@tokio.rs>
- **Description:** Application-level tracing for Rust.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tokio Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tracing-attributes` v0.1.31
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tracing
- **Authors / Contributors:** Tokio Contributors <team@tokio.rs>, Eliza Weisman <eliza@buoyant.io>, David Barsky <dbarsky@amazon.com>
- **Description:** Procedural macro attributes for automatically instrumenting functions.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tokio Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `tracing-core` v0.1.36
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/tokio-rs/tracing
- **Authors / Contributors:** Tokio Contributors <team@tokio.rs>
- **Description:** Core primitives for application-level tracing.

- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2019 Tokio Contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `transpose` v0.2.3
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/ejmahler/transpose
- **Authors / Contributors:** Elliott Mahler <join.together@gmail.com>
- **Description:** Utility for transposing multi-dimensional data
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2022 The transpose Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `try-lock` v0.2.5
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/seanmonstar/try-lock
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** A lightweight atomic lock.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2018-2023 Sean McArthur
Copyright (c) 2016 Alex Crichton

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    [... truncated 5 lines ...]
    ```

#### `typenum` v1.20.1
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/paholg/typenum
- **Description:** Typenum is a Rust library for type-level numbers evaluated at
    compile time. It currently supports bits, unsigned integers, and signed
    integers. It also provides a type-level array of type-level numbers, but its
    implementation is incomplete.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    MIT OR Apache-2.0
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)

Copyright (c) 2014 Paho Lurie-Gregg

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `unicase` v2.9.0
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/seanmonstar/unicase
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** A case-insensitive wrapper around strings.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2014-2026 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `unicode-ident` v1.0.26
- **SPDX License:** `(MIT OR Apache-2.0) AND Unicode-3.0`
- **Repository / Source:** https://github.com/dtolnay/unicode-ident
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** Determine whether characters have the XID_Start or XID_Continue properties according to Unicode Standard Annex #31
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```
  - *File: `LICENSE-UNICODE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 1991-2023 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 24 lines ...]
    ```

#### `unicode-width` v0.1.14
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/unicode-rs/unicode-width
- **Authors / Contributors:** kwantam <kwantam@gmail.com>, Manish Goregaokar <manishsmail@gmail.com>
- **Description:** Determine displayed width of `char` and `str` types
according to Unicode Standard Annex #11 rules.

- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Licensed under the Apache License, Version 2.0
<LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT
license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
at your option. All files in the project carrying such
notice may not be copied, modified, or distributed except
according to those terms.
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `url` v2.5.8
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/servo/rust-url
- **Authors / Contributors:** The rust-url developers
- **Description:** URL library for Rust, based on the WHATWG URL Standard
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2013-2025 The rust-url developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `utf8_iter` v1.0.4
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/hsivonen/utf8_iter
- **Authors / Contributors:** Henri Sivonen <hsivonen@hsivonen.fi>
- **Description:** Iterator by char over potentially-invalid UTF-8 in &[u8]
- **Included Notice / License Content:**
  - *File: `COPYRIGHT`*
    ```text
    Copyright Mozilla Foundation

Licensed under the Apache License (Version 2.0), or the MIT license,
(the "Licenses") at your option. You may not use this file except in
compliance with one of the Licenses. You may obtain copies of the
Licenses at:

   https://www.apache.org/licenses/LICENSE-2.0
   https://opensource.org/licenses/MIT

Unless required by applicable law or agreed to in writing, software
distributed under the Licenses is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the Licenses for the specific language governing permissions and
limitations under the Licenses.
    [... truncated 27 lines ...]
    ```
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright Mozilla Foundation

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `utf8parse` v0.2.2
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/alacritty/vte
- **Authors / Contributors:** Joe Wilm <joe@jwilm.com>, Christian Duerr <contact@christianduerr.com>
- **Description:** Table-driven UTF-8 parser
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 161 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2016 Joe Wilm

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `vcpkg` v0.2.15
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/mcgoo/vcpkg-rs
- **Authors / Contributors:** Jim McGrath <jimmc2@gmail.com>
- **Description:** A library to find native dependencies in a vcpkg tree at build
time in order to be used in Cargo build scripts.

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2017 Jim McGrath

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `vec_map` v0.8.2
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/contain-rs/vec-map
- **Authors / Contributors:** Alex Crichton <alex@alexcrichton.com>, Jorge Aparicio <japaricious@gmail.com>, Alexis Beingessner <a.beingessner@gmail.com>, Brian Anderson <>, tbu- <>, Manish Goregaokar <>, Aaron Turon <aturon@mozilla.com>, Adolfo Ochagavía <>, Niko Matsakis <>, Steven Fackler <>, Chase Southwood <csouth3@illinois.edu>, Eduard Burtescu <>, Florian Wilkens <>, Félix Raimundo <>, Tibor Benke <>, Markus Siemens <markus@m-siemens.de>, Josh Branchaud <jbranchaud@gmail.com>, Huon Wilson <dbau.pp@gmail.com>, Corey Farwell <coref@rwell.org>, Aaron Liblong <>, Nick Cameron <nrc@ncameron.org>, Patrick Walton <pcwalton@mimiga.net>, Felix S Klock II <>, Andrew Paseltiner <apaseltiner@gmail.com>, Sean McArthur <sean.monstar@gmail.com>, Vadim Petrochenkov <>
- **Description:** A simple map based on a vector for small integer keys
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2015 The Rust Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `version-compare` v0.2.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://gitlab.com/timvisee/version-compare
- **Authors / Contributors:** Tim Visee <3a4fb3964f@sinenomine.email>
- **Description:** Rust library to easily compare version numbers with no specific format, and test against various comparison operators.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2017 Tim Visée

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
    [... truncated 4 lines ...]
    ```

#### `version_check` v0.9.5
- **SPDX License:** `MIT/Apache-2.0`
- **Repository / Source:** https://github.com/SergioBenitez/version_check
- **Authors / Contributors:** Sergio Benitez <sb@sergio.bz>
- **Description:** Tiny crate to check the version of the installed/running rustc.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                        Version 2.0, January 2004
                     http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.

   "License" shall mean the terms and conditions for use, reproduction,
   and distribution as defined by Sections 1 through 9 of this document.

   "Licensor" shall mean the copyright owner or entity authorized by
   the copyright owner that is granting the License.

   "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    The MIT License (MIT)
Copyright (c) 2017-2018 Sergio Benitez

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    [... truncated 4 lines ...]
    ```

#### `want` v0.3.1
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/seanmonstar/want
- **Authors / Contributors:** Sean McArthur <sean@seanmonstar.com>
- **Description:** Detect when another Future wants a result.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2018-2019 Sean McArthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    [... truncated 4 lines ...]
    ```

#### `whisper-rs` v0.16.0
- **SPDX License:** `Unlicense`
- **Repository / Source:** https://codeberg.org/tazz4843/whisper-rs
- **Description:** Rust bindings for whisper.cpp
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

    [... truncated 9 lines ...]
    ```

#### `whisper-rs-sys` v0.15.0
- **SPDX License:** `Unlicense`
- **Repository / Source:** https://codeberg.org/tazz4843/whisper-rs
- **Description:** Rust bindings for whisper.cpp (FFI bindings)

#### `winnow` v1.0.4
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/winnow-rs/winnow
- **Description:** A byte-oriented, zero-copy, parser combinators library
- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
    [... truncated 3 lines ...]
    ```

#### `writeable` v0.6.4
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** A more efficient alternative to fmt::Display
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `x11rb` v0.13.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/psychon/x11rb
- **Authors / Contributors:** Uli Schlachter <psychon@znc.in>, Eduardo Sánchez Muñoz <eduardosm-dev@e64.io>, notgull <jtnunley01@gmail.com>
- **Description:** Rust bindings to X11
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2019 x11rb Contributers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `x11rb-protocol` v0.13.2
- **SPDX License:** `MIT OR Apache-2.0`
- **Repository / Source:** https://github.com/psychon/x11rb
- **Authors / Contributors:** Uli Schlachter <psychon@znc.in>, Eduardo Sánchez Muñoz <eduardosm-dev@e64.io>, notgull <jtnunley01@gmail.com>
- **Description:** Rust bindings to X11
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2019 x11rb Contributers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `xdg-home` v1.3.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/zeenix/xdg-home
- **Authors / Contributors:** Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** The user's home directory as per XDG Specification
- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `xml-rs` v0.8.29
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/kornelski/xml-rs
- **Authors / Contributors:** Vladimir Matveev <vmatveev@citrine.cc>
- **Description:** An XML library in pure Rust
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    The MIT License (MIT)

Copyright (c) 2014 Vladimir Matveev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    [... truncated 6 lines ...]
    ```

#### `yoke` v0.8.3
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** Manish Goregaokar <manishsmail@gmail.com>
- **Description:** Abstraction allowing borrowed data to be carried along with the backing data it borrows from
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `yoke-derive` v0.8.4
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** Manish Goregaokar <manishsmail@gmail.com>
- **Description:** Custom derive for the yoke crate
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `zbus` v4.4.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dbus2/zbus/
- **Authors / Contributors:** Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** API for D-Bus communication
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2024 Zeeshan Ali Khan & zbus contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `zbus_macros` v4.4.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dbus2/zbus/
- **Authors / Contributors:** Marc-André Lureau <marcandre.lureau@redhat.com>, Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** proc-macros for zbus
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2024 Zeeshan Ali Khan & zbus contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `zbus_names` v3.0.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dbus2/zbus/
- **Authors / Contributors:** Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** A collection of D-Bus bus names types
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `zerocopy` v0.8.59
- **SPDX License:** `BSD-2-Clause OR Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/google/zerocopy
- **Description:** Zerocopy makes zero-cost memory manipulation effortless. We write "unsafe" so you don't have to.
- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-BSD`*
    ```text
    Copyright 2019 The Fuchsia Authors.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

   * Redistributions of source code must retain the above copyright
notice, this list of conditions and the following disclaimer.
   * Redistributions in binary form must reproduce the above
copyright notice, this list of conditions and the following disclaimer
in the documentation and/or other materials provided with the
distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
    [... truncated 9 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright 2023 The Fuchsia Authors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `zerofrom` v0.1.8
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** ZeroFrom trait for constructing
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `zerofrom-derive` v0.1.8
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** Manish Goregaokar <manishsmail@gmail.com>
- **Description:** Custom derive for the zerofrom crate
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `zeroize` v1.9.0
- **SPDX License:** `Apache-2.0 OR MIT`
- **Repository / Source:** https://github.com/RustCrypto/utils
- **Authors / Contributors:** The RustCrypto Project Developers
- **Description:** Securely clear secrets from memory with a simple trait built on
stable Rust primitives which guarantee memory is zeroed using an
operation will not be 'optimized away' by the compiler.
Uses a portable pure Rust implementation that works everywhere,
even WASM!

- **Included Notice / License Content:**
  - *File: `LICENSE-APACHE`*
    ```text
    Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
    [... truncated 186 lines ...]
    ```
  - *File: `LICENSE-MIT`*
    ```text
    Copyright (c) 2018-2026 The RustCrypto Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `zerotrie` v0.2.5
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** A data structure that efficiently maps strings to integers
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `zerovec` v0.11.8
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** The ICU4X Project Developers
- **Description:** Zero-copy vector backed by a byte array
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `zerovec-derive` v0.11.6
- **SPDX License:** `Unicode-3.0`
- **Repository / Source:** https://github.com/unicode-org/icu4x
- **Authors / Contributors:** Manish Goregaokar <manishsmail@gmail.com>
- **Description:** Custom derive for the zerovec crate
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    UNICODE LICENSE V3

COPYRIGHT AND PERMISSION NOTICE

Copyright © 2020-2024 Unicode, Inc.

NOTICE TO USER: Carefully read the following legal agreement. BY
DOWNLOADING, INSTALLING, COPYING OR OTHERWISE USING DATA FILES, AND/OR
SOFTWARE, YOU UNEQUIVOCALLY ACCEPT, AND AGREE TO BE BOUND BY, ALL OF THE
TERMS AND CONDITIONS OF THIS AGREEMENT. IF YOU DO NOT AGREE, DO NOT
DOWNLOAD, INSTALL, COPY, DISTRIBUTE OR USE THE DATA FILES OR SOFTWARE.

Permission is hereby granted, free of charge, to any person obtaining a
copy of data files and any associated documentation (the "Data Files") or
software and any associated documentation (the "Software") to deal in the
    [... truncated 31 lines ...]
    ```

#### `zmij` v1.0.23
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dtolnay/zmij
- **Authors / Contributors:** David Tolnay <dtolnay@gmail.com>
- **Description:** A double-to-string conversion algorithm based on Schubfach and xjb
- **Included Notice / License Content:**
  - *File: `LICENSE-MIT`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

#### `zvariant` v4.2.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dbus2/zbus/
- **Authors / Contributors:** Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** D-Bus & GVariant encoding & decoding
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2024 Zeeshan Ali Khan & zbus contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `zvariant_derive` v4.2.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dbus2/zbus/
- **Authors / Contributors:** Zeeshan Ali Khan <zeeshanak@gnome.org>
- **Description:** D-Bus & GVariant encoding & decoding
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Copyright (c) 2024 Zeeshan Ali Khan & zbus contributors

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
    [... truncated 10 lines ...]
    ```

#### `zvariant_utils` v2.1.0
- **SPDX License:** `MIT`
- **Repository / Source:** https://github.com/dbus2/zbus/
- **Authors / Contributors:** Zeeshan Ali Khan <zeeshanak@gnome.org>, turbocooler <turbocooler@cocaine.ninja>
- **Description:** Various utilities used internally by the zvariant crate.
- **Included Notice / License Content:**
  - *File: `LICENSE`*
    ```text
    Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    [... truncated 8 lines ...]
    ```

### 3.2 Target-Specific Inactive Dependencies (136 Crates)

The following crates are declared in `Cargo.lock` but are conditional on Windows (`cfg(windows)`), macOS/iOS (`cfg(target_vendor = "apple")`), Android (`cfg(target_os = "android")`), or web targets, and are **not linked** into the Linux ELF executable:

| Crate Name | Version | Declared SPDX License | Target Platform / Purpose |
| :--- | :--- | :--- | :--- |
| `anstyle-wincon` | `3.0.11` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `async-process` | `2.5.0` | `Apache-2.0 OR MIT` | Other OS / Architecture Fallback |
| `async-recursion` | `1.1.1` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `async-signal` | `0.2.14` | `Apache-2.0 OR MIT` | Other OS / Architecture Fallback |
| `bumpalo` | `3.20.3` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `cesu8` | `1.1.0` | `Apache-2.0/MIT` | Other OS / Architecture Fallback |
| `clipboard-win` | `5.4.1` | `BSL-1.0` | Windows (`cfg(windows)`) |
| `combine` | `4.6.8` | `MIT` | Other OS / Architecture Fallback |
| `core-foundation` | `0.9.4` | `MIT OR Apache-2.0` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `core-foundation` | `0.10.1` | `MIT OR Apache-2.0` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `core-foundation-sys` | `0.8.7` | `MIT OR Apache-2.0` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `coreaudio-rs` | `0.11.3` | `MIT/Apache-2.0` | Other OS / Architecture Fallback |
| `coreaudio-sys` | `0.2.18` | `MIT` | Other OS / Architecture Fallback |
| `crunchy` | `0.2.4` | `MIT` | Other OS / Architecture Fallback |
| `defmt` | `1.1.1` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `defmt-macros` | `1.1.1` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `defmt-parser` | `1.0.0` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `dispatch2` | `0.3.1` | `Zlib OR Apache-2.0 OR MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `error-code` | `3.4.0` | `BSL-1.0` | Other OS / Architecture Fallback |
| `fax` | `0.2.7` | `MIT` | Other OS / Architecture Fallback |
| `half` | `2.7.1` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `hermit-abi` | `0.1.19` | `MIT/Apache-2.0` | Other OS / Architecture Fallback |
| `hermit-abi` | `0.5.3` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `hyper-rustls` | `0.27.10` | `Apache-2.0 OR ISC OR MIT` | Other OS / Architecture Fallback |
| `jiff-static` | `0.2.37` | `Unlicense OR MIT` | Other OS / Architecture Fallback |
| `jni` | `0.21.1` | `MIT/Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `jni-sys` | `0.3.1` | `MIT OR Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `jni-sys` | `0.4.1` | `MIT OR Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `jni-sys-macros` | `0.4.1` | `MIT OR Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `jobserver` | `0.1.35` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `js-sys` | `0.3.106` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `libredox` | `0.1.25` | `MIT` | Other OS / Architecture Fallback |
| `mach2` | `0.4.3` | `BSD-2-Clause OR MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `ndk` | `0.8.0` | `MIT OR Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `ndk-context` | `0.1.1` | `MIT OR Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `ndk-sys` | `0.5.0+25.2.9519653` | `MIT OR Apache-2.0` | Android (`cfg(target_os = "android")`) |
| `num-derive` | `0.4.2` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `num_enum` | `0.7.6` | `BSD-3-Clause OR MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `num_enum_derive` | `0.7.6` | `BSD-3-Clause OR MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `objc2` | `0.6.4` | `MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `objc2-app-kit` | `0.3.2` | `Zlib OR Apache-2.0 OR MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `objc2-core-foundation` | `0.3.2` | `Zlib OR Apache-2.0 OR MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `objc2-core-graphics` | `0.3.2` | `Zlib OR Apache-2.0 OR MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `objc2-encode` | `4.1.0` | `MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `objc2-foundation` | `0.3.2` | `MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `objc2-io-surface` | `0.3.2` | `Zlib OR Apache-2.0 OR MIT` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `oboe` | `0.6.1` | `Apache-2.0` | Other OS / Architecture Fallback |
| `oboe-sys` | `0.6.1` | `Apache-2.0` | Other OS / Architecture Fallback |
| `once_cell_polyfill` | `1.70.2` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `portable-atomic` | `1.15.0` | `Apache-2.0 OR MIT` | Other OS / Architecture Fallback |
| `portable-atomic-util` | `0.2.8` | `Apache-2.0 OR MIT` | Other OS / Architecture Fallback |
| `quick-error` | `2.0.1` | `MIT/Apache-2.0` | Other OS / Architecture Fallback |
| `r-efi` | `6.0.0` | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` | Other OS / Architecture Fallback |
| `redox_syscall` | `0.5.18` | `MIT` | Other OS / Architecture Fallback |
| `redox_users` | `0.4.6` | `MIT` | Other OS / Architecture Fallback |
| `ring` | `0.17.14` | `Apache-2.0 AND ISC` | Other OS / Architecture Fallback |
| `rustls` | `0.23.46` | `Apache-2.0 OR ISC OR MIT` | Other OS / Architecture Fallback |
| `rustls-webpki` | `0.103.15` | `ISC` | Other OS / Architecture Fallback |
| `same-file` | `1.0.6` | `Unlicense/MIT` | Other OS / Architecture Fallback |
| `schannel` | `0.1.29` | `MIT` | Windows (`cfg(windows)`) |
| `security-framework` | `3.7.0` | `MIT OR Apache-2.0` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `security-framework-sys` | `2.17.0` | `MIT OR Apache-2.0` | macOS / iOS (`cfg(target_vendor = "apple")`) |
| `subtle` | `2.6.1` | `BSD-3-Clause` | Other OS / Architecture Fallback |
| `system-configuration` | `0.7.0` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `system-configuration-sys` | `0.6.0` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `tempfile` | `3.27.0` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `tiff` | `0.11.3` | `MIT` | Other OS / Architecture Fallback |
| `tokio-rustls` | `0.26.5` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `toml_writer` | `1.1.2+spec-1.1.0` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `uds_windows` | `1.2.1` | `MIT` | Windows (`cfg(windows)`) |
| `untrusted` | `0.9.0` | `ISC` | Other OS / Architecture Fallback |
| `walkdir` | `2.5.0` | `Unlicense/MIT` | Other OS / Architecture Fallback |
| `wasi` | `0.11.1+wasi-snapshot-preview1` | `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` | Other OS / Architecture Fallback |
| `wasm-bindgen` | `0.2.129` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `wasm-bindgen-futures` | `0.4.79` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `wasm-bindgen-macro` | `0.2.129` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `wasm-bindgen-macro-support` | `0.2.129` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `wasm-bindgen-shared` | `0.2.129` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `wasm-streams` | `0.4.2` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `web-sys` | `0.3.106` | `MIT OR Apache-2.0` | WebAssembly (`target_arch = "wasm32"`) |
| `weezl` | `0.1.12` | `MIT OR Apache-2.0` | Other OS / Architecture Fallback |
| `winapi` | `0.3.9` | `MIT/Apache-2.0` | Windows (`cfg(windows)`) |
| `winapi-i686-pc-windows-gnu` | `0.4.0` | `MIT/Apache-2.0` | Windows (`cfg(windows)`) |
| `winapi-util` | `0.1.11` | `Unlicense OR MIT` | Windows (`cfg(windows)`) |
| `winapi-x86_64-pc-windows-gnu` | `0.4.0` | `MIT/Apache-2.0` | Windows (`cfg(windows)`) |
| `windows` | `0.54.0` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-core` | `0.54.0` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-link` | `0.2.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-registry` | `0.6.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-result` | `0.1.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-result` | `0.4.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-strings` | `0.5.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-sys` | `0.45.0` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-sys` | `0.48.0` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-sys` | `0.52.0` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-sys` | `0.59.0` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-sys` | `0.60.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-sys` | `0.61.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-targets` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-targets` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-targets` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows-targets` | `0.53.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_gnullvm` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_gnullvm` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_gnullvm` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_gnullvm` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_msvc` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_msvc` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_msvc` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_aarch64_msvc` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_gnu` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_gnu` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_gnu` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_gnu` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_gnullvm` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_gnullvm` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_msvc` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_msvc` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_msvc` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_i686_msvc` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnu` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnu` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnu` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnu` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnullvm` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnullvm` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnullvm` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_gnullvm` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_msvc` | `0.42.2` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_msvc` | `0.48.5` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_msvc` | `0.52.6` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `windows_x86_64_msvc` | `0.53.1` | `MIT OR Apache-2.0` | Windows (`cfg(windows)`) |
| `zerocopy-derive` | `0.8.59` | `BSD-2-Clause OR Apache-2.0 OR MIT` | Other OS / Architecture Fallback |
| `zlib-rs` | `0.6.8` | `Zlib` | Other OS / Architecture Fallback |
| `zune-core` | `0.5.3` | `MIT OR Apache-2.0 OR Zlib` | Other OS / Architecture Fallback |
| `zune-jpeg` | `0.5.15` | `MIT OR Apache-2.0 OR Zlib` | Other OS / Architecture Fallback |

---

## 4. Full Texts of Governed Licenses

### 4.1 MIT License
```text
MIT License

Copyright (c) 2026 H. K. Saikia

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
```

### 4.2 Apache License, Version 2.0
```text
Apache License
                           Version 2.0, January 2004
                        http://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

### 4.3 Mozilla Public License, Version 2.0 (MPL-2.0)
```text
Mozilla Public License Version 2.0
==================================

1. Definitions
--------------

1.1. "Contributor"
    means each individual or legal entity that creates, contributes to
    the creation of, or owns Covered Software.

1.2. "Contributor Version"
    means the combination of the Contributions of others (if any) used
    by a Contributor and that particular Contributor's Contribution.

1.3. "Contribution"
    means Covered Software of a particular Contributor.

1.4. "Covered Software"
    means Source Code Form to which the initial Contributor has attached
    the notice in Exhibit A, the Executable Form of such Source Code
    Form, and Modifications of such Source Code Form, in each case
    including portions thereof.

1.5. "Incompatible With Secondary Licenses"
    means

    (a) that the initial Contributor has attached the notice described
        in Exhibit B to the Covered Software; or

    (b) that the Covered Software was made available under the terms of
        version 1.1 or earlier of the License, but not also under the
        terms of a Secondary License.

1.6. "Executable Form"
    means any form of the work other than Source Code Form.

1.7. "Larger Work"
    means a work that combines Covered Software with other material, in 
    a separate file or files, that is not Covered Software.

1.8. "License"
    means this document.

1.9. "Licensable"
    means having the right to grant, to the maximum extent possible,
    whether at the time of the initial grant or subsequently, any and
    all of the rights conveyed by this License.

1.10. "Modifications"
    means any of the following:

    (a) any file in Source Code Form that results from an addition to,
        deletion from, or modification of the contents of Covered
        Software; or

    (b) any new file in Source Code Form that contains any Covered
        Software.

1.11. "Patent Claims" of a Contributor
    means any patent claim(s), including without limitation, method,
    process, and apparatus claims, in any patent Licensable by such
    Contributor that would be infringed, but for the grant of the
    License, by the making, using, selling, offering for sale, having
    made, import, or transfer of either its Contributions or its
    Contributor Version.

1.12. "Secondary License"
    means either the GNU General Public License, Version 2.0, the GNU
    Lesser General Public License, Version 2.1, the GNU Affero General
    Public License, Version 3.0, or any later versions of those
    licenses.

1.13. "Source Code Form"
    means the form of the work preferred for making modifications.

1.14. "You" (or "Your")
    means an individual or a legal entity exercising rights under this
    License. For legal entities, "You" includes any entity that
    controls, is controlled by, or is under common control with You. For
    purposes of this definition, "control" means (a) the power, direct
    or indirect, to cause the direction or management of such entity,
    whether by contract or otherwise, or (b) ownership of more than
    fifty percent (50%) of the outstanding shares or beneficial
    ownership of such entity.

2. License Grants and Conditions
--------------------------------

2.1. Grants

Each Contributor hereby grants You a world-wide, royalty-free,
non-exclusive license:

(a) under intellectual property rights (other than patent or trademark)
    Licensable by such Contributor to use, reproduce, make available,
    modify, display, perform, distribute, and otherwise exploit its
    Contributions, either on an unmodified basis, with Modifications, or
    as part of a Larger Work; and

(b) under Patent Claims of such Contributor to make, use, sell, offer
    for sale, have made, import, and otherwise transfer either its
    Contributions or its Contributor Version.

2.2. Effective Date

The licenses granted in Section 2.1 with respect to any Contribution
become effective for each Contribution on the date the Contributor first
distributes such Contribution.

2.3. Limitations on Grant Scope

The licenses granted in this Section 2 are the only rights granted under
this License. No additional rights or licenses will be implied from the
distribution or licensing of Covered Software under this License.
Notwithstanding Section 2.1(b) above, no patent license is granted by a
Contributor:

(a) for any code that a Contributor has removed from Covered Software;
    or

(b) for infringements caused by: (i) Your and any other third party's
    modifications of Covered Software, or (ii) the combination of its
    Contributions with other software (except as part of its Contributor
    Version); or

(c) under Patent Claims infringed by Covered Software in the absence of
    its Contributions.

This License does not grant any rights in the trademarks, service marks,
or logos of any Contributor (except as may be necessary to comply with
the notice requirements in Section 3.4).

2.4. Subsequent Licenses

No Contributor makes additional grants as a result of Your choice to
distribute the Covered Software under a subsequent version of this
License (see Section 10.2) or under the terms of a Secondary License (if
permitted under the terms of Section 3.3).

2.5. Representation

Each Contributor represents that the Contributor believes its
Contributions are its original creation(s) or it has sufficient rights
to grant the rights to its Contributions conveyed by this License.

2.6. Fair Use

This License is not intended to limit any rights You have under
applicable copyright doctrines of fair use, fair dealing, or other
equivalents.

2.7. Conditions

Sections 3.1, 3.2, 3.3, and 3.4 are conditions of the licenses granted
in Section 2.1.

3. Responsibilities
-------------------

3.1. Distribution of Source Form

All distribution of Covered Software in Source Code Form, including any
Modifications that You create or to which You contribute, must be under
the terms of this License. You must inform recipients that the Source
Code Form of the Covered Software is governed by the terms of this
License, and how they can obtain a copy of this License. You may not
attempt to alter or restrict the recipients' rights in the Source Code
Form.

3.2. Distribution of Executable Form

If You distribute Covered Software in Executable Form then:

(a) such Covered Software must also be made available in Source Code
    Form, as described in Section 3.1, and You must inform recipients of
    the Executable Form how they can obtain a copy of such Source Code
    Form by reasonable means in a timely manner, at a charge no more
    than the cost of distribution to the recipient; and

(b) You may distribute such Executable Form under the terms of this
    License, or sublicense it under different terms, provided that the
    license for the Executable Form does not attempt to limit or alter
    the recipients' rights in the Source Code Form under this License.

3.3. Distribution of a Larger Work

You may create and distribute a Larger Work under terms of Your choice,
provided that You also comply with the requirements of this License for
the Covered Software. If the Larger Work is a combination of Covered
Software with a work governed by one or more Secondary Licenses, and the
Covered Software is not Incompatible With Secondary Licenses, this
License permits You to additionally distribute such Covered Software
under the terms of such Secondary License(s), so that the recipient of
the Larger Work may, at their option, further distribute the Covered
Software under the terms of either this License or such Secondary
License(s).

3.4. Notices

You may not remove or alter the substance of any license notices
(including copyright notices, patent notices, disclaimers of warranty,
or limitations of liability) contained within the Source Code Form of
the Covered Software, except that You may alter any license notices to
the extent required to remedy known factual inaccuracies.

3.5. Application of Additional Terms

You may choose to offer, and to charge a fee for, warranty, support,
indemnity or liability obligations to one or more recipients of Covered
Software. However, You may do so only on Your own behalf, and not on
behalf of any Contributor. You must make it absolutely clear that any
such warranty, support, indemnity, or liability obligation is offered by
You alone, and You hereby agree to indemnify every Contributor for any
liability incurred by such Contributor as a result of warranty, support,
indemnity or liability terms You offer. You may include additional
disclaimers of warranty and limitations of liability specific to any
jurisdiction.

4. Inability to Comply Due to Statute or Regulation
---------------------------------------------------

If it is impossible for You to comply with any of the terms of this
License with respect to some or all of the Covered Software due to
statute, judicial order, or regulation then You must: (a) comply with
the terms of this License to the maximum extent possible; and (b)
describe the limitations and the code they affect. Such description must
be placed in a text file included with all distributions of the Covered
Software under this License. Except to the extent prohibited by statute
or regulation, such description must be sufficiently detailed for a
recipient of ordinary skill to be able to understand it.

5. Termination
--------------

5.1. The rights granted under this License will terminate automatically
if You fail to comply with any of its terms. However, if You become
compliant, then the rights granted under this License from a particular
Contributor are reinstated (a) provisionally, unless and until such
Contributor explicitly and finally terminates Your grants, and (b) on an
ongoing basis, if such Contributor fails to notify You of the
non-compliance by some reasonable means prior to 60 days after You have
come back into compliance. Moreover, Your grants from a particular
Contributor are reinstated on an ongoing basis if such Contributor
notifies You of the non-compliance by some reasonable means, this is the
first time You have received notice of non-compliance with this License
from such Contributor, and You become compliant prior to 30 days after
Your receipt of the notice.

5.2. If You initiate litigation against any entity by asserting a patent
infringement claim (excluding declaratory judgment actions,
counter-claims, and cross-claims) alleging that a Contributor Version
directly or indirectly infringes any patent, then the rights granted to
You by any and all Contributors for the Covered Software under Section
2.1 of this License shall terminate.

5.3. In the event of termination under Sections 5.1 or 5.2 above, all
end user license agreements (excluding distributors and resellers) which
have been validly granted by You or Your distributors under this License
prior to termination shall survive termination.

************************************************************************
*                                                                      *
*  6. Disclaimer of Warranty                                           *
*  -------------------------                                           *
*                                                                      *
*  Covered Software is provided under this License on an "as is"       *
*  basis, without warranty of any kind, either expressed, implied, or  *
*  statutory, including, without limitation, warranties that the       *
*  Covered Software is free of defects, merchantable, fit for a        *
*  particular purpose or non-infringing. The entire risk as to the     *
*  quality and performance of the Covered Software is with You.        *
*  Should any Covered Software prove defective in any respect, You     *
*  (not any Contributor) assume the cost of any necessary servicing,   *
*  repair, or correction. This disclaimer of warranty constitutes an   *
*  essential part of this License. No use of any Covered Software is   *
*  authorized under this License except under this disclaimer.         *
*                                                                      *
************************************************************************

************************************************************************
*                                                                      *
*  7. Limitation of Liability                                          *
*  --------------------------                                          *
*                                                                      *
*  Under no circumstances and under no legal theory, whether tort      *
*  (including negligence), contract, or otherwise, shall any           *
*  Contributor, or anyone who distributes Covered Software as          *
*  permitted above, be liable to You for any direct, indirect,         *
*  special, incidental, or consequential damages of any character      *
*  including, without limitation, damages for lost profits, loss of    *
*  goodwill, work stoppage, computer failure or malfunction, or any    *
*  and all other commercial damages or losses, even if such party      *
*  shall have been informed of the possibility of such damages. This   *
*  limitation of liability shall not apply to liability for death or   *
*  personal injury resulting from such party's negligence to the       *
*  extent applicable law prohibits such limitation. Some               *
*  jurisdictions do not allow the exclusion or limitation of           *
*  incidental or consequential damages, so this exclusion and          *
*  limitation may not apply to You.                                    *
*                                                                      *
************************************************************************

8. Litigation
-------------

Any litigation relating to this License may be brought only in the
courts of a jurisdiction where the defendant maintains its principal
place of business and such litigation shall be governed by laws of that
jurisdiction, without reference to its conflict-of-law provisions.
Nothing in this Section shall prevent a party's ability to bring
cross-claims or counter-claims.

9. Miscellaneous
----------------

This License represents the complete agreement concerning the subject
matter hereof. If any provision of this License is held to be
unenforceable, such provision shall be reformed only to the extent
necessary to make it enforceable. Any law or regulation which provides
that the language of a contract shall be construed against the drafter
shall not be used to construe this License against a Contributor.

10. Versions of the License
---------------------------

10.1. New Versions

Mozilla Foundation is the license steward. Except as provided in Section
10.3, no one other than the license steward has the right to modify or
publish new versions of this License. Each version will be given a
distinguishing version number.

10.2. Effect of New Versions

You may distribute the Covered Software under the terms of the version
of the License under which You originally received the Covered Software,
or under the terms of any subsequent version published by the license
steward.

10.3. Modified Versions

If you create software not governed by this License, and you want to
create a new license for such software, you may create and use a
modified version of this License if you rename the license and remove
any references to the name of the license steward (except to note that
such modified license differs from this License).

10.4. Distributing Source Code Form that is Incompatible With Secondary
Licenses

If You choose to distribute Source Code Form that is Incompatible With
Secondary Licenses under the terms of this version of the License, the
notice described in Exhibit B of this License must be attached.

Exhibit A - Source Code Form License Notice
-------------------------------------------

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at https://mozilla.org/MPL/2.0/.

If it is not possible or desirable to put the notice in a particular
file, then You may include the notice in a location (such as a LICENSE
file in a relevant directory) where a recipient would be likely to look
for such a notice.

You may add additional accurate notices of copyright ownership.

Exhibit B - "Incompatible With Secondary Licenses" Notice
---------------------------------------------------------

  This Source Code Form is "Incompatible With Secondary Licenses", as
  defined by the Mozilla Public License, v. 2.0.
```

### 4.4 GNU Lesser General Public License, Version 2.1 (LGPL-2.1)
```text
GNU LESSER GENERAL PUBLIC LICENSE
		       Version 2.1, February 1999

 Copyright (C) 1991, 1999 Free Software Foundation, Inc.
     51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.

[This is the first released version of the Lesser GPL.  It also counts
 as the successor of the GNU Library Public License, version 2, hence
 the version number 2.1.]

			    Preamble

  The licenses for most software are designed to take away your
freedom to share and change it.  By contrast, the GNU General Public
Licenses are intended to guarantee your freedom to share and change
free software--to make sure the software is free for all its users.

  This license, the Lesser General Public License, applies to some
specially designated software packages--typically libraries--of the
Free Software Foundation and other authors who decide to use it.  You
can use it too, but we suggest you first think carefully about whether
this license or the ordinary General Public License is the better
strategy to use in any particular case, based on the explanations below.

  When we speak of free software, we are referring to freedom of use,
not price.  Our General Public Licenses are designed to make sure that
you have the freedom to distribute copies of free software (and charge
for this service if you wish); that you receive source code or can get
it if you want it; that you can change the software and use pieces of
it in new free programs; and that you are informed that you can do
these things.

  To protect your rights, we need to make restrictions that forbid
distributors to deny you these rights or to ask you to surrender these
rights.  These restrictions translate to certain responsibilities for
you if you distribute copies of the library or if you modify it.

  For example, if you distribute copies of the library, whether gratis
or for a fee, you must give the recipients all the rights that we gave
you.  You must make sure that they, too, receive or can get the source
code.  If you link other code with the library, you must provide
complete object files to the recipients, so that they can relink them
with the library after making changes to the library and recompiling
it.  And you must show them these terms so they know their rights.

  We protect your rights with a two-step method: (1) we copyright the
library, and (2) we offer you this license, which gives you legal
permission to copy, distribute and/or modify the library.

  To protect each distributor, we want to make it very clear that
there is no warranty for the free library.  Also, if the library is
modified by someone else and passed on, the recipients should know
that what they have is not the original version, so that the original
author's reputation will not be affected by problems that might be
introduced by others.

  Finally, software patents pose a constant threat to the existence of
any free program.  We wish to make sure that a company cannot
effectively restrict the users of a free program by obtaining a
restrictive license from a patent holder.  Therefore, we insist that
any patent license obtained for a version of the library must be
consistent with the full freedom of use specified in this license.

  Most GNU software, including some libraries, is covered by the
ordinary GNU General Public License.  This license, the GNU Lesser
General Public License, applies to certain designated libraries, and
is quite different from the ordinary General Public License.  We use
this license for certain libraries in order to permit linking those
libraries into non-free programs.

  When a program is linked with a library, whether statically or using
a shared library, the combination of the two is legally speaking a
combined work, a derivative of the original library.  The ordinary
General Public License therefore permits such linking only if the
entire combination fits its criteria of freedom.  The Lesser General
Public License permits more lax criteria for linking other code with
the library.

  We call this license the "Lesser" General Public License because it
does Less to protect the user's freedom than the ordinary General
Public License.  It also provides other free software developers Less
of an advantage over competing non-free programs.  These disadvantages
are the reason we use the ordinary General Public License for many
libraries.  However, the Lesser license provides advantages in certain
special circumstances.

  For example, on rare occasions, there may be a special need to
encourage the widest possible use of a certain library, so that it becomes
a de-facto standard.  To achieve this, non-free programs must be
allowed to use the library.  A more frequent case is that a free
library does the same job as widely used non-free libraries.  In this
case, there is little to gain by limiting the free library to free
software only, so we use the Lesser General Public License.

  In other cases, permission to use a particular library in non-free
programs enables a greater number of people to use a large body of
free software.  For example, permission to use the GNU C Library in
non-free programs enables many more people to use the whole GNU
operating system, as well as its variant, the GNU/Linux operating
system.

  Although the Lesser General Public License is Less protective of the
users' freedom, it does ensure that the user of a program that is
linked with the Library has the freedom and the wherewithal to run
that program using a modified version of the Library.

  The precise terms and conditions for copying, distribution and
modification follow.  Pay close attention to the difference between a
"work based on the library" and a "work that uses the library".  The
former contains code derived from the library, whereas the latter must
be combined with the library in order to run.

		  GNU LESSER GENERAL PUBLIC LICENSE
   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

  0. This License Agreement applies to any software library or other
program which contains a notice placed by the copyright holder or
other authorized party saying it may be distributed under the terms of
this Lesser General Public License (also called "this License").
Each licensee is addressed as "you".

  A "library" means a collection of software functions and/or data
prepared so as to be conveniently linked with application programs
(which use some of those functions and data) to form executables.

  The "Library", below, refers to any such software library or work
which has been distributed under these terms.  A "work based on the
Library" means either the Library or any derivative work under
copyright law: that is to say, a work containing the Library or a
portion of it, either verbatim or with modifications and/or translated
straightforwardly into another language.  (Hereinafter, translation is
included without limitation in the term "modification".)

  "Source code" for a work means the preferred form of the work for
making modifications to it.  For a library, complete source code means
all the source code for all modules it contains, plus any associated
interface definition files, plus the scripts used to control compilation
and installation of the library.

  Activities other than copying, distribution and modification are not
covered by this License; they are outside its scope.  The act of
running a program using the Library is not restricted, and output from
such a program is covered only if its contents constitute a work based
on the Library (independent of the use of the Library in a tool for
writing it).  Whether that is true depends on what the Library does
and what the program that uses the Library does.
  
  1. You may copy and distribute verbatim copies of the Library's
complete source code as you receive it, in any medium, provided that
you conspicuously and appropriately publish on each copy an
appropriate copyright notice and disclaimer of warranty; keep intact
all the notices that refer to this License and to the absence of any
warranty; and distribute a copy of this License along with the
Library.

  You may charge a fee for the physical act of transferring a copy,
and you may at your option offer warranty protection in exchange for a
fee.

  2. You may modify your copy or copies of the Library or any portion
of it, thus forming a work based on the Library, and copy and
distribute such modifications or work under the terms of Section 1
above, provided that you also meet all of these conditions:

    a) The modified work must itself be a software library.

    b) You must cause the files modified to carry prominent notices
    stating that you changed the files and the date of any change.

    c) You must cause the whole of the work to be licensed at no
    charge to all third parties under the terms of this License.

    d) If a facility in the modified Library refers to a function or a
    table of data to be supplied by an application program that uses
    the facility, other than as an argument passed when the facility
    is invoked, then you must make a good faith effort to ensure that,
    in the event an application does not supply such function or
    table, the facility still operates, and performs whatever part of
    its purpose remains meaningful.

    (For example, a function in a library to compute square roots has
    a purpose that is entirely well-defined independent of the
    application.  Therefore, Subsection 2d requires that any
    application-supplied function or table used by this function must
    be optional: if the application does not supply it, the square
    root function must still compute square roots.)

These requirements apply to the modified work as a whole.  If
identifiable sections of that work are not derived from the Library,
and can be reasonably considered independent and separate works in
themselves, then this License, and its terms, do not apply to those
sections when you distribute them as separate works.  But when you
distribute the same sections as part of a whole which is a work based
on the Library, the distribution of the whole must be on the terms of
this License, whose permissions for other licensees extend to the
entire whole, and thus to each and every part regardless of who wrote
it.

Thus, it is not the intent of this section to claim rights or contest
your rights to work written entirely by you; rather, the intent is to
exercise the right to control the distribution of derivative or
collective works based on the Library.

In addition, mere aggregation of another work not based on the Library
with the Library (or with a work based on the Library) on a volume of
a storage or distribution medium does not bring the other work under
the scope of this License.

  3. You may opt to apply the terms of the ordinary GNU General Public
License instead of this License to a given copy of the Library.  To do
this, you must alter all the notices that refer to this License, so
that they refer to the ordinary GNU General Public License, version 2,
instead of to this License.  (If a newer version than version 2 of the
ordinary GNU General Public License has appeared, then you can specify
that version instead if you wish.)  Do not make any other change in
these notices.

  Once this change is made in a given copy, it is irreversible for
that copy, so the ordinary GNU General Public License applies to all
subsequent copies and derivative works made from that copy.

  This option is useful when you wish to copy part of the code of
the Library into a program that is not a library.

  4. You may copy and distribute the Library (or a portion or
derivative of it, under Section 2) in object code or executable form
under the terms of Sections 1 and 2 above provided that you accompany
it with the complete corresponding machine-readable source code, which
must be distributed under the terms of Sections 1 and 2 above on a
medium customarily used for software interchange.

  If distribution of object code is made by offering access to copy
from a designated place, then offering equivalent access to copy the
source code from the same place satisfies the requirement to
distribute the source code, even though third parties are not
compelled to copy the source along with the object code.

  5. A program that contains no derivative of any portion of the
Library, but is designed to work with the Library by being compiled or
linked with it, is called a "work that uses the Library".  Such a
work, in isolation, is not a derivative work of the Library, and
therefore falls outside the scope of this License.

  However, linking a "work that uses the Library" with the Library
creates an executable that is a derivative of the Library (because it
contains portions of the Library), rather than a "work that uses the
library".  The executable is therefore covered by this License.
Section 6 states terms for distribution of such executables.

  When a "work that uses the Library" uses material from a header file
that is part of the Library, the object code for the work may be a
derivative work of the Library even though the source code is not.
Whether this is true is especially significant if the work can be
linked without the Library, or if the work is itself a library.  The
threshold for this to be true is not precisely defined by law.

  If such an object file uses only numerical parameters, data
structure layouts and accessors, and small macros and small inline
functions (ten lines or less in length), then the use of the object
file is unrestricted, regardless of whether it is legally a derivative
work.  (Executables containing this object code plus portions of the
Library will still fall under Section 6.)

  Otherwise, if the work is a derivative of the Library, you may
distribute the object code for the work under the terms of Section 6.
Any executables containing that work also fall under Section 6,
whether or not they are linked directly with the Library itself.

  6. As an exception to the Sections above, you may also combine or
link a "work that uses the Library" with the Library to produce a
work containing portions of the Library, and distribute that work
under terms of your choice, provided that the terms permit
modification of the work for the customer's own use and reverse
engineering for debugging such modifications.

  You must give prominent notice with each copy of the work that the
Library is used in it and that the Library and its use are covered by
this License.  You must supply a copy of this License.  If the work
during execution displays copyright notices, you must include the
copyright notice for the Library among them, as well as a reference
directing the user to the copy of this License.  Also, you must do one
of these things:

    a) Accompany the work with the complete corresponding
    machine-readable source code for the Library including whatever
    changes were used in the work (which must be distributed under
    Sections 1 and 2 above); and, if the work is an executable linked
    with the Library, with the complete machine-readable "work that
    uses the Library", as object code and/or source code, so that the
    user can modify the Library and then relink to produce a modified
    executable containing the modified Library.  (It is understood
    that the user who changes the contents of definitions files in the
    Library will not necessarily be able to recompile the application
    to use the modified definitions.)

    b) Use a suitable shared library mechanism for linking with the
    Library.  A suitable mechanism is one that (1) uses at run time a
    copy of the library already present on the user's computer system,
    rather than copying library functions into the executable, and (2)
    will operate properly with a modified version of the library, if
    the user installs one, as long as the modified version is
    interface-compatible with the version that the work was made with.

    c) Accompany the work with a written offer, valid for at
    least three years, to give the same user the materials
    specified in Subsection 6a, above, for a charge no more
    than the cost of performing this distribution.

    d) If distribution of the work is made by offering access to copy
    from a designated place, offer equivalent access to copy the above
    specified materials from the same place.

    e) Verify that the user has already received a copy of these
    materials or that you have already sent this user a copy.

  For an executable, the required form of the "work that uses the
Library" must include any data and utility programs needed for
reproducing the executable from it.  However, as a special exception,
the materials to be distributed need not include anything that is
normally distributed (in either source or binary form) with the major
components (compiler, kernel, and so on) of the operating system on
which the executable runs, unless that component itself accompanies
the executable.

  It may happen that this requirement contradicts the license
restrictions of other proprietary libraries that do not normally
accompany the operating system.  Such a contradiction means you cannot
use both them and the Library together in an executable that you
distribute.

  7. You may place library facilities that are a work based on the
Library side-by-side in a single library together with other library
facilities not covered by this License, and distribute such a combined
library, provided that the separate distribution of the work based on
the Library and of the other library facilities is otherwise
permitted, and provided that you do these two things:

    a) Accompany the combined library with a copy of the same work
    based on the Library, uncombined with any other library
    facilities.  This must be distributed under the terms of the
    Sections above.

    b) Give prominent notice with the combined library of the fact
    that part of it is a work based on the Library, and explaining
    where to find the accompanying uncombined form of the same work.

  8. You may not copy, modify, sublicense, link with, or distribute
the Library except as expressly provided under this License.  Any
attempt otherwise to copy, modify, sublicense, link with, or
distribute the Library is void, and will automatically terminate your
rights under this License.  However, parties who have received copies,
or rights, from you under this License will not have their licenses
terminated so long as such parties remain in full compliance.

  9. You are not required to accept this License, since you have not
signed it.  However, nothing else grants you permission to modify or
distribute the Library or its derivative works.  These actions are
prohibited by law if you do not accept this License.  Therefore, by
modifying or distributing the Library (or any work based on the
Library), you indicate your acceptance of this License to do so, and
all its terms and conditions for copying, distributing or modifying
the Library or works based on it.

  10. Each time you redistribute the Library (or any work based on the
Library), the recipient automatically receives a license from the
original licensor to copy, distribute, link with or modify the Library
subject to these terms and conditions.  You may not impose any further
restrictions on the recipients' exercise of the rights granted herein.
You are not responsible for enforcing compliance by third parties with
this License.

  11. If, as a consequence of a court judgment or allegation of patent
infringement or for any other reason (not limited to patent issues),
conditions are imposed on you (whether by court order, agreement or
otherwise) that contradict the conditions of this License, they do not
excuse you from the conditions of this License.  If you cannot
distribute so as to satisfy simultaneously your obligations under this
License and any other pertinent obligations, then as a consequence you
may not distribute the Library at all.  For example, if a patent
license would not permit royalty-free redistribution of the Library by
all those who receive copies directly or indirectly through you, then
the only way you could satisfy both it and this License would be to
refrain entirely from distribution of the Library.

If any portion of this section is held invalid or unenforceable under any
particular circumstance, the balance of the section is intended to apply,
and the section as a whole is intended to apply in other circumstances.

It is not the purpose of this section to induce you to infringe any
patents or other property right claims or to contest validity of any
such claims; this section has the sole purpose of protecting the
integrity of the free software distribution system which is
implemented by public license practices.  Many people have made
generous contributions to the wide range of software distributed
through that system in reliance on consistent application of that
system; it is up to the author/donor to decide if he or she is willing
to distribute software through any other system and a licensee cannot
impose that choice.

This section is intended to make thoroughly clear what is believed to
be a consequence of the rest of this License.

  12. If the distribution and/or use of the Library is restricted in
certain countries either by patents or by copyrighted interfaces, the
original copyright holder who places the Library under this License may add
an explicit geographical distribution limitation excluding those countries,
so that distribution is permitted only in or among countries not thus
excluded.  In such case, this License incorporates the limitation as if
written in the body of this License.

  13. The Free Software Foundation may publish revised and/or new
versions of the Lesser General Public License from time to time.
Such new versions will be similar in spirit to the present version,
but may differ in detail to address new problems or concerns.

Each version is given a distinguishing version number.  If the Library
specifies a version number of this License which applies to it and
"any later version", you have the option of following the terms and
conditions either of that version or of any later version published by
the Free Software Foundation.  If the Library does not specify a
license version number, you may choose any version ever published by
the Free Software Foundation.

  14. If you wish to incorporate parts of the Library into other free
programs whose distribution conditions are incompatible with these,
write to the author to ask for permission.  For software which is
copyrighted by the Free Software Foundation, write to the Free
Software Foundation; we sometimes make exceptions for this.  Our
decision will be guided by the two goals of preserving the free status
of all derivatives of our free software and of promoting the sharing
and reuse of software generally.

			    NO WARRANTY

  15. BECAUSE THE LIBRARY IS LICENSED FREE OF CHARGE, THERE IS NO
WARRANTY FOR THE LIBRARY, TO THE EXTENT PERMITTED BY APPLICABLE LAW.
EXCEPT WHEN OTHERWISE STATED IN WRITING THE COPYRIGHT HOLDERS AND/OR
OTHER PARTIES PROVIDE THE LIBRARY "AS IS" WITHOUT WARRANTY OF ANY
KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
PURPOSE.  THE ENTIRE RISK AS TO THE QUALITY AND PERFORMANCE OF THE
LIBRARY IS WITH YOU.  SHOULD THE LIBRARY PROVE DEFECTIVE, YOU ASSUME
THE COST OF ALL NECESSARY SERVICING, REPAIR OR CORRECTION.

  16. IN NO EVENT UNLESS REQUIRED BY APPLICABLE LAW OR AGREED TO IN
WRITING WILL ANY COPYRIGHT HOLDER, OR ANY OTHER PARTY WHO MAY MODIFY
AND/OR REDISTRIBUTE THE LIBRARY AS PERMITTED ABOVE, BE LIABLE TO YOU
FOR DAMAGES, INCLUDING ANY GENERAL, SPECIAL, INCIDENTAL OR
CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OR INABILITY TO USE THE
LIBRARY (INCLUDING BUT NOT LIMITED TO LOSS OF DATA OR DATA BEING
RENDERED INACCURATE OR LOSSES SUSTAINED BY YOU OR THIRD PARTIES OR A
FAILURE OF THE LIBRARY TO OPERATE WITH ANY OTHER SOFTWARE), EVEN IF
SUCH HOLDER OR OTHER PARTY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
DAMAGES.

		     END OF TERMS AND CONDITIONS

           How to Apply These Terms to Your New Libraries

  If you develop a new library, and you want it to be of the greatest
possible use to the public, we recommend making it free software that
everyone can redistribute and change.  You can do so by permitting
redistribution under these terms (or, alternatively, under the terms of the
ordinary General Public License).

  To apply these terms, attach the following notices to the library.  It is
safest to attach them to the start of each source file to most effectively
convey the exclusion of warranty; and each file should have at least the
"copyright" line and a pointer to where the full notice is found.

    <one line to give the library's name and a brief idea of what it does.>
    Copyright (C) <year>  <name of author>

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public
    License along with this library; if not, write to the Free Software
    Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

Also add information on how to contact you by electronic and paper mail.

You should also get your employer (if you work as a programmer) or your
school, if any, to sign a "copyright disclaimer" for the library, if
necessary.  Here is a sample; alter the names:

  Yoyodyne, Inc., hereby disclaims all copyright interest in the
  library `Frob' (a library for tweaking knobs) written by James Random Hacker.

  <signature of Ty Coon>, 1 April 1990
  Ty Coon, President of Vice

That's all there is to it!
```

### 4.5 The Unlicense
```text
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
```

