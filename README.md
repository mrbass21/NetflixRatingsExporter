# Introduction

The purpose of this project is, primarily, for me to learn Rust. It also happens that I love Netflix and hate the new
ratings system, and wanted a way to backup my own ratings.

This also serves as my first github project that's public, so I expect the history to be pretty... messy.

# Status

This project is NOT done or functionally operational. Everything I have committed so far will build, but no real functionality is there, yet.

# How will it work?

It's very important that anyone who plans on using this program to export their own ratings understands how this works. I am technically using an "exploit" called Session Hijacking (https://en.wikipedia.org/wiki/Session_hijacking).

Essentially, you log into Netflix's site and it will set a cookie with (among other items) a Netflix Id and a Secure Netflix id. These are how the browser keeps track of your log in session. This application then submits your ids as a cookie with the request to get your ratings from the API endpoint.

To even further drill this down: Don't share your Netflix Id or Secure Netflix Id with anyone! If you write code, do _not_ statically place it in your code and place it somewhere public (like github). Do not use a binary (.exe file) from anyone unless you compile it yourself or you trust that person (even then, you shouldn't do it). This application is open source so that you can see exactly what it's doing and to keep it honest.

This application (as you can see) does not store those values or transmit them anywhere but the Netflix API endpoint. This is only done over TLS. If you are still worried, the better bet is to not use this application at all.

If you still want to get your ratings, but still feel uneasy about using this application, after the ratings are downloaded, go to Netflix and log out. Those cookie ids will be invalidated and logging in again will generate a new set of ids, rendering the old ones useless.

# Woah. Exploit? That sounds bad!
I'm asking every user to provide the program with your _own_ ids, so really I'm just accessing the rating history API end-point page the exact same way your browser would. But it does have the possibility to be miss-used if users don't understand how this works or the danger they _might_ be putting themselves in. That's why I'm writing this section. To be completely transparent on how this works.

I would prefer that Netflix offer a way to export your ratings directly. Until this is an option, this is at least one way to get a backup of your ratings.