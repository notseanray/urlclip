#### url shortner

Very simple url shortner, stores urls for 1 week in memory.

##### usage

create new short

```
http://127.0.0.1:5456/new/https://github.com/notseanray/urlclip
```

it will generate a random not already occupied 4 letter code, this should give it ~26 ^ 4 combinations or nearly half a million while being super easy to remember

access short
```
http://127.0.0.1:5456/code
```
