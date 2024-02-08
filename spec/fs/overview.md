# Oxide Filesystem Standard

Oxide's filesystem, similar to many other components of Oxide, does not adhere to common standards (see [Compatibility](/en/latest/compat/)). Rather, it tries to create a filesystem that best fits its own design philosophy. Oxide's filesystem tries to make it easy to develop software within Oxide's ecosystem while also making it simple to be a casual user without ever having to peek behind the proverbial curtain.

**NOTICE**: The filesystem spec is currently in early development. Expect incompleteness and frequent changes.

## Filesystem Root
 - `/bin` contains system-level binaries. It should not be accessed by a normal user.
 - `/lib` and `/lib64` contain system-level libraries. These should never be modified and should not be accessed by a normal user. They should contain the bare minimum libraries required for Rust to function.
 - `/system` is the unit directory for the `system` unit. See [System Unit](/en/latest/quartz/unit/#system_unit) for more details.
 - `/user` is the default directory in which user units are stored on a desktop system. If the system being run is a server without the need for permission-based logins, this directory can safely be deleted.
