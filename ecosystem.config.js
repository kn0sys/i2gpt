module.exports = {
    apps : [{
      name: 'i2gpt',
        script: "main.js",
        instances: 1,
        exec_mode: 'fork',
        cron_restart: "0,30 * * * *",
        watch: false,
        autorestart: false
    }]
  }
  