# big-luca-bot

<p align="center">
  <img src="/docs/images/big-luca-bot.png" width="256" height="256" alt="logo" />
</p>

<p align="center">~ Le perle e le news del papi su Telegram in ogni momento ~</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.6.0 (10/09/2022)</p>

<p align="center">
  <a href="https://opensource.org/licenses/Unlicense"
    ><img
      src="https://img.shields.io/badge/License-Unlicense-teal.svg"
      alt="License-Unlicense"
  /></a>
  <a href="https://github.com/veeso/big-luca-bot/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/big-luca-bot.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/big-luca-bot"
    ><img
      src="https://img.shields.io/crates/d/big-luca-bot.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/big-luca-bot"
    ><img
      src="https://img.shields.io/crates/v/big-luca-bot.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/big-luca-bot/actions"
    ><img
      src="https://github.com/veeso/big-luca-bot/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
</p>

---

- [big-luca-bot](#big-luca-bot)
  - [About big-luca-bot 📰](#about-big-luca-bot-)
  - [Command API 🐚](#command-api-)
  - [Get started 🏁](#get-started-)
    - [Users](#users)
    - [Developers](#developers)
      - [Configure parameters](#configure-parameters)
  - [Support the developer ☕](#support-the-developer-)
  - [Powered by 💪](#powered-by-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About big-luca-bot 📰

big-luca-bot is a Telegram bot to get the best aphorisms said by Big Luca and to get the latest news from "Il Papi" on your favourite telegram groups.

![Demo](/docs/images/demo.gif)

---

## Command API 🐚

- `/bigcorsi`

    This command will display the latest Big luca course

- `/bigkatanga`

    This command will make the bot to send automatic message

    > ❗ Automatic messages includes daily aphorisms and new videos

- `/bignews`

    This commands will return the latest videos from "Il Papi"

- `/bigperla`

    This command will make the bot sending a "perla del papi"

- `/bigpezzente`

    Unsubscribe from automated messages

- `/bigsito`

    This commands will return the OFFICIAL PAPI'S WEBSITE

- `/bigsocial`

    This commands will return the latest post from the papi's instagram account

- `/bigvideo`

    This command will display the latest video from "Il Papi"

<p align="center">
  <img src="/docs/images/big-telegram.webp" alt="telegram qr code" />
</p>

---

## Get started 🏁

### Users

Scan this QR code or go to this URL <https://t.me/bigluca_bot> to start a chat with Big Luca bot, then add it to any group or chat directly with him.

![telegram-qr](/docs/images/qr-code-md.webp)

### Developers

If you want to develop on this bot, you can follow these simple steps:

1. Clone this repository `git clone git@github.com:veeso/big-luca-bot.git`
2. Create your bot with the [Botfather](https://t.me/botfather)
3. Get your API key
4. Set your API key in your environment using the variable `TELOXIDE_TOKEN`
5. Set your database path in your environment using the variable `DATABASE_URL`
6. Set the path to `parameters.json` in your environment using the variable `PARAMETERS_PATH`
7. Set your redis url in your environment using the variable `REDIS_URL`
8. Set your instagram credentials in your environment using the variable `INSTAGRAM_USERNAME` and `INSTAGRAM_PASSWSORD` (optional)
9. Set `BIG_LUCA_PIDFILE` to the location where pidfile will be written (optional)
10. Touch the database file `touch $DATABASE_URL`
11. Run the big-luca bot

Or use `big-luca-bot.sh` script with `start/stop` commands

#### Configure parameters

You can add custom aphorisms and courses using the `parameters.json` file. Default file is located at `config/parameters.json` from the project root folder.

---

## Support the developer ☕

If you like big-luca-bot and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)
[![litecoin](https://img.shields.io/badge/Litecoin-345d9d?style=for-the-badge&logo=Litecoin&logoColor=white)](https://blockchair.com/litecoin/address/ltc1q89a7f859gt7nuekvnuuc25wapkq2f8ny78mp8l)

---

## Powered by 💪

- [feed-rs](https://github.com/feed-rs/feed-rs)
- [teloxide](https://github.com/teloxide/teloxide)
- [tokio](https://tokio.rs/)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve big-luca-bot, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View big-luca-bot's changelog [HERE](CHANGELOG.md)

---

## License 📃

big-luca-bot is licensed under the Unlicense license.

You can read the entire license [HERE](LICENSE)
