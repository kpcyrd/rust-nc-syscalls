#!/usr/bin/env python3.9
# Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by Apache-2.0 License that can be found
# in the LICENSE file.

import os
import re
import subprocess
import sys


HEADER_FILE = "/usr/include/sys/errno.h"


def read_errno(header_file):
    errors = []
    with open(header_file) as fh:
        errors.extend(parse_errno(fh.read()))

    lines = ["""
    // Code generated by mkerrno_netbsd.py; DO NOT EDIT.

    use crate::syscalls::Errno;
    """
    ]

    for errno in errors:
        if errno[0] == "ELAST":
            continue
        if len(errno) > 2:
            lines.append("/// {0}".format(errno[2]))
        lines.append("pub const {0}: Errno = {1};".format(errno[0], errno[1]))

    lines.append("""
    /// Get errno description.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub const fn strerror(errno: Errno) -> &'static str {
        match errno {
    """)

    for errno in errors:
        if errno[0] == "ELAST":
            continue
        if type(errno[1]) is int:
            lines.append('{0} => "{1}",'.format(errno[0], errno[2]))

    lines.append("""
        _ => "Unknown errno!",
        }
    }
    """)
    return lines

def parse_errno(content):
    # For `define    EKEYEXPIRED     127     /* Key has expired */`
    errno_pattern = re.compile("^#define\s+(E\w+)\s+(\d+)\s+/\\*([^\\*]+)\\*/")
    # For `define   EDEADLOCK       EDEADLK`
    alias_pattern = re.compile("^#define\s+(E\w+)\s+(E\w+)")
    errors = []
    for line in content.split('\n'):
        m = errno_pattern.match(line)
        if m:
            errors.append((m.group(1), int(m.group(2)), m.group(3).strip()))
        else:
            m = alias_pattern.match(line)
            if m:
                errors.append((m.group(1), m.group(2)))
    return errors


def rust_fmt(filename):
    subprocess.run(["rustfmt", filename])


def write_errno(arch_name, lines):
    errno_file = os.path.join("platform", "netbsd-%s" % arch_name, "errno.rs")
    with open(errno_file, "w") as fh:
        fh.write("\n".join(lines))

    rust_fmt(errno_file)


def main():
    if len(sys.argv) < 2:
        print("Usage: %s arch-name" % sys.argv[0])
        sys.exit(1)
    arch_name = sys.argv[1]
    if arch_name == "-e":
        error_header_file = sys.argv[2]
        with open(error_header_file) as fh:
            content = fh.read()
            lines = parse_errno(content)
            print("\n".join(lines))
        return

    lines = read_errno(HEADER_FILE)
    write_errno(arch_name, lines)

if __name__ == "__main__":
    main()
