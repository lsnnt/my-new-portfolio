---
title: "Building a Massive Q&A Dataset from Sarthaks.com"
date: 2026-06-04
description: "Long before starting my college journey, I wanted to scrape Sarthaks.com. I tried multiple times, but I kept failing."
---

Long before starting my college journey, I wanted to scrape Sarthaks.com. I tried multiple times, but I kept failing.

The biggest problem was rate limiting. Every time I tried traditional scraping methods, the website would eventually block requests.

Another issue was data quality. Even when I managed to scrape pages, important information like LaTeX equations was often missing, making the dataset much less useful.

So instead of scraping pages randomly, I changed my approach and started thinking about how web crawlers actually work.

## Discovering the Sitemap

I visited the sitemap index:

`https://www.sarthaks.com/sitemap.xml`

It contained entries like this:

```xml
<sitemap>
  <loc>https://www.sarthaks.com/sitemaps/questions-1.xml</loc>
  <lastmod>2026-06-03T15:02:27+00:00</lastmod>
</sitemap>
```

Each sitemap file then contained thousands of question URLs:

```xml
<url>
  <loc>https://www.sarthaks.com/1/what-is-digital-marketing</loc>
  <priority>0.10180855504822</priority>
</url>
```

At this point, I realized I didn’t need to “discover” pages manually anymore. The website itself already exposed all question URLs through sitemaps.

## Fetching All URLs

Using Go, I fetched all sitemap XML files and extracted every question URL.

Then I stored all collected links inside a large `urls` file, which ended up being around 172 MB.

After collecting URLs, I inspected the HTML structure of individual pages and found that:

*   `div.qa-q-view-content` contained the question
    
*   `div.qa-a-item-content` contained the answers
    

I then used the Go scraping framework Colly and extracted the content using the `OnHTML` callback.

[https://pastebin.com/embed_iframe/he4XEJNF](https://pastebin.com/embed_iframe/he4XEJNF) 

## Dataset Scale

The scale of the scrape ended up being much larger than I initially expected.

*   Total questions discovered: **1,850,743**
    
*   Rows successfully stored in the database: **1,850,674**
    
*   Final dataset size: **592 MB** (Snappy-compressed Parquet)
    
*   Total scraping duration: **~2 days**
    

The crawling process also consumed a huge amount of bandwidth:

*   53 GB from mobile data
    
*   37 GB on my server
    

Eventually, I ended up with two large databases:

*   one local (`qa.db`)
    
*   one hosted on the server (`qb.db`)
    

## Converting the Dataset

For uploading datasets to Hugging Face, formats like Parquet or CSV are preferred.

My first attempt used `parquet-go`, but the results were not very good. DuckDB could read the files, but Hugging Face dataset cards kept showing errors.

So I switched to Python for the conversion process.

Using Python, I successfully converted and uploaded one version of the dataset. However, the low-RAM server kept freezing during conversion.

To solve this, I used Google Cloud Shell, which provides a free cloud-based terminal environment for Google Cloud users.

That finally allowed me to complete the conversion and upload process successfully.

## Repository and Dataset

All scripts and files used in the project are available here:

*   GitHub: `https://github.com/lsnnt/qa-db`
    
*   Dataset DOI: `https://www.doi.org/10.57967/hf/9032`
    

The repository is still messy, and some scripts may fail immediately after cloning, but the overall workflow and scraping pipeline are there.

## What I Learned

This project taught me that large-scale scraping is less about aggressive crawling and more about understanding how websites are structured.

Once I started thinking like a search engine crawler instead of a normal scraper, the entire process became much simpler and more reliable.

Sitemaps turned out to be the key.
