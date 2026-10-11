---
title: "How do i reverse engineered Chotadhobi app"
date: 2026-05-01
description: "TLDR :- The app i have used is PCAPDroid which was the easiest method to view the backend apis of the app."
---

TLDR :- The app i have used is PCAPDroid which was the easiest method to view the backend apis of the app.

## Method 1

Setup a rooted AVD in Android studio via rootavd and install Pcapdroid on it after that install this [Always trust user certificate](https://github.com/NVISOsecurity/AlwaysTrustUserCerts) module in magisk manager

open pcapdroid and setup it for decrypting TLS traffic you can refer [this](https://emanuele-f.github.io/PCAPdroid/tls_decryption).

After you saved the file after doing the things required you will get a sslkeylog file and a pcapng file

in my case

pcap file - download [here](https://www.dropbox.com/scl/fi/v52536qwdnflhez8kx4wo/PCAPdroid_14_Apr_18_13_33.pcap?rlkey=p1a5ww56guznvpm4gaauuod2t&st=17e4sjj9&dl=0)

ssl.log file - download [here](https://www.dropbox.com/scl/fi/wi8hnh40xqu415y6uvcw8/ssl.log?rlkey=2oveyv8ujktx4exo0p5ja08ku&st=sz4bczo0&dl=0)

Then open the pcap file in wireshark and go to preferences then protocols then TLS like this

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/39321b53-3c5b-40ed-bb61-9b277226ebf0.png)

and set the key log file to the downloaded file then you will be able to see the decrypted tls file you can follow any stream to see what it did

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/2966a65c-27a3-4a74-b6c1-fdaee3f66373.png)

## Method 2

You can use frida + ssl-pinning + android rooted AVD

This thing i have tried but the first method was easier

Further read the blog posts below for more information:-

[https://www.trickster.dev/post/setting-up-rooted-android-emulator-with-frida-and-mitmproxy/](https://www.trickster.dev/post/setting-up-rooted-android-emulator-with-frida-and-mitmproxy/)

[https://infosecwriteups.com/hail-frida-the-universal-ssl-pinning-bypass-for-android-e9e1d733d29](https://infosecwriteups.com/hail-frida-the-universal-ssl-pinning-bypass-for-android-e9e1d733d29)

[https://www.redfoxsec.com/blog/ssl-pinning-bypass-for-android-using-frida](https://www.redfoxsec.com/blog/ssl-pinning-bypass-for-android-using-frida)

Signing off,

Nityanand Thakur

Cover photo credits:

[Image by freepik](https://www.magnific.com/free-vector/technology-background_4227101.htm)
