# Changelog

- [Changelog](#changelog)
  - [0.6.0](#060)
  - [0.5.0](#050)
  - [0.4.1](#041)
  - [0.4.0](#040)
  - [0.3.4](#034)
  - [0.3.3](#033)
  - [0.3.2](#032)
  - [0.3.1](#031)
  - [0.3.0](#030)
  - [0.2.2](#022)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.6.0

Released on 10/09/2022

- Use instagram scraper instead of rsshub

## 0.5.0

Released on 05/09/2022

- Restored picuki for social network, since instagram tries to block your account
- Add new sticker
- From now on each chat has a unique random sequence for aphorisms; aphorisms sequence is refreshed only when aphorisms change

## 0.4.1

Released on 01/09/2022

- New stickers
- More aphorisms during the day

## 0.4.0

Released on 31/08/2022

- From now on courses and aphorisms are read from `parameters.json`. RELOAD REQUIRES A BOT RESTART.
- Added `PARAMETERS_PATH` to environment parameters
- Repeating random aphorisms are now PREVENTED creating a shuffled sequence

## 0.3.4

Released on 29/08/2022

- Implemented `/start` command
- New `/big-release` command
- New stickers

## 0.3.3

Released on 29/08/2022

- New aphorisms
- Automatizer tasks: new behaviour for latest video/post
  - From now on, they won't show the LATEST video, but the latest UNSENT OLDEST video/post in the feed.

## 0.3.2

Released on 26/08/2022

- New stickers

## 0.3.1

Released on 25/08/2022

- Fetch instagram instead of picuki

## 0.3.0

Released on 24/08/2022

- Persist fetched times to redis
- Get latest instagram post via rsshub (use picuki)

## 0.2.2

Released on 24/08/2022

- Nota del papi

## 0.2.1

Released on 24/08/2022

- New scheduled aphorism in the afternoon
- Changed scheduled time
- Check whether chat is already subscribed

## 0.2.0

Released on 22/08/2022

- New aphorisms

## 0.1.1

Released on 21/08/2022

- New aphorisms
- Use webhook on heroku
- Changed scheduled jobs:
  - morning aphorism: 7AM UTC
  - evening aphorism: 6PM UTC
  - video poll: every hour at 30minutes
- Send a notification when the bot stops, to remind chats to resubscribe to katanga

## 0.1.0

Released on 20/08/2022

- First release
