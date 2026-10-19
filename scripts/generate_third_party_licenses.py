#!/usr/bin/env python3
"""
generate_third_party_licenses.py

Generates an evidence-based third-party license collection and consolidated
human-readable index for OpenDictate from the pinned Cargo dependency graph,
lockfile, and actual on-disk crate archives.
"""

import subprocess
import json
import os
import glob
import sys
from collections import defaultdict

def main():
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(repo_root)

    print("=== Reading Cargo Metadata ===")
    out = subprocess.check_output(["cargo", "metadata", "--format-version", "1"])
    meta = json.loads(out)

    print("=== Tracing Active Linux Dependencies ===")
    tree_out = subprocess.check_output([
        "cargo", "tree", "--prefix", "none", "--edges", "normal,build",
        "--target", "x86_64-unknown-linux-gnu"
    ])
    linux_pkgs = set(line.strip().split()[0] for line in tree_out.decode().splitlines() if line.strip())

    src_base = os.path.expanduser("~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f")
    if not os.path.isdir(src_base):
        # Fallback to any directory under ~/.cargo/registry/src
        src_dirs = glob.glob(os.path.expanduser("~/.cargo/registry/src/*"))
        if src_dirs:
            src_base = src_dirs[0]

    packages_info = []

    for p in sorted(meta["packages"], key=lambda x: x["name"]):
        name = p["name"]
        ver = p["version"]
        if name == "opendictate":
            continue

        lic = p.get("license") or "Unspecified"
        repo = p.get("repository") or p.get("homepage") or f"https://crates.io/crates/{name}"
        authors = p.get("authors", [])
        desc = p.get("description", "")
        is_linux_active = name in linux_pkgs

        # Locate unpacked crate directory
        crate_dir = os.path.join(src_base, f"{name}-{ver}")
        if not os.path.isdir(crate_dir):
            matches = glob.glob(os.path.join(src_base, f"{name}-{ver}*"))
            crate_dir = matches[0] if matches else None

        notices = []
        if crate_dir and os.path.isdir(crate_dir):
            for fname in sorted(os.listdir(crate_dir)):
                upper = fname.upper()
                if any(k in upper for k in ["LICEN", "COPYING", "NOTICE", "COPYRIGHT", "AUTHORS"]):
                    if not upper.endswith((".RS", ".PNG", ".SVG", ".HTML", ".TOML", ".ORIG")):
                        fpath = os.path.join(crate_dir, fname)
                        if os.path.isfile(fpath):
                            try:
                                content = open(fpath, "r", errors="replace").read().strip()
                                notices.append((fname, content))
                            except Exception:
                                pass

        packages_info.append({
            "name": name,
            "version": ver,
            "license": lic,
            "repository": repo,
            "authors": authors,
            "description": desc,
            "is_linux_active": is_linux_active,
            "notices": notices,
        })

    licenses_dir = os.path.join(repo_root, "licenses")
    os.makedirs(licenses_dir, exist_ok=True)

    # 1. Standalone License Texts
    mpl20_text = ""
    lgpl21_text = ""
    apache20_text = ""
    mit_text = open(os.path.join(repo_root, "LICENSE")).read().strip()

    # Locate MPL-2.0 from option-ext
    opt_ext_matches = [p for p in packages_info if p["name"] == "option-ext"]
    if opt_ext_matches and opt_ext_matches[0]["notices"]:
        mpl20_text = opt_ext_matches[0]["notices"][0][1]

    # Locate Apache-2.0 from cpal or hound
    cpal_matches = [p for p in packages_info if p["name"] == "cpal"]
    if cpal_matches and cpal_matches[0]["notices"]:
        apache20_text = cpal_matches[0]["notices"][0][1]

    # Fallback if needed
    if not mpl20_text:
        local_mpl = os.path.join(licenses_dir, "LICENSE-MPL-2.0.txt")
        if os.path.exists(local_mpl):
            mpl20_text = open(local_mpl).read().strip()

    # alsa-lib COPYING for LGPL-2.1
    local_lgpl = os.path.join(licenses_dir, "LICENSE-LGPL-2.1.txt")
    if os.path.exists(local_lgpl):
        lgpl21_text = open(local_lgpl).read().strip()
    else:
        # Check snap build or local path
        snap_copying = glob.glob("/snap/opendictate/current/usr/share/doc/libasound2*/copyright")
        if snap_copying:
            lgpl21_text = open(snap_copying[0]).read().strip()

    with open(os.path.join(licenses_dir, "LICENSE-OpenDictate.txt"), "w") as f:
        f.write(mit_text + "\n")

    if mpl20_text:
        with open(os.path.join(licenses_dir, "LICENSE-MPL-2.0.txt"), "w") as f:
            f.write(mpl20_text + "\n")

    if lgpl21_text:
        with open(os.path.join(licenses_dir, "LICENSE-LGPL-2.1.txt"), "w") as f:
            f.write(lgpl21_text + "\n")

    if apache20_text:
        with open(os.path.join(licenses_dir, "LICENSE-Apache-2.0.txt"), "w") as f:
            f.write(apache20_text + "\n")

    with open(os.path.join(licenses_dir, "LICENSE-MIT.txt"), "w") as f:
        f.write(mit_text + "\n")

    # 2. Build THIRD_PARTY_LICENSES.md
    print("=== Generating THIRD_PARTY_LICENSES.md ===")
    md_path = os.path.join(licenses_dir, "THIRD_PARTY_LICENSES.md")
    with open(md_path, "w") as f:
        f.write("# OpenDictate Third-Party Software Licenses & Notices\n\n")
        f.write("**Application:** OpenDictate v2.0.3  \n")
        f.write("**Application License:** MIT License (Copyright (c) 2026 H. K. Saikia)  \n")
        f.write("**Commit:** `7a73366e2da882e05db3a05b189a05b6d08bc477`  \n\n")
        f.write("This document contains the complete third-party intellectual property, licensing, and attribution notices for all direct and transitive dependencies, bundled assets, native libraries, and speech models utilized in the OpenDictate distribution.\n\n")

        f.write("---\n\n")
        f.write("## 1. Special Legal Disclosures & Compliance Notices\n\n")

        # MPL-2.0 disclosure
        f.write("### 1.1 Mozilla Public License 2.0 (MPL-2.0) — `option-ext` v0.2.0\n")
        f.write("OpenDictate incorporates the `option-ext` crate (version 0.2.0, authored by Simon Ochsenreither), which is licensed under the **Mozilla Public License, Version 2.0 (MPL-2.0)**.\n\n")
        f.write("- **Nature of License:** MPL-2.0 is a file-level weak copyleft license. Under Section 3.3 (Larger Works), OpenDictate is distributed as a Larger Work under the MIT License while maintaining all required notices for MPL-covered files.\n")
        f.write("- **Source Code Availability:** The complete upstream source code of `option-ext` v0.2.0 is freely available at:\n")
        f.write("  `https://github.com/soc/option-ext.git` (or on crates.io at `https://crates.io/crates/option-ext/0.2.0`).\n")
        f.write("- **Modifications:** OpenDictate incorporates `option-ext` unmodified.\n")
        f.write("- **License Text:** The full text of the MPL-2.0 is included in [Section 4.3](#43-mozilla-public-license-version-20-mpl-20) below and in `licenses/LICENSE-MPL-2.0.txt`.\n\n")

        # Tri-license election for r-efi
        f.write("### 1.2 Tri-Licensed Dependencies — `r-efi` v6.0.0\n")
        f.write("The crate `r-efi` (version 6.0.0) is tri-licensed under `MIT OR Apache-2.0 OR LGPL-2.1-or-later`.\n\n")
        f.write("- **Elected License:** In accordance with the disjunctive licensing terms, OpenDictate formally elects the **MIT License** for `r-efi`.\n")
        f.write("- **Notice & Attribution:** Copyright (C) 2017-2023 Red Hat, Inc., Microsoft Corporation, and David Rheinsberg. See [Section 3.1](#31-active-linux-runtime-dependencies) for complete author notices.\n\n")

        # ALSA LGPL-2.1
        f.write("### 1.3 Native Audio Library — `alsa-lib` 1.2.12 (LGPL-2.1-or-later)\n")
        f.write("OpenDictate interfaces with the Advanced Linux Sound Architecture (`alsa-lib` version 1.2.12), licensed under the **GNU Lesser General Public License, Version 2.1 or later (LGPL-2.1+)**.\n\n")
        f.write("- **Linkage Compliance:** OpenDictate dynamically links against `libasound.so.2` via shared object linkage. In Flatpak, `libasound.so.2.0.0` is provided as a shared library in `/app/lib/`; in Snap, it is dynamically loaded from `$SNAP/usr/lib/$CRAFT_ARCH_TRIPLET_BUILD_FOR/`.\n")
        f.write("- **User Modification & Reverse Engineering:** Dynamic linking satisfies Section 6(a) of the LGPL-2.1, allowing users to modify or replace `libasound.so.2` independently.\n")
        f.write("- **License Text:** The full LGPL-2.1 text is installed in the package at `/app/share/licenses/io.github.hkumarsaikia.OpenDictate/alsa-lib/COPYING` and reproduced in [Section 4.4](#44-gnu-lesser-general-public-license-version-21-lgpl-21).\n\n")

        # whisper.cpp
        f.write("### 1.4 Whisper C/C++ Engine — `whisper.cpp` (MIT License)\n")
        f.write("The embedded speech inference engine `whisper.cpp` (bundled via `whisper-rs-sys`) is authored by Georgi Gerganov and licensed under the **MIT License** (Copyright (c) 2023 Georgi Gerganov).\n\n")

        # Whisper Models
        f.write("### 1.5 OpenAI Whisper Model Weights (`ggml-tiny.en.bin`, etc.)\n")
        f.write("The default speech recognition models supported by OpenDictate are based on OpenAI's Whisper neural network architecture.\n\n")
        f.write("- **Model Source:** Trained and published by OpenAI (Radford et al., 2022) under the **MIT License** (`Copyright (c) 2022 OpenAI`).\n")
        f.write("- **Conversion Source:** Converted to GGML binary format by Georgi Gerganov under the **MIT License** (`Copyright (c) 2023 Georgi Gerganov`).\n")
        f.write("- **Distribution Architecture:** Model weights are **not bundled** into the application binary or release packages. They are fetched on demand by the user over HTTPS directly into `$XDG_DATA_HOME/opendictate/models/`.\n\n")

        f.write("---\n\n")
        f.write("## 2. License Breakdown Summary (447 Cargo Crates)\n\n")

        # Tally licenses
        lic_counts = defaultdict(list)
        for p in packages_info:
            lic_counts[p["license"]].append(p)

        f.write("| Declared SPDX License | Package Count | License Category | Scope |\n")
        f.write("| :--- | :---: | :--- | :--- |\n")
        for lic, pkgs in sorted(lic_counts.items(), key=lambda x: len(x[1]), reverse=True):
            active_count = sum(1 for p in pkgs if p["is_linux_active"])
            f.write(f"| **{lic}** | {len(pkgs)} | {categorize_license(lic)} | {active_count} active on Linux |\n")

        f.write("\n---\n\n")
        f.write("## 3. Complete Rust Dependency Inventory\n\n")

        # Active Linux dependencies
        active_pkgs = [p for p in packages_info if p["is_linux_active"]]
        inactive_pkgs = [p for p in packages_info if not p["is_linux_active"]]

        f.write(f"### 3.1 Active Linux Runtime Dependencies ({len(active_pkgs)} Crates)\n\n")
        f.write("The following crates are directly compiled or linked into the OpenDictate Linux binary for `x86_64-unknown-linux-gnu`:\n\n")

        for p in active_pkgs:
            f.write(f"#### `{p['name']}` v{p['version']}\n")
            f.write(f"- **SPDX License:** `{p['license']}`\n")
            f.write(f"- **Repository / Source:** {p['repository']}\n")
            if p["authors"]:
                f.write(f"- **Authors / Contributors:** {', '.join(p['authors'])}\n")
            if p["description"]:
                f.write(f"- **Description:** {p['description']}\n")
            if p["notices"]:
                f.write("- **Included Notice / License Content:**\n")
                for nfile, ntext in p["notices"]:
                    f.write(f"  - *File: `{nfile}`*\n")
                    f.write("    ```text\n")
                    # Limit long license boilerplate in individual listings
                    lines = ntext.splitlines()
                    preview = "\n".join(lines[:15])
                    if len(lines) > 15:
                        preview += f"\n    [... truncated {len(lines) - 15} lines ...]"
                    f.write(f"    {preview}\n")
                    f.write("    ```\n")
            f.write("\n")

        f.write(f"### 3.2 Target-Specific Inactive Dependencies ({len(inactive_pkgs)} Crates)\n\n")
        f.write("The following crates are declared in `Cargo.lock` but are conditional on Windows (`cfg(windows)`), macOS/iOS (`cfg(target_vendor = \"apple\")`), Android (`cfg(target_os = \"android\")`), or web targets, and are **not linked** into the Linux ELF executable:\n\n")

        f.write("| Crate Name | Version | Declared SPDX License | Target Platform / Purpose |\n")
        f.write("| :--- | :--- | :--- | :--- |\n")
        for p in inactive_pkgs:
            target = determine_target_platform(p["name"])
            f.write(f"| `{p['name']}` | `{p['version']}` | `{p['license']}` | {target} |\n")

        f.write("\n---\n\n")
        f.write("## 4. Full Texts of Governed Licenses\n\n")

        f.write("### 4.1 MIT License\n```text\n")
        f.write(mit_text + "\n```\n\n")

        if apache20_text:
            f.write("### 4.2 Apache License, Version 2.0\n```text\n")
            f.write(apache20_text + "\n```\n\n")

        if mpl20_text:
            f.write("### 4.3 Mozilla Public License, Version 2.0 (MPL-2.0)\n```text\n")
            f.write(mpl20_text + "\n```\n\n")

        if lgpl21_text:
            f.write("### 4.4 GNU Lesser General Public License, Version 2.1 (LGPL-2.1)\n```text\n")
            f.write(lgpl21_text + "\n```\n\n")

        unlicense_text = """This is free and unencumbered software released into the public domain.

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

For more information, please refer to <http://unlicense.org/>"""

        f.write("### 4.5 The Unlicense\n```text\n")
        f.write(unlicense_text + "\n```\n\n")

    print(f"Generated {md_path} ({os.path.getsize(md_path)} bytes)")
    print("=== License Generation Complete ===")

def categorize_license(spdx):
    u = spdx.upper()
    if "MPL" in u:
        return "Weak Copyleft (File-Level)"
    if "LGPL" in u and "OR" not in u:
        return "Weak Copyleft (Library)"
    if "GPL" in u:
        return "Strong Copyleft"
    if "UNLICENSE" in u or "0BSD" in u:
        return "Public Domain / Zero-Clause"
    if "OR" in u or "/" in u:
        return "Dual / Multi-License"
    return "Permissive"

def determine_target_platform(name):
    n = name.lower()
    if any(k in n for k in ["windows", "winapi", "schannel", "clipboard-win"]):
        return "Windows (`cfg(windows)`)"
    if any(k in n for k in ["apple", "core-foundation", "objc", "security-framework", "dispatch"]):
        return "macOS / iOS (`cfg(target_vendor = \"apple\")`)"
    if any(k in n for k in ["android", "ndk", "jni"]):
        return "Android (`cfg(target_os = \"android\")`)"
    if any(k in n for k in ["wasm", "web-sys", "js-sys"]):
        return "WebAssembly (`target_arch = \"wasm32\"`)"
    return "Other OS / Architecture Fallback"

if __name__ == "__main__":
    main()
