// Start the Rust Engine
var spawn = require('child_process').spawn;
var engine = spawn('../target/release/sandbox.exe')

engine.stderr.on('data', function (data) {
  console.log(data + '')
})
engine.stdout.on('data', function (data) {
  data += ''
  data.split('\n').filter(i => i).forEach(parse)
});


function parse(data) { 
  var things = data.split(':');
  if (things.length < 2) {
    console.log(data)
    return
  }

  switch (things[0]) {
    case 'COMMENT':
      console.log(data)
      Board.$data.status = (data.substr(8))
    break
    case 'EVENT':
      Board.$data.ready = true
    break
    case 'MOVES':
      var arr = data.substr(7).replace(/\(/g,'[').replace(/\)/g,']')
      console.log(arr)/*
      JSON.parse(arr).filter(tup => tup[1] != 0).forEach((i) => {
        Board.$data.squares[i[0]].value = i[1]
      })*/
      setTimeout(() => {
        Board.activate(Board.$data.squares[JSON.parse(arr)[0]], true)
      },10)
    break
  }



}