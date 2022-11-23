# Lapce Plugin for Vue (based volar)

### Preview

![vue Plugin for lapce](https://raw.githubusercontent.com/xiaoxin-sky/lapce-vue/master/image/lps.png)

### Usage


****

**Config language server path**
> if download language Binary slow, You can set language server.
> you need install @volar/vue-language-server first, and Paste the npm global xxx@volar/vue-language-server/bin/vue-language-server.js

install global vue language server
```bash
npm install @volar/vue-language-server -g
```

get global path
```bash 
npm root -g 
# such as echo:  /Users/skymac/.nvm/versions/node/v14.19.2/lib/node_modules
```

such as: language server path
`/Users/skymac/.nvm/versions/node/v14.19.2/lib/node_modules@volar/vue-language-server/bin/vue-language-server.js`


**Manual installation** 

1. Open Lapce， Press the F1 key on your keyboard, then input `:open plugins Directory` in the command palette.
2. Download `lapce-vue.tar.gz` [click here to get release](https://github.com/xiaoxin-sky/lapce-vue/releases)
3. Extract lapce-vua.tar.gz into your lapce plugins directory.
4. Reload Lapce
