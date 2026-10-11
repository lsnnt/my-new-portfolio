---
title: "Making an http server very close to computer"
date: 2026-06-23
description: "As i was developing on Macbook the linux library is not automatically present here so what i did instead i make an ubuntu docker container and then got insid..."
---

As i was developing on Macbook the linux library is not automatically present here so what i did instead i make an ubuntu docker container and then got inside that container and then installed the necessary things

### Provisioning a docker container and connecting to it

```shell
docker pull ubuntu
docker run --platform linux/amd64 --privileged -it --rm -v ~/Downloads/linux:/workspace/ ubuntu
```

Here I also made a linux directory on my downloads folder and linked it to my `/workspace` inside the ubuntu docker container so anything in workspace goes automatically to my `~/Downloads/linux` folder. As we will later boot on my old laptop which is x86\_64 architechture the --platform flag ensures the build is for that architechture.

## Setting up docker environment

Download all the necessary dependencies to build linux kernel

```shell
apt update
apt install -y bzip2 git vim make gcc libncurses-dev flex bison bc cpio libelf-dev libssl-dev xz-utils
```

While selecting timezone dont select indian first select first Asian then Kolkata, I dont really know why its like that but that like it is.

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/7909327a-ff82-4933-8a29-28a80a22a7ac.png)

## Cloning Linux repository

After that inside my docker i go to my workspace and cloned linux repo

```shell
cd linux 
make menuconfig
```

This already include the wifi drivers vga support and everything needed for development after development try to turn off some features which is unnecessarily included.

### building it (the cpu heavy part)

```shell
make -j$(nproc)
```

use this to build the kernel

Some of the images of cpu consumption you can skip these i am adding it for fun.

![cpu consumption](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/eb3abd8f-e5b3-4487-b4f8-b7d6d41a62b8.png)

The build was taking too much time so i went to build the absolute minimum wireless support + vga support

#### wireless stack

Networking support ---> Wireless ---> cfg80211 cfg80211 wireless extensions compatibility

#### Intel wifi

Device Drivers ---> Network device support ---> Wireless LAN ---> Intel devices ---> Intel Wireless WiFi Next Gen AGN

#### Firmware loader

Device Drivers ---> Generic Driver Options ---> Firmware loader

#### Networking

Networking support ---> Networking options ---> TCP/IP networking

#### Cryptographic apis for WPA/WPA2 PSK

AES CCM GCM SHA1 SHA256 HMAC CMAC

#### Enable this for debugging

Device Drivers -> Character devices -> Serial drivers -> 8250/16550 ... (set to \*, not M)

For getting the kernel panic message when no initramfs is there.

Later you should enable this also so that we can mirror the vga output to the serial console

```plaintext
 Symbol: SERIAL_8250 [=y]                                                                                               │
  │ Type  : tristate                                                                                                       │
  │ Defined at drivers/tty/serial/8250/Kconfig:7                                                                           │
  │   Prompt: 8250/16550 and compatible serial support                                                                     │
  │   Depends on: TTY [=y] && HAS_IOMEM [=y] && !S390                                                                      │
  │   Location:                                                                                                            │
  │     -> Device Drivers                                                                                                  │
  │       -> Character devices                                                                                             │
  │         -> Enable TTY (TTY [=y])                                                                                       │
  │           -> Serial drivers                                                                                            │
  │ (1)         -> 8250/16550 and compatible serial support (SERIAL_8250 [=y])                                             │
  │ Selects: SERIAL_CORE [=n] && SERIAL_MCTRL_GPIO [=n]
```

#### Enable initramfs for initial ram filesystem

General setup --> Initial RAM filesystem and RAM disk (initramfs/initrd) support

#### Enable Executable file formats

```plaintext
[*] Kernel support for ELF binaries                                                          │ │
  │ │                       [*] Kernel support for scripts starting with #!                                              │ │
  │ │                       [*] Kernel support for MISC binaries                                                         │ │
  │ │                       [ ] Enable core dump support
```

Then start building using

```shell
make -j$(nproc)
```

After that you will get a kernel image at `arch/x86/boot/bzImage`

you can launch it via `qemu-system-x86_64` using

```shell
qemu-system-x86_64 \
    -kernel arch/x86/boot/bzImage \
    -append "console=ttyS0" \
    -nographic
```

This will start a linux kernel which will panic at last because there is no init file there

My log is embedded here

[https://gist.github.com/lsnnt/3b46d1b6fb79ae4444a1179d3d2bdc05](https://gist.github.com/lsnnt/3b46d1b6fb79ae4444a1179d3d2bdc05) 

### Building initramfs

After we have built the kernel we should build the initramfs which is the first file system which is like You can give Linux a specific compressed archive containing some files, and it will load it into RAM, and use it as its initial root file system

For the init filesystem we are using busybox and also we need the `init` file that will be entrypoint of out linux system

#### Building busybox

clone busybox repository

```shell
git clone --depth 1 https://github.com/vda-linux/busybox_mirror
cd busybox_mirror/
make menuconfig
```

in menuconfig disable tc

```plaintext
sed -i 's/CONFIG_TC=y/# CONFIG_TC is not set/' .config
```

make the binary

```plaintext
make -j8
```

After compilation we do need the initramfs so make a bootf directory in the workspace and will start working in it

```plaintext
mkdir -p /workspace/bootf/initramfs
```

We need that initramfs for later on developing the files that will be there at the boot time

After building busybox we need to install it inside the initramfs directory

```plaintext
make CONFIG_PREFIX=/workspace/bootf/initramfs install
```

This will make the initial ram filesystem for the linux kernel

also we will make the /init file which will be loading at the startup so make an init file and write this inside initramfs folder

```plaintext
#!/bin/sh

mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev

echo ""
echo "================================="
echo "      Ghost Linux Booted"
echo "================================="
echo ""

exec /bin/sh
```

This file is very basic it will mount the required folders for the linux and starts a filesystem. Also we need to build it as an archive we will use cpio here because its an older archive format and linux kernel supports it inside initramfs do this

```plaintext
find . | cpio -H newc -o | gzip -9 > ../initramfs.cpio.gz
```

then we already have the `bzImage` and `initramfs.cpio.gz` we can now make the .iso file or we can just boot it with qemu

booting it

```plaintext
qemu-system-x86_64 \                                         
  -kernel bzImage \
  -initrd initramfs.cpio.gz \
  -append "console=ttyS0" \
  -nographic
```

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/e2fa1406-03c3-4981-856d-defac01e010e.png)

