// Start the Rust Engine
var spawn = require('child_process').spawn;
var engine = spawn('../target/debug/sandbox.exe')

engine.stdout.on('data', function (data) {
  data += ''
  if (data.substr(0,8) == 'COMMENT:') {
    console.log(data)
    Board.$data.status = (data.substr(8)).toLowerCase();
  } else {
    var arr = data.replace(/\(/g,'[').replace(/\)/g,']')
    console.log(arr)
    console.log(JSON.parse(arr))

  }
});