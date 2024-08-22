# i2gpt
gpt with no js or trackers

# getting started

* install sever-side js with [node.js](https://github.com/nvm-sh/nvm)
* run `npm i`


# setting up the model with ollama

* install [ollama](https://ollama.com/download)
* `cd models`
* download the [zephyr 7b-Q5](https://huggingface.co/TheBloke/zephyr-7B-beta-GGUF/blob/main/zephyr-7b-beta.Q5_K_M.gguf) model from huggingface
* `ollama create zephyr-local -f Modelfile`
* `ollama list`
* `ollama run zephyr-local`
* `/bye` to exit cli
* `ollama serve`

## api test

```bash
curl -v http://localhost:11434/api/generate -d '{ "model": "zephyr-local", "prompt": "What is water made of?", "stream": false }'
```

## serving with pm2

* https://www.npmjs.com/package/pm2
* `npm install pm2 -g`
* `pm2 start ecosystem.config.js`

## i2p tunnel

see [i2p](https://geti2p.net/en/) official documentation for setting up tunnels
