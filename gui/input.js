// Start the Rust Engine
var spawn = require('child_process').spawn;
var engine = spawn('../target/debug/sandbox.exe')

engine.stdout.on('data', function (data) {
  data += ''
  data.split('\n').filter(i => i).forEach(parse)
});


function parse(data) { 
  if (data.substr(0,8) == 'COMMENT:') {
    console.log(data)
    Board.$data.status = (data.substr(8)).toLowerCase();
  } else if (data.substr(0,6) == 'MOVES:') {
    var arr = data.substr(7).replace(/\(/g,'[').replace(/\)/g,']')
    console.log(arr)
    JSON.parse(arr).filter(tup => tup[1] != 0).forEach((i) => {
      Board.$data.squares[i[0]].value = i[1]
    })
    Board.activate(Board.$data.squares[JSON.parse(arr).filter(tup => tup[1] != 0).sort((a,b) => (b[1] - a[1]))[0][0]], true)
  }else {
    console.log(data)
  }
}