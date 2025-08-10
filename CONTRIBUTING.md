# Contributing

Thank you for your interest in contributing to **rawsys\_linux**!
This project exists thanks to the passion and expertise of the community, and I truly appreciate every effort to make it better.
That said, because **this library operates at the raw inline assembly level**, it must be maintained with the utmost care and conservatism — stability comes first, always.


## Stability Over Features

This library uses inline assembly to directly invoke Linux syscalls.
While new features are exciting, **our highest priority is safety, correctness, and long-term stability**.
Pull requests that introduce large new features without a clear, stability-focused justification may not be accepted.


## Kernel Version Policy

Pull requests that simply add support for a new kernel version **may be declined**.
However, **long-term kernel version support requests (via an issue)** are welcome, and I will prioritize implementing them as soon as possible.


## Areas Where Community Help Shines

Your expertise can make a huge difference in the following areas:

* **SPARC syscall implementation** — Currently, this file is empty and needs your input.
* **Automated testing tools** — For example, a QEMU-based multienvironment testing setup would be invaluable.
* **Nightly-to-Stable migrations** — Some ISAs require nightly Rust for inline assembly; when they become stable, we need to adapt accordingly.
* **Use case sharing** — This is perhaps **the most valuable contribution of all**. While these may not be merged into the codebase, real-world usage examples inspire and guide the direction of this project.

## Pull Request Expectations

Please understand that, due to the low-level and highly sensitive nature of this library, **many PRs will not be merged**.
This is not a rejection of your effort — on the contrary, I deeply value every discussion, suggestion, and shared experience that comes from these contributions.
Please actively use the `Issues` and `Discusions` functions!


## Final Words

Every contribution — whether code, documentation, testing ideas, or usage stories — is a part of the community’s collective knowledge.
Even if your PR does not get merged, **your input is always appreciated and remembered**.
Thank you for helping rawsys\_linux remain stable, reliable, and truly useful for everyone.
