title: GCC on Fedora 12(!)
slug: gcc-on-fedora-12
created: 2015-11-02
updated: 2015-11-02
tags: gcc, fedora, linux
summary: getting a compiler to run on a very old system

# Installing the GCC on Fedora 12(!)

**NB:** this post was imported from [medium](https://medium.com/@mplanchard/installing-the-gcc-compiler-on-fedora-12-1f3494823afa)

I recently found myself needing a compiler on an old Fedora 12 armv5tel system. Fedora never released a compiler for the armv5tel version 12, so I had to backport from 13. Putting aside for now the reasonable, proper correct solution of finding a way to upgrade the OS, I figured I’d document the upgrade process for anyone else who finds themselves in this situation.

All told, the process is probably easier than you would expect. It’s just a matter of package downloads, upgrades, and installations. All told, it required 16 new or updated RPMs and about 46 MB of disk space.

All of the required packages can be found at rpmfind.net and are listed below:

* binutils-2.20.51.0.2–23.fc13.armv5tel.rpm
* cloog-ppl-0.15.7–1.fc12.armv5tel.rpm
* cpp-4.4.5–2.fc13.armv5tel.rpm
* gcc-4.4.5–2.fc13.armv5tel.rpm
* glibc-2.12–4.armv5tel.rpm
* glibc-common-2.12–4.armv5tel.rpm
* glibc-devel-2.12–4.armv5tel.rpm
* glibc-headers-2.12–4.armv5tel.rpm
* gmp-4.3.1–6.fc13.armv5tel.rpm
* kernel-headers-2.6.33.3–85.fc13.armv5tel.rpm
* libffi-3.0.9–1.fc13.armv5tel.rpm
* libffi-devel-3.0.9–1.fc13.armv5tel.rpm
* libgcc-4.4.5–2.fc13.armv5tel.rpm
* libgomp-4.4.5–2.fc13.armv5tel.rpm
* mpfr-2.4.2–1.fc13.armv5tel.rpm
* ppl-0.10.2–11.fc13.armv5tel.rpm

Use wget or your preferred ftp client to download the requisite packages to a local directory, at which point installation is as easy as

```raw
yum — nogpgcheck localinstall *.rpm
```

And voila! You’ve got a compiler. I make no guarantees as to its fitness for any particular purpose, although in my experience it has worked fairly well for installing python packages with C extensions from pip.