This starts a busybox shell on top of linux system which is mainly for testing and will be removed when i find out how to access internet here

For the iso building we will be using grub bootloader grub expects the file format as follows

```plaintext
root@183ae334d9f9:/workspace/bootf# tree iso
iso
`-- boot
    |-- bzImage
    |-- grub
    |   `-- grub.cfg
    `-- initramfs.cpio.gz

3 directories, 3 files
```

just copy the bzImage and initramfs.cpio.gz into the `iso/boot` folder and inside `iso/boot/grub/grub.cfg` make this file for giving grub what to boot

```plaintext
set timeout=3
set default=0

insmod all_video

menuentry "Ghost Linux Test" {
    search --file --set=root /boot/bzImage
    linux /boot/bzImage console=tty0
    initrd /boot/initramfs.cpio.gz
}
```

here insmod all\_video does the kernel module insertion for video drivers

making the iso file is easy now

just this command

```plaintext
 grub-mkrescue -o test.iso iso
```

Alas! I dont have the USB drive.

I got it now

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/454002c2-f2f7-40c7-8336-c328f86b12d6.png)

So i managed to get the usb drive and after formatting it via balena etcher with my custom made iso file i will try to boot it on my old (10 years old :-) ) laptop.

moment of truth

### Booting on my old laptop (moment of truth)

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/b16099f8-b095-4e88-8995-8833f8c10a42.jpg)

### Add internet to it

For internet i have to do many changes in linux kernel config and also add iwlwifi driver module firmware say whatever

changed the init script all the workspace will be shared so lookup that for all the files

each and everything is changed after adding internet to it better use my shared workspace and take help

### Just give me ISO file i will run it myself

here is the public url of the iso file : [here](https://drive.google.com/file/d/1mtShmLvxRIbAaaPvABE7Z-dD-fXSoCKf/view?usp=sharing)

This is not a distributable version and only contains kernels and firmwares according to my laptop wifi card and other specification

### Now i want your workspace each and every file that you used to generate the iso file

download it here : [zip](https://drive.google.com/file/d/1YWTSYYcPPVDIYSoh_7tqaqJyJbB69oOl/view?usp=sharing) (1.57GB compressed) (3 GB uncompressed)

explaining the project directory

```plaintext
.
├── bootf (contains initramfs and iso)
│   ├── initramfs
│   └── iso 
├── busybox_mirror
│   ├── applets
│   ├── applets_sh
│   ├── arch
│   ├── archival
│   ├── configs
│   ├── console-tools
│   ├── coreutils
│   ├── debianutils
│   ├── docs
│   ├── e2fsprogs
│   ├── editors
│   ├── examples
│   ├── findutils
│   ├── include
│   ├── init
│   ├── klibc-utils
│   ├── libbb
│   ├── libpwdgrp
│   ├── loginutils
│   ├── mailutils
│   ├── miscutils
│   ├── modutils
│   ├── networking
│   ├── printutils
│   ├── procps
│   ├── qemu_multiarch_testing
│   ├── runit
│   ├── scripts
│   ├── selinux
│   ├── shell
│   ├── sysklogd
│   ├── testsuite
│   └── util-linux
├── firmwares
│   └── linux-firmware
├── httpserver
│   ├── src
│   └── target
├── iw
│   └── iw-6.7
├── libnl-3.9.0
│   ├── build-aux
│   ├── etc
│   ├── include
│   ├── lib
│   ├── m4
│   ├── man
│   ├── python
│   ├── src
│   └── tests
└── linux
    ├── arch
    ├── block
    ├── certs
    ├── crypto
    ├── Documentation
    ├── drivers
    ├── fs
    ├── include
    ├── init
    ├── io_uring
    ├── ipc
    ├── kernel
    ├── lib
    ├── LICENSES
    ├── mm
    ├── net
    ├── rust
    ├── samples
    ├── scripts
    ├── security
    ├── sound
    ├── tools
    ├── usr
    └── virt
```

Expanding iso

```plaintext
iso
└── boot
    ├── bzImage
    ├── grub
    │   └── grub.cfg
    └── initramfs.cpio.gz
```

Busybox\_mirror is used to compile it statically and include it inside the initramfs

firmware directory contains linux firmwares such as firmware and driver code to connect via intel chip wifi

initramfs contains the directories as follows

```plaintext
initramfs
├── bin
├── dev
├── etc
├── lib
│   ├── firmware
│   └── modules
│       └── 7.1.0+
├── proc
├── run
├── sbin
├── sys
├── usr
│   ├── bin
│   ├── sbin
│   └── share
│       └── udhcpc
└── www
```

The index.html in www will be loaded

#### Youtube video demonstration (moment of truth)

[https://youtu.be/7sAwB5_MRzM](https://youtu.be/7sAwB5_MRzM) 

At last keep your curiosity high and bye :-)

Signing off,

Nityanand Thakur

Email : `tnityanand523@gmail.com`
