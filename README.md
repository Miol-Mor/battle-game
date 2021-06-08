# BATTLE GAME

## About
Battle-game is a multiplayer browser turn-based game written in [Rust](https://www.rust-lang.org/)

Its purpose is to master our development skills in general and rust skills in particular

## Install and run
To play the game, you have to download it and build and run it with [docker-compose](https://docs.docker.com/compose/install/):

```
git clone https://github.com/Miol-Mor/battle-game.git
cd battle-game
docker-compose build
docker-compose up
```

After this frontend and backend will be running on your machine and you just have to connect to the game in the browser using this link: `http://localhost:8080/dist/`

The game will start when two players connect. The simplest way is to connect from two tabs

## Game process

![battle-game gameplay](https://user-images.githubusercontent.com/8941791/121175164-6c58ab80-c863-11eb-852f-10cd47e5b369.gif)

There is a field with several units. Each unit belongs to one of two players. Players take turns in order. On each turn, player can move their unit and attack enemy unit. Player also can attack without movement or skip turn in any moment

You can find more gameplay information in [wiki](https://github.com/Miol-Mor/battle-game/wiki/Design-document-of-the-MVP-version-of-the-game)

## Work in progress
1. You can not configure game field or unit's initial location yet
2. There is no login - game starts when to players connect
3. Game should end when one of the players lost all their units, but it's not

## More
This project is distributed under a [MIT](https://github.com/Miol-Mor/battle-game/blob/master/LICENSE) license

You can find more detailed information [here](https://github.com/Miol-Mor/battle-game/wiki)

