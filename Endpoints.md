# Backend API endpoints

## Queue support

-   toggle queue open/closed
-   add user to queue (is adding users manually desirable?)
-   remove user from queue
-   pop group from queu

## Chat commands

-   get command list - `GET /commands/`
-   create a command - `POST /commands/`
-   edit a command - `PATCH /commands/identifier`
-   disable a command - `PATCH /commands/identifier/enable`
-   delete a command - `DELETE /commands/identifier`

## Bot settings

-   get bot/other info? - `GET /bot/`
-   get user/oauth creds - `GET /user/`

## Chat log

3 options I can see for embedding chat

-   Use the twitch iframe, makes interacting with chat (pronouns or similar things) more difficult
-   Connect from the client side (means two connections to twitch, is that problematic?)
-   Use websockets or other streaming connection to feed chat info from backend to client

## OBS Websockets commands

For obs - commands will have to be passed to the client side where the websocket connection to OBS resides

-   Have a listener driven by the commands API that handles the execution in the client itself
-   Websockets again to pass event triggers from the backend bot
