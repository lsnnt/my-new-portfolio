---
title: "How i got the data of train"
date: 2026-01-16
description: "First after sometime in train i noticed that there was a Wifi Network in my train with SSID “Vandebharatinfotainment” which has some movies but also tells th..."
---

First after sometime in train i noticed that there was a Wifi Network in my train with SSID “Vandebharatinfotainment” which has some movies but also tells the speed and other details so i figured out how the data is fetched

it was fetched by continuous fetching the data from

http://192.168.191.1/assets/pis/routedata.txt?date=0.6662353108255421

then i started to continuously fetch the data as fast as possible with the command given below and the data was stored in routedata

```bash
while true; do curl 'http://192.168.191.1/assets/pis/routedata.txt?date=0.6662353108255421' -H 'Accept: application/json, text/plain, */*' -H 'Accept-Language: en-GB,en-US;q=0.9,en;q=0.8' -H 'Connection: keep-alive' -H 'Referer: http://192.168.191.1/traininfo' -H 'User-Agent: Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Mobile Safari/537.36' --compressed --insecure; done >> routedata
```

then it was coming in the format of

```plaintext
DATE
TIME
SPEED
TIME REMAINING FOR NEXT STATION
DELAY
DISTANCE OF NEXT STATION
```

one example of that is

```plaintext
04-01-26
16:27
130 kmph
00:45
00:00
96 km
```

so it last when i come at station and my wifi disconnected i stopped that script and the output was 370Kb file then i converted it to csv with this script but before running the script i checked if anything was off as maybe some html came in the place of data and i find one html tag which i removed before running the script.

```python
import csv

df = []
with open("route.txt","r") as f:
alllines = f.readlines()
for i in range(0,len(alllines)//6):
stripped = [x.strip() for x in alllines[i*6:i*6+6]]
df.append(stripped)
fieldnames = ['Date','Time','Speed','Time Remaining for next station','Delay','Distance of next station']
# below lines were written with the help of ai
with open('routedata.csv', 'w', newline='') as csvfile:
writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
writer.writeheader()
# convert each row (list of values) to a dict matching the fieldnames
dict_rows = [dict(zip(fieldnames, row)) for row in df]
writer.writerows(dict_rows)

```

then it created a csv file while i uploaded to hugging face

[**nghjtvh/train-from-sbc-to-Katpadi · Datasets at Hugging Face**](https://huggingface.co/datasets/nghjtvh/train-from-sbc-to-Katpadi)

Which you can analyze on huggingface
