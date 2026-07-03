<!--
 Copyright (C) 2026 Lordseriouspig
 
 This file is part of lordseriouspigs-website.
 
 lordseriouspigs-website is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 lordseriouspigs-website is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with lordseriouspigs-website.  If not, see <https://www.gnu.org/licenses/>.
-->

# Lordseriouspig's Website

![License](https://img.shields.io/github/license/Lordseriouspig/lordseriouspigs-website)
![GitHub last commit](https://img.shields.io/github/last-commit/Lordseriouspig/lordseriouspigs-website)

Hi hello, you've stumpled upon the website to my humble website. Whilst the frontend may not look like much yet (I do
have plans to neaten it up and make it a little nicer and add some more features soon!) the backend is a lot more. This
isn't the ordinary TUI-style website, for some reason beyond my own comprehension I decided to make the website an
*actual* TUI, with a websocket streaming it to your browser, and XTerm rendering that. I don't know why I put myself
through all this, but here we are. If you want to read a bit more on the development, take a look at the actual website,
which contains a page on how it was made.

## Usage

If you, for some reason, want to download and run this on your own system, you may do so through docker compose. Simply:

1. Clone the repo
   ```git clone https://github.com/Lordseriouspig/lordseriouspigs-website.git```
2. cd into the directory
   ```cd lordseriouspigs-website```
3. Run
   ```docker compose up```
4. Profit!

### Without docker

I have no clue why you'd want to do this if you're not me, but if something compels you to, you may run this without
docker pretty easily,

1. Clone the repo
   ```git clone https://github.com/Lordseriouspig/lordseriouspigs-website.git```
2. cd into the directory
   ```cd lordseriouspigs-website```
3. Run ``cargo run --manifest-path ./backend/Cargo.toml & npm --prefix ./frontend run dev`` (or just yk run these
   separately, then you wont have to kill the backend task manually)
4. Profit?

## Contributing

All contributions are welcome, please submit a pull request if you want to contribute. Please ensure you test anything
first though!
