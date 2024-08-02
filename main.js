const axios = require('axios')
const express = require('express')
const bodyParser = require('body-parser')
const app = express()
const port = 3141

const serveIndex = require('serve-index');

app.use(express.static("public"))
app.use(bodyParser.urlencoded({ extended: false }));

app.set('x-powered-by', false);
app.set('etag', false)
// home
app.get('/', (req, res) => {
  res.send(`I2GPT - gpt access with no js or trackers`)
});
// ollama interface
app.get('/response', async (req, res) => {
  let q = req.query;
  // send query to model api via axios
  console.log('sending request to model');
  let output = await axios({
    method: 'post',
    url: 'http://127.0.0.1:11434/api/generate',
    data: {
      model: 'zephyr-local',
      prompt: `${q.search}`,
      stream: false,
    }
  });
  console.log(`model returned ${output.data.response}`)
  // TODO: inject model response into html to make it pretty
  res.send(`${output.data.response}`)
})
// 404 catcher
app.get('*', (req, res) => {
  res.status(404).send('The requested resource could not be found.');
});
app.listen(port, () => {
  console.log(`i2gpt server running on port ${port}`)
})
