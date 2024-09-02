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

## start the server 

./i2gpt

## i2p tunnel

* embedded i2p and automatic tunnel generation
* set the http proxy port with `export I2GPT_I2P_PROXY=xxxx`
* default is port 4455
* .b32.i2p destination will printed in the logs
* see [j4i2prs](https://github.com/kn0sys/j4-i2p-rs) 
