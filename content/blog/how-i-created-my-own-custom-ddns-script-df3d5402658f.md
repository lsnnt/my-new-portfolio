---
title: "How i created my own custom ddns script"
date: 2026-01-16
description: "My ISP (Internet Service Provider) gives me /64 ipv6 address on which i configured a postfix mailserver on port 25 which can directly mail to my laptop."
---

My ISP (Internet Service Provider) gives me /64 ipv6 address on which i configured a postfix mailserver on port 25 which can directly mail to my laptop.

But the problem was that the ipv6 address changed every time i connected to my hotspot. So i created a program to handle it.

1. I want to get the ipv6 address which i can get with
    

`curl ip.se`

2\. I want to update the ip address on cloudflare as i got connected to my wifi network with specific ssid so i used cloudflare api here

More info

[**Cloudflare API | DNS › Records › Create DNS Record**](https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/create/)

Mainly in my case i want to get mail at nnt@jol.fustin.top which is pointed to mx1.fustin.top in my case.

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1768584999604/53f9616b-985e-4cc1-a523-e6bd6f78a654.png)

So finally i have to update the AAAA record of mx1 using cloudflare api i have done it

you can get the api key from cloudflare here

[https://dash.cloudflare.com/profile/api-tokens](https://dash.cloudflare.com/profile/api-tokens)

[**Create API token · Cloudflare Fundamentals docs**  
](https://developers.cloudflare.com/fundamentals/api/get-started/create-token/)

So now i have to automate it every time i connect to my wifi network

So in mac we have shortcuts which can automate the tasks

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1768585001434/78c48b77-a6e4-453c-ab39-750ba591d6c4.png)

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1768585003062/69ce1dff-bfad-4ee0-92bf-96a8866d4b55.png)

So everything is set now i have the binary file in updateipv6addr directory

> set -a = used to automatically export the variables to the system

> source .env = to set the .env file

> set +a = to reset the above thing

So i got the automated ipv6 update

The repo for this project:

[https://github.com/lsnnt/ipv6updater](https://github.com/lsnnt/ipv6updater)

Thank you,

At last i want to say you that the mail server is receive only because outbound connection to port 25 is blocked by my isp

You can mail me at [nnt@jol.fustin.top](mailto:nnt@jol.fustin.top)
